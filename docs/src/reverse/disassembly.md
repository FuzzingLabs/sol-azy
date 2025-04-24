# Disassembly

Sol-azy statically disassembles compiled Solana eBPF programs into a readable, instruction-by-instruction view.  
This view is enhanced with **immediate data decoding**, especially for strings loaded from `.rodata`.

---

## Overview

The disassembly engine in Sol-azy builds upon `sbpf-solana`'s instruction decoder.  
It adds layers of audit-focused context by:

- Labeling basic blocks (e.g., `lbb_42`)
- Resolving immediate values from `.rodata`
- Emitting annotated output into `disassembly.out`

---

## Example

Here’s a disassembly snippet produced by Sol-azy:

```
lbb_58:
    ldxdw  r1, [r10-0xa0]
    ldxdw  r2, [r10-0x90]
    syscall [invalid]
    ldxw   r1, [r10-0xa8]
    ldxw   r2, [r10-0x4]
    add64  r2, r1
    lsh64  r2, 32
    rsh64  r2, 32
    jne    r2, 1337, lbb_59

lbb_59:
    lddw   r1, 0x1000043e0   --> b"You win!"
    mov64  r2, 8
    syscall [invalid]
    mov64  r1, 987654321
    ja     lbb_63

lbb_60:
    lddw   r1, 0x1000043e8   --> b"You lose!"
    mov64  r2, 9
    syscall [invalid]
    mov64  r1, 123456789
```

---

## Annotating Immediate Loads

Instructions like:

```text
lddw   r1, 0x1000043e0
```

point into `.rodata`. Sol-azy:

1. Checks if `imm >= MM_RODATA_START`
2. Extracts the corresponding bytes from the `.so`
3. Uses the next `MOV64_IMM` (here, `mov64 r2, 8`) to determine the length
4. Displays a byte string: `b"You win!"`

This process is handled by:

```rust
fn get_string_repr(program: &[u8], insn: &Insn, next: Option<&Insn>) -> String
```

---

## Visualization

Here is an example of a control flow graph with disassembly and immediate data decoded:

![Disassembly with .rodata decoded](../images/reverse_cfg_example.png)

- Arrows represent jumps or branches
- Blocks show disassembled instructions
- `--> b"...string..."` indicates `.rodata` interpretation

---

## Output Files

When running:

```bash
cargo run -- reverse --mode disass --out-dir ./out --bytecodes-file ./program.so
```

You get:

| File                  | Description                                  |
|-----------------------|----------------------------------------------|
| `disassembly.out`     | Main instruction listing with annotations    |
| `immediate_data_table.out` | All tracked immediate memory ranges      |

Example from `immediate_data_table.out`:

```
0x1000043e0 (+ 0x43e0): b"You win!"
0x1000043e8 (+ 0x43e8): b"You lose!"
```

---

## Tips

- Enable `--labeling` to auto-gen labels.
- Use `mode = both` to get disassembly + CFG together.

---

## Related

- [Immediate Tracking](immediates.md)
- [Control Flow Graph](cfg.md)
- [Reverse CLI Command](../cli/reverse.md)
