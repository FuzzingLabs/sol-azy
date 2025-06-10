# Reverse Overview

This section explains how Sol-azy performs static reverse engineering on Solana programs compiled to SBF.

The reverse module combines disassembly, control flow analysis, and memory inspection, using a customized static analysis engine adapted from [`sbpf-solana`](https://github.com/anza-xyz/sbpf-solana).

---

## How It Works

1. **ELF Parsing**

   Sol-azy loads the `.so` bytecode using Solana’s `Executable` abstraction (from `solana_rbpf`), which parses the ELF and loads its segments (e.g., `.text`, `.rodata`).

2. **Instruction Analysis**

   Using the `Analysis` struct from `sbpf-solana`, the tool walks through all valid instruction addresses, building:

   - A disassembled instruction list
   - Basic block boundaries
   - Cross-references and destination mappings

3. **Immediate Tracking**

   When `LD_DW_IMM` instructions reference `MM_RODATA`, Sol-azy tries to:

   - Interpret the referenced memory slice
   - Associate it with a `MOV64_IMM` or `MOV32_IMM` defining its length
   - Format the result as a printable string (e.g., `b"hello world"`)

4. **Graph Generation**

   For control flow graphs, each basic block becomes a node in a `.dot` file, with edges linking jumps, calls, and returns.

---

## Internal Components

- [`ImmediateTracker`](./immediates.md): Tracks memory ranges referenced by LD_DW_IMM
- [`get_string_repr`](./cfg.html#strings-from-rodata): Converts slices from `.rodata` into readable strings
- [`export_cfg_to_dot`](./cfg.html#overview): Emits Graphviz-compatible control flow graphs
- [`disassemble_wrapper`](./immediates.html#behind-the-scenes): Main entrypoint for disassembly + data extraction

---

## ReverseOutputMode

The CLI dispatches different logic depending on this enum:

```rust
pub enum ReverseOutputMode {
    Disassembly(String),
    ControlFlowGraph(String),
    DisassemblyAndCFG(String),
}
```

---

## Example Workflow (Recap)

```bash
cargo run -- reverse \
  --mode both \
  --out-dir ./out/ \
  --bytecodes-file ./bytecodes/program.so \
  --labeling
```
