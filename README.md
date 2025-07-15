# sol-azy

**sol-azy** is a modular CLI toolchain for static analysis and reverse engineering of Solana sBPF programs.  
It supports disassembly, control flow analysis, and custom Starlark-based rule evaluation.

## Features

- Compile Solana programs (Anchor or native SBF)
- Perform static AST-based security analysis with Starlark rules
- Reverse-engineer `.so` bytecode: disassembly (& rust equivalences), control flow graphs, and immediate value decoding
- Modular CFG editing (`dotting`)
- On-chain binary or raw AccountInfo retrieval (`fetcher`)

## Getting Started

### Prerequisites

- [Rust + Cargo](https://www.rust-lang.org/tools/install)
- (only for Build command)[ Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- (Optional) [Anchor](https://www.anchor-lang.com/) for Anchor-based projects

### Installation

```bash
git clone https://github.com/your-org/sol-azy.git
cd sol-azy
cargo build --release
````

Or for development:

```bash
cargo run -- <command> [options]
```

## Usage

Here are some basic examples. See [`docs/book`](docs/book) for full documentation.

### Build a project

```bash
cargo run -- build --target-dir ./my_project --out-dir ./out/
```

### Run static analysis

```bash
cargo run -- sast --target-dir ./my_project --rules-dir ./rules/
```

> HIGHLY RECOMMENDED: Using the --release is wayyyyy faster, so if you don’t need debug logs, I’d recommend using it

### Reverse engineer a compiled `.so`

```bash
cargo run -- reverse --mode both --out-dir ./out --bytecodes-file ./program.so --labeling --reduced
```

### Fetch deployed bytecode from the mainnet

```bash
cargo run -- fetcher --program-id <PROGRAM_ID> --out-dir ./out/
```

### Reinsert functions in a reduced CFG

```bash
cargo run -- dotting -c functions.json -r reduced.dot -f full.dot
```

## Documentation

The project includes full [mdBook documentation](https://github.com/FuzzingLabs/sol-azy/tree/master/docs):

### Serve the docs

```bash
cd book
mdbook serve
```

### Build the docs

```bash
cd book
mdbook build
```

## License

This project is licensed under the [Server Side Public License v1](./LICENSE)

## Contact

If you have any questions, suggestions, or need support:

- Feel free to [open an issue](https://github.com/FuzzingLabs/sol-azy/issues).
- You can also reach out directly to _(responses may take longer than issue depending on individual availability)_:
    - [FuzzingLabs](https://x.com/FuzzingLabs)
    - [MohaFuzzingLabs](https://github.com/MohaFuzzingLabs)
    - [Ectario](https://x.com/Ectari0)

We're happy to help and value community engagement!