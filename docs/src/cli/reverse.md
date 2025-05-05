# `reverse` Command

The `reverse` command performs static reverse engineering on compiled Solana eBPF bytecode (`.so` files).  
It supports disassembly, control flow graph (CFG) generation, and immediate data inspection.

---

## Usage

```bash
cargo run -- reverse \
  --mode both \
  --out-dir ./out/ \
  --bytecodes-file ./bytecodes/program.so \
  --labeling \
  --reduced \
  --only-entrypoint
```

**Arguments:**

* `--mode`: Output mode. One of:

  * `disass`: Disassemble the bytecode
  * `cfg`: Export control flow graph
  * `both`: Disassemble and export CFG
  * `rusteq`: *(Not implemented yet)*
* `--out-dir`: Output directory for result files.
* `--bytecodes-file`: Path to the compiled `.so` file.
* `--labeling`: Enables use of symbol and section labels when available.
* `--reduced`: *(Optional)* Excludes functions defined before the entrypoint (often library or startup code).
* `--only-entrypoint`: *(Optional)* Only generates the CFG for the entrypoint function, allowing custom extension via dotting.

---

## Modes

| Mode     | Description                                   | Output Files                                  |
| -------- | --------------------------------------------- | --------------------------------------------- |
| `disass` | Disassembles bytecode and extracts immediates | `disassembly.out`, `immediate_data_table.out` |
| `cfg`    | Builds a `.dot` graph from instruction flow   | `cfg.dot`                                     |
| `both`   | Performs both operations                      | All of the above                              |
| `rusteq` | (WIP) Generate Rust-like output               | None                                          |

---

## Output Files

Depending on the selected mode and options, the following files may be generated in `--out-dir`:

* `disassembly.out`: Human-readable disassembly of eBPF instructions
* `immediate_data_table.out`: Table of `.rodata` strings and constants
* `cfg.dot`: Full control flow graph

You can visualize `.dot` files using tools like:

```bash
dot -Tpng cfg.dot -o cfg.png
xdot cfg.dot
```

> ⚠️ For very large programs, even the `--reduced` version of the CFG can take significant time to generate due to the size and complexity of the bytecode being analyzed and rendered by `dot`.

---

## Example

```bash
cargo run -- reverse \
  --mode both \
  --out-dir test_cases/base_sbf_addition_checker/out1/ \
  --bytecodes-file test_cases/base_sbf_addition_checker/bytecodes/addition_checker.so \
  --labeling \
  --reduced
```

This command will disassemble the program and generate both a full and reduced CFG.

---

## Advanced Use Case

If using `--only-entrypoint`, Sol-azy will generate a minimal CFG with only the entrypoint's subgraph.
You can later extend this graph manually using [`dotting`](../reverse/dotting.md) with a JSON list of function clusters to add.

---

## Related

* [Disassembly details](../reverse/disassembly.md)
* [Control Flow Graph](../reverse/cfg.md)
* [Immediate Tracking](../reverse/immediates.md)
* [Dotting (manual CFG editing)](../reverse/dotting.md)