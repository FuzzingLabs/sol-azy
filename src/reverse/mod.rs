pub mod cfg;
pub mod disass;
pub mod immutable_tracker;
pub mod utils;

use cfg::*;

use disass::disassemble_wrapper;
use immutable_tracker::ImmutableTracker;
use solana_sbpf::{
    elf::Executable,
    program::{BuiltinProgram, FunctionRegistry, SBPFVersion},
    static_analysis::Analysis, vm::Config,
};
use std::{fs::File, io::Read as _, path::Path, sync::Arc, u8};
use test_utils::TestContextObject;

use std::io::Result;

pub enum OutputFile {
    Disassembly,
    ImmutableDataTable,
    Cfg,
}

impl OutputFile {
    pub fn default_filename(&self) -> &'static str {
        match self {
            OutputFile::Disassembly => "disassembly.out",
            OutputFile::ImmutableDataTable => "immutable_data_table.out",
            OutputFile::Cfg => "cfg.dot",
        }
    }
}

/// Defines the output mode for the analysis function.
pub enum ReverseOutputMode {
    /// Only disassemble the program to stdout.
    Disassembly(String), // path to save every files (.table, .disass) output
    /// Export the control flow graph (CFG) to a .dot file.
    ControlFlowGraph(String), // path to save .dot output
    /// Disassemble and output CFG.
    DisassemblyAndCFG(String), // path to save every files (.table, .disass, .dot) output
    /// Future extension: Attempt to produce a Rust-like representation.
    DisassAndRustEquivalent(String), // path to save every files (.table, .disass) output
}

impl ReverseOutputMode {
    pub fn path(&self) -> &str {
        match self {
            ReverseOutputMode::Disassembly(p)
            | ReverseOutputMode::ControlFlowGraph(p)
            | ReverseOutputMode::DisassemblyAndCFG(p)
            | ReverseOutputMode::DisassAndRustEquivalent(p) => p,
        }
    }
}


/// Analyzes a given eBPF program according to the specified output mode.
pub fn analyze_program(mode: ReverseOutputMode, target_bytecode: String, is_stripped: bool) -> Result<()> {
    // Mocking a loader & create an executable
    let loader = Arc::new(BuiltinProgram::new_loader(Config {
        enable_instruction_tracing: true,
        enable_symbol_and_section_labels: is_stripped,
        ..Config::default()
    }));

    let mut file = File::open(Path::new(&target_bytecode)).unwrap();
    let mut elf = Vec::new();
    file.read_to_end(&mut elf).unwrap();

    let program = elf.clone();

    let executable = Executable::<TestContextObject>::from_elf(&elf, loader).map_err(|err| format!("Executable constructor failed: {err:?}")).unwrap();

    // Perform analysis on the executable (e.g., necessary for disassembly, control flow graph, etc..).
    let mut analysis = Analysis::from_executable(&executable).unwrap();

    // Used to track all immutable datas in order to create a table with their possible associated values
    let mut imm_tracker = ImmutableTracker::new(program.len());
    let imm_tracker_wrapped = Some(&mut imm_tracker);

    match mode {
        ReverseOutputMode::Disassembly(path) => {
            let _ = disassemble_wrapper(&program, &mut analysis, imm_tracker_wrapped, &path);
        }
        ReverseOutputMode::ControlFlowGraph(path) => {
            export_cfg_to_dot(&program, &mut analysis, &path)?;
        }
        ReverseOutputMode::DisassemblyAndCFG(path) => {
            let _ = disassemble_wrapper(&program, &mut analysis, imm_tracker_wrapped, &path);
            export_cfg_to_dot(&program, &mut analysis, &path)?;
        }
        ReverseOutputMode::DisassAndRustEquivalent(path) => {
            println!("Rust equivalent generation is not implemented yet.");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let _ = analyze_program(
            ReverseOutputMode::DisassemblyAndCFG("test_cases/base_sbf_addition_checker/out1/".to_string()),
            "test_cases/base_sbf_addition_checker/bytecodes/addition_checker.so".to_string(),
            true
        );
    }

    #[test]
    fn test2() {
        let _ = analyze_program(
            ReverseOutputMode::DisassemblyAndCFG("test_cases/base_sbf_addition_checker/out2/".to_string()),
            "test_cases/base_sbf_addition_checker/bytecodes/addition_checker_sbpf_solana.so".to_string(),
            false
        );
    }
}
