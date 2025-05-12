# CLI Usage

Sol-azy provides a command-line interface (CLI) for interacting with Solana programs through various operations:

- Building programs
- Running static analysis
- Reversing compiled bytecode
- Modifying CFG .dot files
- Fetching deployed bytecode
- (Future) Fuzzing and testing support

All commands are accessible via:

```bash
cargo run -- <command> [options]
```

---

## Available Commands

### [`build`](cli/build.md)

Compiles a Solana project using either Anchor or the native SBF toolchain.

```bash
cargo run -- build --target-dir ./my_project --out-dir ./out/
```

---

### [`sast`](cli/sast.md)

Runs static analysis using Starlark-based rules on the project's source code.

```bash
cargo run -- sast --target-dir ./my_project --rules-dir ./rules/ --syn-scan-only
```

---

### [`reverse`](cli/reverse.md)

Performs disassembly, control flow graph (CFG) generation, and immediate value extraction on compiled `.so` files.

```bash
cargo run -- reverse --mode both --out-dir ./out --bytecodes-file ./program.so --labeling
```

---

### [`dotting`](../reverse/dotting.md)

Allows you to edit a reduced control flow graph (`.dot`) by selectively re-inserting functions from the full graph.
This is especially useful when working with large binaries where the full CFG is too dense.

```bash
cargo run -- dotting \
  -c temp_config.json \
  -r cfg_reduced.dot \
  -f cfg.dot
```

---

### [`fetcher`](../reverse/fetcher.md)

Fetches an on-chain deployed Solana program’s bytecode (`.so`) using its program ID.
Useful when you want to analyze a program without having its local source or compiled artifact.

```bash
cargo run -- fetcher \
  --program-id 4MEX8vDCZzAxQkuyd6onJCTeFdof6c1HJgznEtCGqA1N \
  --out-dir ./bytecodes/
```

Optional RPC override:

```bash
cargo run -- fetcher \
  -p 4MEX8vDCZzAxQkuyd6onJCTeFdof6c1HJgznEtCGqA1N \
  -o ./bytecodes/ \
  -r https://api.mainnet-beta.solana.com
```

---

### `test` *(TO DO)*

---

### `fuzz` *(TO DO)*

---

## Quickstart

To get started with Sol-azy:

1. [Install prerequisites](installation.md)
2. [Build your project](cli/build.md)
3. [Run static analysis](cli/sast.md)
4. [Reverse engineer the bytecode](cli/reverse.md)

---

## Related

- [Architecture Overview](../architecture/app_state.md)
- [Writing Custom Rules](../rules/format.md)