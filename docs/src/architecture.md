# Architecture

Sol-azy is a modular static analysis toolkit designed to work on Solana programs compiled to eBPF.  
It is capable of disassembling, analyzing control flow, decoding embedded `.rodata` strings, and performing pattern-based syntactic analysis through rule-based AST matching.

---

## High-Level Design

Sol-azy is structured around **three core engines**:

1. **Reverse Engine** (binary-level disassembler + CFG)
2. **SAST Engine** (source-level AST scanning and pattern rules)
3. **Build Engine** (project type detection and bytecode compilation)

Each engine is triggered via a dedicated CLI subcommand (`reverse`, `sast`, `build`).

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

- [sbpf-solana (anza-xyz)](https://github.com/anza-xyz/sbpf-solana): Disassembly / Analysis core
- [`syn`](https://docs.rs/syn): Source AST parsing
- [`starlark-rust`](https://github.com/facebook/starlark-rust): Rule evaluation engine
- `Graphviz`: For CFG rendering

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