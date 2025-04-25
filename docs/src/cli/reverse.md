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
  --labeling
```

**Arguments:**

- `--mode`: Output mode. One of:
  - `disass`: Disassemble the bytecode
  - `cfg`: Export control flow graph
  - `both`: Disassemble and export CFG
  - `rusteq`: *(Not implemented yet)*
- `--out-dir`: Output directory for result files.
- `--bytecodes-file`: Path to the compiled `.so` file.
- `--labeling`: Enables use of symbol and section labels when available.

---

## Modes

| Mode     | Description                                | Output Files                  |
|----------|--------------------------------------------|-------------------------------|
| `disass` | Disassembles bytecode and extracts immediates | `disassembly.out`, `immediate_data_table.out` |
| `cfg`    | Builds a `.dot` graph from instruction flow | `cfg.dot`                     |
| `both`   | Performs both operations                    | All of the above              |
| `rusteq` | (WIP) Generate Rust-like output             | None                          |

---

## Outputs

Depending on the selected mode, the following files may be generated in `--out-dir`:

- `disassembly.out`: Human-readable disassembly of eBPF instructions
- `immediate_data_table.out`: Table of loaded immediate data from `.rodata`
- `cfg.dot`: Graphviz-compatible control flow graph

You can visualize `cfg.dot` using tools like:

```bash
dot -Tpng cfg.dot -o cfg.png
```

---

## Example

```bash
cargo run -- reverse \
  --mode both \
  --out-dir test_cases/base_sbf_addition_checker/out1/ \
  --bytecodes-file test_cases/base_sbf_addition_checker/bytecodes/addition_checker.so \
  --labeling
```

---

## Related

- [Disassembly details](../reverse/disassembly.md)
- [Control Flow Graph](../reverse/cfg.md)
- [Immediate Tracking](../reverse/immediates.md)
