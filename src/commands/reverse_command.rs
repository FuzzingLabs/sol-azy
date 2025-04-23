use crate::reverse::{analyze_program, ReverseOutputMode};
use anyhow::Result;

/// Dispatches the reverse engineering workflow based on a user-specified mode.
///
/// Converts a string-based mode (`"disass"`, `"cfg"`, `"both"`, or `"rusteq"`)
/// into a `ReverseOutputMode` enum and calls `analyze_program` accordingly.
///
/// # Arguments
///
/// * `mode` - A string indicating which analysis mode to use.
/// * `out_dir` - The path to the directory where output files will be written.
/// * `bytecodes_file` - Path to the compiled eBPF bytecode (.so file).
/// * `labeling` - Whether to enable symbol and section labeling in the analysis.
///
/// # Returns
///
/// A `Result<()>` that is `Ok` if the analysis succeeded, or an error if the mode was unknown
/// or analysis failed.
///
/// # Errors
///
/// Returns an error if the provided `mode` string does not match any known `ReverseOutputMode`.
pub fn run(mode: String, out_dir: String, bytecodes_file: String, labeling: bool) -> Result<()> {
    let output_mode = match mode.as_str() {
        "disass" => ReverseOutputMode::Disassembly(out_dir),
        "cfg" => ReverseOutputMode::ControlFlowGraph(out_dir),
        "both" => ReverseOutputMode::DisassemblyAndCFG(out_dir),
        "rusteq" => ReverseOutputMode::DisassAndRustEquivalent(out_dir),
        other => {
            return Err(anyhow::anyhow!("Unknown reverse mode: {}", other));
        }
    };

    analyze_program(output_mode, bytecodes_file, labeling)
}
