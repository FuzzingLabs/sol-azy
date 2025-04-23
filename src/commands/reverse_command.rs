use crate::reverse::{analyze_program, ReverseOutputMode};
use anyhow::Result;

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
