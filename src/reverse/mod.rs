pub mod disass;

/// Defines the output mode for the analysis function.
pub enum ReverseOutputMode {
    /// Only disassemble the program to stdout.
    Disassembly,
    /// Export the control flow graph (CFG) to a .dot file.
    ControlFlowGraph(String), // path to .dot output
    /// Disassemble and output CFG.
    DisassemblyAndCFG(String), // path to .dot output
    /// Future extension: Attempt to produce a Rust-like representation.
    DisassAndRustEquivalent(String), // path to .dot output (optional)
}