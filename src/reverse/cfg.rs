use solana_sbpf::static_analysis::Analysis;
use std::collections::{BTreeMap, HashSet};
use std::u8;

use crate::reverse::utils::{MAX_BYTES_USED_TO_READ_FOR_IMMEDIATE_STRING_REPR, get_string_repr};
use crate::reverse::OutputFile;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Exports the control flow graph (CFG) of a program to a Graphviz-compatible DOT file.
/// Each function is rendered as a subgraph showing basic blocks (`lbb_XXX`) and instruction-level content.
///
/// This function is a modified version of `visualize_graphically` from the `sbpf-solana` project,
/// and supports advanced filtering for cleaner output in complex programs.
///
/// # Arguments
///
/// * `program` - Raw bytecode of the program
/// * `analysis` - A mutable reference to the `Analysis` structure containing disassembly and CFG data.
/// * `path` - Path to the output directory where the `.dot` file will be saved.
/// * `reduced` - If `true`, only includes functions defined **after** the program entrypoint in the CFG output.
///   This is useful to exclude prelude or system/library functions and focus on the main logic.
/// * `only_entrypoint` - If `true`, only includes the cluster corresponding to the entrypoint function (e.g., `cluster_XX`)
///   in the DOT output. This enables minimal CFGs that users can extend manually using the `dotting` module.
///
/// # Returns
///
/// * `Ok(())` if the DOT file was generated successfully.
/// * `Err(std::io::Error)` if there was a problem writing the file.
pub fn export_cfg_to_dot<P: AsRef<Path>>(
    program: &[u8],
    analysis: &mut Analysis,
    path: P,
    reduced: bool,
    only_entrypoint: bool
) -> std::io::Result<()> {
    let mut cfg_path = PathBuf::from(path.as_ref());
    cfg_path.push(OutputFile::Cfg.default_filename());
    let mut output = File::create(cfg_path)?;

    /// Escapes a string for safe inclusion in HTML (used in DOT labels).
    fn html_escape(string: &str) -> String {
        string
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('\"', "&quot;")
    }

    /// Emits a single CFG node and recursively its children to the DOT output.
    ///
    /// # Arguments
    ///
    /// * `program` - The bytecode
    /// * `output` - Output writer
    /// * `analysis` - Reference to the analysis data
    /// * `function_range` - Bytecode range of the current function
    /// * `alias_nodes` - Set of alias node indices
    /// * `cfg_node_start` - Entry point of the current node
    fn emit_cfg_node<W: std::io::Write>(
        program: &[u8],
        output: &mut W,
        analysis: &Analysis,
        function_range: std::ops::Range<usize>,
        alias_nodes: &mut HashSet<usize>,
        visited_nodes: &mut HashSet<usize>,
        cfg_node_start: usize,
        reduced:bool,
    ) -> std::io::Result<()> {
        let cfg_node = &analysis.cfg_nodes[&cfg_node_start];
        let insns = analysis.instructions[cfg_node.instructions.clone()].to_vec();

        if reduced { // this will save some memory for not-reduced CFG
            visited_nodes.insert(cfg_node_start);
        }

        writeln!(output, "    lbb_{} [label=<<table border=\"0\" cellborder=\"0\" cellpadding=\"3\">{}</table>>];",
            cfg_node_start,
            analysis.instructions[cfg_node.instructions.clone()].iter()
            .enumerate().map(|(pc, insn)| {
                let mut desc = analysis.disassemble_instruction(insn, pc);
                // next instruction lookup to gather information (like for string and their length when it uses MOV64_IMM)
                let next_insn = insns.get(pc + 1);
                // add immediate string repr if it does exists on bytecode 
                let str_repr = get_string_repr(program, insn, next_insn);
                if str_repr != "" {
                    desc.push_str(" --> ");
                    desc.push_str(&str_repr);
                }
                if let Some(split_index) = desc.find(' ') {
                    let mut rest = desc[split_index+1..].to_string();
                    if rest.len() > MAX_CELL_CONTENT_LENGTH + 1 {
                        rest.truncate(MAX_CELL_CONTENT_LENGTH);
                        rest = format!("{rest}…");
                    }
                    format!("<tr><td align=\"left\">{}</td><td align=\"left\">{}</td></tr>", html_escape(&desc[..split_index]), html_escape(&rest))
                } else {
                    format!("<tr><td align=\"left\">{}</td></tr>", html_escape(&desc))
                }
            }).collect::<String>()
        )?;

        for child in &cfg_node.dominated_children {
            emit_cfg_node(
                program,
                output,
                analysis,
                function_range.clone(),
                alias_nodes,
                visited_nodes,
                *child,
                reduced
            )?;
        }

        Ok(())
    }

    writeln!(
        output,
        "digraph {{
graph [
rankdir=LR;
concentrate=True;
style=filled;
color=lightgrey;
];
node [
shape=rect;
style=filled;
fillcolor=white;
fontname=\"Courier New\";
];
edge [
fontname=\"Courier New\";
];"
    )?;

    const MAX_CELL_CONTENT_LENGTH: usize =
        15 + MAX_BYTES_USED_TO_READ_FOR_IMMEDIATE_STRING_REPR as usize;


    let mut is_entrypoint_visited = false;
    let function_iter = &mut analysis.functions.keys().peekable();
    let mut visited_nodes = HashSet::new();

    while let Some(function_start) = function_iter.next() {
        let label = &analysis.cfg_nodes[function_start].label;
        if ( reduced || only_entrypoint ) && !is_entrypoint_visited && label != "entrypoint" {
            continue;
        }
        if is_entrypoint_visited && only_entrypoint {
            break;
        }
        if label == "entrypoint" {
            is_entrypoint_visited = true;
        }
        let function_end = if let Some(next_function) = function_iter.peek() {
            **next_function
        } else {
            &analysis.instructions.last().unwrap().ptr + 1
        };

        let mut alias_nodes = HashSet::new();

        writeln!(output, "  subgraph cluster_{} {{", *function_start)?;
        writeln!(
            output,
            "    label={:?};",
            html_escape(&analysis.cfg_nodes[function_start].label)
        )?;
        writeln!(output, "    tooltip=lbb_{};", *function_start)?;

        emit_cfg_node(
            program,
            &mut output,
            &analysis,
            *function_start..function_end,
            &mut alias_nodes,
            &mut visited_nodes,
            *function_start,
            reduced || only_entrypoint
        )?;

        for alias_node in alias_nodes.iter() {
            writeln!(
                output,
                "    alias_{}_lbb_{} [",
                *function_start, *alias_node
            )?;
            writeln!(output, "        label=lbb_{:?};", *alias_node)?;
            writeln!(output, "        tooltip=lbb_{:?};", *alias_node)?;
            writeln!(output, "        URL=\"#lbb_{:?}\";", *alias_node)?;
            writeln!(output, "    ];")?;
        }

        writeln!(output, "  }}")?;
    }

    for (_, cfg_node_start, cfg_node) in analysis.iter_cfg_by_function() {
        if reduced || only_entrypoint {
            if !visited_nodes.contains(&cfg_node_start) {
                continue;
            }
            if cfg_node_start != cfg_node.dominator_parent {
                writeln!(
                    output,
                    "  lbb_{} -> lbb_{} [style=dotted; arrowhead=none];",
                    cfg_node_start, cfg_node.dominator_parent,
                )?;
            }
        }

        let edges: BTreeMap<usize, usize> = cfg_node
            .destinations
            .iter()
            .map(|destination| (*destination, 0))
            .collect();

        let counter_sum: usize = edges.values().sum();

        if counter_sum == 0 && !edges.is_empty() {
            writeln!(
                output,
                "  lbb_{} -> {{{}}};",
                cfg_node_start,
                edges
                    .keys()
                    .map(|destination| format!("lbb_{}", *destination))
                    .collect::<Vec<String>>()
                    .join(" ")
            )?;
        }
    }

    writeln!(output, "}}")?;
    Ok(())
}
