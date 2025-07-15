# Architecture

sol-azy is a modular static analysis toolkit designed to work on Solana programs compiled to eBPF.  
It is capable of disassembling, analyzing control flow, decoding embedded `.rodata` strings, and performing pattern-based syntactic analysis through rule-based AST matching.

---

## High-Level Design

sol-azy is structured around **three main engines**, supported by **auxiliary modules**:

### Core Engines

1. **Reverse Engine**
   Handles binary-level disassembly, control flow graph generation, and `.rodata` analysis.
   → Triggered via the `reverse` CLI command.

2. **SAST Engine**
   Performs static source-level analysis using Starlark-based rule evaluation on Rust ASTs.
   → Triggered via the `sast` CLI command.

3. **Build Engine**
   Detects the project type (`Anchor`, `SBF`) and compiles the bytecode accordingly.
   → Triggered via the `build` CLI command.

### Supporting Modules

* **Dotting Module**
  Allows users to manually reintroduce function clusters into reduced CFGs by editing `.dot` files post-generation.
  → Useful for large programs or targeted function exploration.

* **Fetcher Module**
  Retrieves deployed program bytecode directly from on-chain Solana accounts via RPC.
  → Enables reverse analysis even without access to local source code.

Each component is designed to be **composable and scriptable**, making sol-azy flexible for both auditing and program analysis workflows.

---

## Component Overview

### 1. `reverse/`

Handles all operations on compiled `.so` eBPF files:

- `disass.rs`: Disassembler with inline `.rodata` resolution
- `cfg.rs`: Generates DOT graphs from the control flow
- `immediate_tracker.rs`: Tracks data regions used by `LD_DW_IMM`
- `utils.rs`: String formatting and decoding

It produces:

- `disassembly.out`
- `immediate_data_table.out`
- `cfg.dot`

→ See [Reverse Overview](reverse/overview.md)

---

### 2. `parsers/` + `state/sast_state.rs`

Used in source-level static analysis.

- Parses all `.rs` files into [`syn::File`] ASTs
- Builds `AstPositions` with span references
- Applies Starlark-based rules to nodes and attributes
- Aggregates findings into a `SastState`

→ See [SAST Overview](reverse/sast.md)

---

### 3. `engines/starlark_engine.rs`

Embeds a Starlark interpreter to run user-defined rules against Rust ASTs.

- Prepares the AST (JSON + span tracking)
- Loads `.star` files from `rules/`
- Invokes `syn_ast_rule(...)` with context
- Collects `matches` and metadata as JSON

→ See [Writing Rules](../rules/README.md)

---

### 4. `commands/`

Command routing layer:

- `build_command.rs`: Uses Anchor/Cargo to compile `.so` files
- `reverse_command.rs`: Dispatches disass + cfg generation
- `sast_command.rs`: Launches Starlark rule scanning

Used by `AppState::run_cli()` to manage flow.

---

### 5. `helpers/`

Utilities to:

- Detect project type (`Anchor.toml`, `Cargo.toml`)
- Check external dependencies (`cargo`, `anchor`)
- Run subprocesses with environment overrides

---

### 6. `dotting/`

Post-processing module for `.dot` control flow graphs:

* Allows restoring function subgraphs in reduced or entrypoint-only graphs
* Takes as input a list of function IDs (clusters) to reinject
* Outputs an `updated_*.dot` file with the requested functions and edges

This module is especially useful when a full CFG is too large or noisy, letting analysts rebuild targeted graphs incrementally.

→ See [Dotting](reverse/dotting.md)

---

### 7. `fetcher/`

Bytecode retrieval module for on-chain programs:

* Connects to Solana RPC endpoints
* Downloads the deployed `.so` bytecode of a program ID
* Saves the ELF file locally for reverse analysis

This feature is useful for audits where source code is unavailable or unverifiable.

→ See [Fetcher](cli/fetcher.md)

## Output Flow

```
        +----------------+
        | .so Bytecode   | ← built via cargo / anchor
        +----------------+
                 |
         [reverse_command]
                 ↓
        +-----------------------+
        |  Analysis (sbpf-solana)
        |  + immediate tracker
        +-----------------------+
          ↓        ↓        ↓
    disass  immediate   cfg.dot
     .out    _data.out

--------------------------------------

        +----------------+
        |  Rust Source   | ← e.g. Anchor project
        +----------------+
                 |
           [sast_command]
                 ↓
        +----------------------+
        | syn::File + spans    |
        | + rule evaluation    |
        +----------------------+
                 ↓
             Findings
           (printed / JSON)
```

---

## External Dependencies

- [`sbpf-solana (anza-xyz)`](https://github.com/anza-xyz/sbpf-solana): Disassembly / Analysis core
- [`syn`](https://docs.rs/syn): Source AST parsing
- [`starlark-rust`](https://github.com/facebook/starlark-rust): Rule evaluation engine

---

## Extensibility

The architecture is modular and designed for extension:

- Add new output formats by extending the `ReverseOutputMode`
- Plug in new analysis passes (e.g., MIR or LLVM IR) in `engines/`
- Write new rules without modifying Rust code (`.star` files)
- Integrate into CI pipelines via CLI interface

---

## Next Steps

- [Disassembly](reverse/disassembly.md)
- [Immediate Data Tracking](reverse/immediates.md)
- [SAST](./static_analysis.md)
- [CLI Usage](./cli_usage.md)