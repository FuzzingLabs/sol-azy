# CLI Usage

Sol-azy provides a command-line interface (CLI) for interacting with Solana programs through various operations:

- Building programs
- Running static analysis
- Reversing compiled bytecode
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

### `test` *(Coming soon)*

Will allow targeted test case execution and integration with test campaign definitions.

---

### `fuzz` *(Coming soon)*

Will provide integrations with fuzzing engines.

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