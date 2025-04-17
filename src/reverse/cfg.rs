use solana_sbpf::{ebpf::Insn, static_analysis::Analysis};
use std::collections::{BTreeMap, HashSet};
use std::u8;

use crate::reverse::utils::format_bytes;
use crate::reverse::OutputFile;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

const MAX_BYTES_TO_READ_FOR_IMMUTABLE_STRING_REPR: u8 = 50;

fn get_string_repr(
    program: &'static [u8],
    insn: &Insn,
    next_insn_wrapped: Option<&Insn>,
) -> String {
    match insn.opc {
        solana_sbpf::ebpf::LD_DW_IMM => {
            let start = insn.imm as usize;

            // Make sure we don't read out of bounds
            if start >= program.len() {
                return "".to_string();
            }

            // Default number of bytes to read from the program when no length found
            let mut length = MAX_BYTES_TO_READ_FOR_IMMUTABLE_STRING_REPR as usize;

            if let Some(next_insn) = next_insn_wrapped {
                if next_insn.opc == solana_sbpf::ebpf::MOV64_IMM
                    || next_insn.opc == solana_sbpf::ebpf::MOV32_IMM
                {
                    let maybe_len = next_insn.imm as usize;
                    if maybe_len > 0 {
                        length = maybe_len;
                    }
                }
            }

            let end = usize::min(start + length, program.len());

            let slice = &program[start..end];
            format_bytes(slice)
        }

        _ => "".to_string(),
    }
}

// modified version of "visualize_graphically" from sbpf-solana for full static analysis
pub fn export_cfg_to_dot<P: AsRef<Path>>(
    program: &'static [u8],
    analysis: &mut Analysis,
    path: P,
) -> std::io::Result<()> {
    let mut cfg_path = PathBuf::from(path.as_ref());
    cfg_path.push(OutputFile::Cfg.default_filename());
    let mut output = File::create(cfg_path)?;

    fn html_escape(string: &str) -> String {
        string
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('\"', "&quot;")
    }
    fn emit_cfg_node<W: std::io::Write>(
        program: &'static [u8],
        output: &mut W,
        analysis: &Analysis,
        function_range: std::ops::Range<usize>,
        alias_nodes: &mut HashSet<usize>,
        cfg_node_start: usize,
    ) -> std::io::Result<()> {
        let cfg_node = &analysis.cfg_nodes[&cfg_node_start];
        let insns = analysis.instructions[cfg_node.instructions.clone()].to_vec();
        writeln!(output, "    lbb_{} [label=<<table border=\"0\" cellborder=\"0\" cellpadding=\"3\">{}</table>>];",
            cfg_node_start,
            analysis.instructions[cfg_node.instructions.clone()].iter()
            .enumerate().map(|(pc, insn)| {
                let mut desc = analysis.disassemble_instruction(
                    insn, pc
                );
                // next instruction lookup to gather information (like for string and their length when it uses MOV64_IMM)
                let next_insn = insns.get(pc + 1);
                // add immutable string repr if it does exists on bytecode 
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
            })
            .collect::<String>()
        )?;
        for child in &cfg_node.dominated_children {
            emit_cfg_node(
                program,
                output,
                analysis,
                function_range.clone(),
                alias_nodes,
                *child,
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
        15 + MAX_BYTES_TO_READ_FOR_IMMUTABLE_STRING_REPR as usize;
    let function_iter = &mut analysis.functions.keys().peekable();
    while let Some(function_start) = function_iter.next() {
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
            *function_start,
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
        if cfg_node_start != cfg_node.dominator_parent {
            writeln!(
                output,
                "  lbb_{} -> lbb_{} [style=dotted; arrowhead=none];",
                cfg_node_start, cfg_node.dominator_parent,
            )?;
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
