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
- Adding Rust-like comparison for better understanding

---

## Example

Here’s a disassembly snippet produced by Sol-azy:

```
entrypoint:
    mov64 r2, r1                                    r2 = r1
    mov64 r1, r10                                   r1 = r10
    add64 r1, -96                                   r1 += -96   ///  r1 = r1.wrapping_add(-96 as i32 as i64 as u64)
    call function_308                       
    ldxdw r7, [r10-0x48]                    
    ldxdw r8, [r10-0x58]                    
    ldxdw r1, [r10-0x38]                    
    mov64 r2, 8                                     r2 = 8 as i32 as i64 as u64
    jgt r2, r1, lbb_91                              if r2 > r1 { pc += 79 }
    ldxdw r1, [r10-0x40]                    
    ldxw r2, [r1+0x0]                       
    stxw [r10-0xa8], r2                     
    ldxw r1, [r1+0x4]                       
    stxw [r10-0xa4], r1                     
    mov64 r1, 0                                     r1 = 0 as i32 as i64 as u64
    stxdw [r10-0x40], r1                    
    lddw r1, 0x100004610 --> b"\x00\x00\x00\x00\xd0C\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00…        r1 load str located at 4294985232
    stxdw [r10-0x60], r1                    
    mov64 r1, 2                                     r1 = 2 as i32 as i64 as u64
    stxdw [r10-0x58], r1                    
    stxdw [r10-0x48], r1                    
    mov64 r1, r10                                   r1 = r10
    add64 r1, -136                                  r1 += -136   ///  r1 = r1.wrapping_add(-136 as i32 as i64 as u64)
    stxdw [r10-0x50], r1                    
    mov64 r1, r10                                   r1 = r10
    add64 r1, -164                                  r1 += -164   ///  r1 = r1.wrapping_add(-164 as i32 as i64 as u64)
    stxdw [r10-0x78], r1                    
    lddw r1, 0x100004210 --> b"\xbf#\x00\x00\x00\x00\x00\x00a\x11\x00\x00\x00\x00\x00\x00\xb7\x02\x00\x0…        r1 load str located at 4294984208
    stxdw [r10-0x70], r1                    
    stxdw [r10-0x80], r1                    
    mov64 r1, r10                                   r1 = r10
    add64 r1, -168                                  r1 += -168   ///  r1 = r1.wrapping_add(-168 as i32 as i64 as u64)
    stxdw [r10-0x88], r1                    
    mov64 r1, r10                                   r1 = r10
    add64 r1, -160                                  r1 += -160   ///  r1 = r1.wrapping_add(-160 as i32 as i64 as u64)
    mov64 r2, r10                                   r2 = r10
    add64 r2, -96                                   r2 += -96   ///  r2 = r2.wrapping_add(-96 as i32 as i64 as u64)
    call function_858                       
    ldxdw r1, [r10-0xa0]                    
    ldxdw r2, [r10-0x90]                    
    syscall [invalid]                       
    ldxw r1, [r10-0xa8]                     
    ldxw r2, [r10-0xa4]                     
    add64 r2, r1                                    r2 += r1   ///  r2 = r2.wrapping_add(r1)
    lsh64 r2, 32                                    r2 <<= 32   ///  r2 = r2.wrapping_shl(32)
    rsh64 r2, 32                                    r2 >>= 32   ///  r2 = r2.wrapping_shr(32)
    jne r2, 1337, lbb_58                            if r2 != (1337 as i32 as i64 as u64) { pc += 6 }
    lddw r1, 0x1000043e0 --> b"You win!"            r1 load str located at 4294984672
    mov64 r2, 8                                     r2 = 8 as i32 as i64 as u64
    syscall [invalid]                       
    mov64 r1, 987654321                             r1 = 987654321 as i32 as i64 as u64
    ja lbb_63                                       if true { pc += 5 }
lbb_58:
    lddw r1, 0x1000043e8 --> b"You lose!"           r1 load str located at 4294984680
    mov64 r2, 9                                     r2 = 9 as i32 as i64 as u64
    syscall [invalid]                       
    mov64 r1, 123456789                             r1 = 123456789 as i32 as i64 as u64
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
