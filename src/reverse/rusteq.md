Instructions by Class
---------------------

The following Rust equivalents assume that:
- `src` and `dst` registers are `u64`
- `imm` is `u32`
- `off` is `u16`

### Memory Load or 32 bit Arithmetic and Logic
| opcode (hex / bin) | feature set | assembler mnemonic     | Rust equivalent
| ------------------ | ----------- | ---------------------- | ---------------
| `04` / `00000100`  | until v2    | `add32 dst, imm`       | `dst = (dst as u32).wrapping_add(imm) as i32 as i64 as u64`
| `04` / `00000100`  | from v2     | `add32 dst, imm`       | `dst = (dst as u32).wrapping_add(imm) as u64`
| `0C` / `00001100`  | until v2    | `add32 dst, src`       | `dst = (dst as u32).wrapping_add(src as u32) as i32 as i64 as u64`
| `0C` / `00001100`  | from v2     | `add32 dst, src`       | `dst = (dst as u32).wrapping_add(src as u32) as u64`
| `14` / `00010100`  | until v2    | `sub32 dst, imm`       | `dst = (dst as u32).wrapping_sub(imm) as u64`
| `14` / `00010100`  | from v2     | `sub32 dst, imm`       | `dst = imm.wrapping_sub(dst as u32) as u64`
| `1C` / `00011100`  | until v2    | `sub32 dst, src`       | `dst = (dst as u32).wrapping_sub(src as u32) as i32 as i64 as u64`
| `1C` / `00011100`  | from v2     | `sub32 dst, src`       | `dst = (dst as u32).wrapping_sub(src as u32) as u64`
| `24` / `00100100`  | until v2    | `mul32 dst, imm`       | `dst = (dst as i32).wrapping_mul(imm as i32) as i64 as u64`
| `24` / `00100100`  | from v2     | -- reserved --
| `2C` / `00101100`  | until v2    | `mul32 dst, src`       | `dst = (dst as i32).wrapping_mul(src as i32) as i64 as u64`
| `2C` / `00101100`  | from v2     | `ldxb dst, [src + off]`
| `34` / `00110100`  | until v2    | `div32 dst, imm`       | `dst = ((dst as u32) / imm) as u64`
| `34` / `00110100`  | from v2     | -- reserved --
| `3C` / `00111100`  | until v2    | `div32 dst, src`       | `dst = ((dst as u32) / (src as u32)) as u64`
| `3C` / `00111100`  | from v2     | `ldxh dst, [src + off]`
| `44` / `01000100`  | all         | `or32 dst, imm`        | `dst = (dst as u32).or(imm) as u64`
| `4C` / `01001100`  | all         | `or32 dst, src`        | `dst = (dst as u32).or(src as u32) as u64`
| `54` / `01010100`  | all         | `and32 dst, imm`       | `dst = (dst as u32).and(imm) as u64`
| `5C` / `01011100`  | all         | `and32 dst, src`       | `dst = (dst as u32).and(src as u32) as u64`
| `64` / `01100100`  | all         | `lsh32 dst, imm`       | `dst = (dst as u32).wrapping_shl(imm) as u64`
| `6C` / `01101100`  | all         | `lsh32 dst, src`       | `dst = (dst as u32).wrapping_shl(src as u32) as u64`
| `74` / `01110100`  | all         | `rsh32 dst, imm`       | `dst = (dst as u32).wrapping_shr(imm) as u64`
| `7C` / `01111100`  | all         | `rsh32 dst, src`       | `dst = (dst as u32).wrapping_shr(src as u32) as u64`
| `84` / `10000100`  | until v2    | `neg32 dst`            | `dst = (dst as i32).wrapping_neg() as u32 as u64`
| `84` / `10000100`  | from v2     | -- reserved --
| `8C` / `10001100`  | until v2    | -- reserved --
| `8C` / `01100001`  | from v2     | `ldxw dst, [src + off]`
| `94` / `10010100`  | until v2    | `mod32 dst, imm`       | `dst = ((dst as u32) % imm) as u64`
| `94` / `10010100`  | from v2     | -- reserved --
| `9C` / `10011100`  | until v2    | `mod32 dst, src`       | `dst = ((dst as u32) % (src as u32)) as u64`
| `9C` / `01111001`  | from v2     | `ldxdw dst, [src + off]`
| `A4` / `10100100`  | all         | `xor32 dst, imm`       | `dst = (dst as u32).xor(imm) as u64`
| `AC` / `10101100`  | all         | `xor32 dst, src`       | `dst = (dst as u32).xor(src as u32) as u64`
| `B4` / `10110100`  | all         | `mov32 dst, imm`       | `dst = imm as u64`
| `BC` / `10111100`  | until v2    | `mov32 dst, src`       | `dst = src as u32 as u64`
| `BC` / `10111100`  | from v2     | `mov32 dst, src`       | `dst = src as i32 as i64 as u64`
| `C4` / `11000100`  | all         | `ash32 dst, imm`       | `dst = (dst as i32).wrapping_shr(imm) as u32 as u64`
| `CC` / `11001100`  | all         | `ash32 dst, src`       | `dst = (dst as i32).wrapping_shr(src as u32) as u32 as u64`
| `D4` / `11010100`  | until v2    | `le dst, imm` | `dst = dst as u32 as u64`
| `D4` / `11010100`  | from v2     | -- reserved --
| `DC` / `11011100`  | all         | `be dst, imm` | `dst = match imm { 16 => (dst as u16).swap_bytes() as u64, 32 => (dst as u32).swap_bytes() as u64, 64 => dst.swap_bytes() }`
| `E4` to `FC`       | all         | -- reserved --

### Memory Store or 64 bit Arithmetic and Logic
| opcode (hex / bin) | feature set | assembler mnemonic | Rust equivalent
| ------------------ | ----------- | ------------------ | ---------------
| `07` / `00000111`  | all         | `add64 dst, imm`   | `dst = dst.wrapping_add(imm as i32 as i64 as u64)`
| `0F` / `00001111`  | all         | `add64 dst, src`   | `dst = dst.wrapping_add(src)`
| `17` / `00010111`  | until v2    | `sub64 dst, imm`   | `dst = dst.wrapping_sub(imm as i32 as i64 as u64)`
| `17` / `00010111`  | from v2     | `sub64 dst, imm`   | `dst = (imm as i32 as i64 as u64).wrapping_sub(dst)`
| `1F` / `00011111`  | all         | `sub64 dst, src`   | `dst = dst.wrapping_sub(src)`
| `27` / `00100111`  | until v2    | `mul64 dst, imm`   | `dst = dst.wrapping_mul(imm as u64)`
| `27` / `01110010`  | from v2     | `stb [dst + off], imm`
| `2F` / `00101111`  | until v2    | `mul64 dst, src`   | `dst = dst.wrapping_mul(src)`
| `2F` / `01110011`  | from v2     | `stxb [dst + off], src`
| `37` / `00110111`  | until v2    | `div64 dst, imm`   | `dst = dst / (imm as u64)`
| `37` / `01101010`  | from v2     | `sth [dst + off], imm`
| `3F` / `00111111`  | until v2    | `div64 dst, src`   | `dst = dst / src`
| `3F` / `01101011`  | from v2     | `stxh [dst + off], src`
| `47` / `01000111`  | all         | `or64 dst, imm`    | `dst = dst.or(imm)`
| `4F` / `01001111`  | all         | `or64 dst, src`    | `dst = dst.or(src)`
| `57` / `01010111`  | all         | `and64 dst, imm`   | `dst = dst.and(imm)`
| `5F` / `01011111`  | all         | `and64 dst, src`   | `dst = dst.and(src)`
| `67` / `01100111`  | all         | `lsh64 dst, imm`   | `dst = dst.wrapping_shl(imm)`
| `6F` / `01101111`  | all         | `lsh64 dst, src`   | `dst = dst.wrapping_shl(src as u32)`
| `77` / `01110111`  | all         | `rsh64 dst, imm`   | `dst = dst.wrapping_shr(imm)`
| `7F` / `01111111`  | all         | `rsh64 dst, src`   | `dst = dst.wrapping_shr(src as u32)`
| `87` / `10000111`  | until v2    | `neg64 dst`        | `dst = (dst as i64).wrapping_neg() as u64`
| `87` / `01100010`  | from v2     | `stw [dst + off], imm`
| `8F` / `10001111`  | until       | -- reserved --
| `8F` / `01100011`  | from v2     | `stxw [dst + off], src`
| `97` / `10010111`  | until v2    | `mod64 dst, imm`   | `dst = dst % (imm as u64)`
| `97` / `01111010`  | from v2     | `stdw [dst + off], imm`
| `9F` / `10011111`  | until v2    | `mod64 dst, src`   | `dst = dst % src`
| `9F` / `01111011`  | from v2     | `stxdw [dst + off], src`
| `A7` / `10100111`  | all         | `xor64 dst, imm`   | `dst = dst.xor(imm)`
| `AF` / `10101111`  | all         | `xor64 dst, src`   | `dst = dst.xor(src)`
| `B7` / `10110111`  | all         | `mov64 dst, imm`   | `dst = imm as i32 as i64 as u64`
| `BF` / `10111111`  | all         | `mov64 dst, src`   | `dst = src`
| `C7` / `11000111`  | all         | `ash64 dst, imm`   | `dst = (dst as i64).wrapping_shr(imm)`
| `CF` / `11001111`  | all         | `ash64 dst, src`   | `dst = (dst as i64).wrapping_shr(src as u32)`
| `D7` to `EF`       | all         | -- reserved --
| `F7` / `11110111`  | until v2    | -- reserved --
| `F7` / `11110111`  | from v2     | `hor64 dst, imm`   | `dst = dst.or((imm as u64).wrapping_shl(32))`
| `FF` / `11111111`  | all         | -- reserved --

#### Panics
- Division by zero: `if src == 0`
- Negative overflow: `if src == -1 && dst == u64::MIN`

| opcode (hex / bin) | feature set | assembler mnemonic | Rust equivalent
| ------------------ | ----------- | ------------------ | ---------------
| `06` to `2E`       | all         | -- reserved --
| `36` / `00110110`  | from v2     | `uhmul64 dst, imm` | `dst = (dst as u128).wrapping_mul(imm as u128).wrapping_shr(64) as u64`
| `3E` / `00111110`  | from v2     | `uhmul64 dst, src` | `dst = (dst as u128).wrapping_mul(src as u128).wrapping_shr(64) as u64`
| `46` / `01000110`  | from v2     | `udiv32 dst, imm`  | `dst = ((dst as u32) / imm) as u64`
| `4E` / `01001110`  | from v2     | `udiv32 dst, src`  | `dst = ((dst as u32) / (src as u32)) as u64`
| `56` / `01010110`  | from v2     | `udiv64 dst, imm`  | `dst = dst / (imm as u64)`
| `5E` / `01011110`  | from v2     | `udiv64 dst, src`  | `dst = dst / src`
| `66` / `01100110`  | from v2     | `urem32 dst, imm`  | `dst = ((dst as u32) % imm) as u64`
| `6E` / `01101110`  | from v2     | `urem32 dst, src`  | `dst = ((dst as u32) % (src as u32)) as u64`
| `76` / `01110110`  | from v2     | `urem64 dst, imm`  | `dst = dst % (imm as u64)`
| `7E` / `01111110`  | from v2     | `urem64 dst, src`  | `dst = dst % src`
| `86` / `10000110`  | from v2     | `lmul32 dst, imm`  | `dst = (dst as i32).wrapping_mul(imm as i32) as u32 as u64`
| `8E` / `10001110`  | from v2     | `lmul32 dst, src`  | `dst = (dst as i32).wrapping_mul(src as i32) as u32 as u64`
| `96` / `10010110`  | from v2     | `lmul64 dst, imm`  | `dst = dst.wrapping_mul(imm as u64)`
| `9E` / `10011110`  | from v2     | `lmul64 dst, src`  | `dst = dst.wrapping_mul(src)`
| `A6` to `AE`       | all         | -- reserved --
| `B6` / `10110110`  | from v2     | `shmul64 dst, imm` | `dst = (dst as i128).wrapping_mul(imm as i32 as i128).wrapping_shr(64) as i64 as u64`
| `BE` / `10111110`  | from v2     | `shmul64 dst, src` | `dst = (dst as i128).wrapping_mul(src as i64 as i128).wrapping_shr(64) as i64 as u64`
| `C6` / `11000110`  | from v2     | `sdiv32 dst, imm`  | `dst = ((dst as i32) / (imm as i32)) as u32 as u64`
| `CE` / `11001110`  | from v2     | `sdiv32 dst, src`  | `dst = ((dst as i32) / (src as i32)) as u32 as u64`
| `D6` / `11010110`  | from v2     | `sdiv64 dst, imm`  | `dst = ((dst as i64) / (imm as i64)) as u64`
| `DE` / `11011110`  | from v2     | `sdiv64 dst, src`  | `dst = ((dst as i64) / (src as i64)) as u64`
| `E6` / `11100110`  | from v2     | `srem32 dst, imm`  | `dst = ((dst as i32) % (imm as i32)) as u32 as u64`
| `EE` / `11101110`  | from v2     | `srem32 dst, src`  | `dst = ((dst as i32) % (src as i32)) as u32 as u64`
| `F6` / `11110110`  | from v2     | `srem64 dst, imm`  | `dst = ((dst as i64) % (imm as i64)) as u64`
| `FE` / `11111110`  | from v2     | `srem64 dst, src`  | `dst = ((dst as i64) % (src as i64)) as u64`

### Deprecated Memory Load and Store

#### Panics
- Out of bounds: When the memory location is not mapped.
- Access violation: When a store to a readonly region happens.

| opcode (hex / bin) | feature set | assembler mnemonic | Rust equivalent
| ------------------ | ----------- | ------------------ | ---------------
| `00` / `00000000`  | until v2    | `lddw dst, imm`    | `dst = dst.or((imm as u64).wrapping_shl(32))`
| `08` to `10`       | all         | -- reserved --
| `18` / `00011000`  | until v2    | `lddw dst, imm`    | `dst = imm as u64`
| `20` to `F8`       | all         | -- reserved --


#### Panics
- Out of bounds: When the target location is outside the bytecode if < v3.
- Out of bounds: When the target location is outside the current function if ≥ v3 and a jump.
- Out of bounds: When the target location is not a registered function if ≥ v3 and a call.
- Second slot of `lddw`: When the target location has opcode `0x00`.
- Stack overflow: When one too many nested call happens.

| opcode (hex / bin) | feature set | assembler mnemonic   | condition Rust equivalent
| ------------------ | ----------- | -------------------- | -------------------------
| `05` / `00000101`  | all         | `ja off`             | `true`
| `0D` / `00001101`  | all         | -- reserved --
| `15` / `00010101`  | all         | `jeq dst, imm, off`  | `dst == (imm as i32 as i64 as u64)`
| `1D` / `00011101`  | all         | `jeq dst, src, off`  | `dst == src`
| `25` / `00100101`  | all         | `jgt dst, imm, off`  | `dst > (imm as i32 as i64 as u64)`
| `2D` / `00101101`  | all         | `jgt dst, src, off`  | `dst > src`
| `35` / `00110101`  | all         | `jge dst, imm, off`  | `dst >= (imm as i32 as i64 as u64)`
| `3D` / `00111101`  | all         | `jge dst, src, off`  | `dst >= src`
| `45` / `01000101`  | all         | `jset dst, imm, off` | `dst.and(imm as i32 as i64 as u64) != 0`
| `4D` / `01001101`  | all         | `jset dst, src, off` | `dst.and(src) != 0`
| `55` / `01010101`  | all         | `jne dst, imm, off`  | `dst != (imm as i32 as i64 as u64)`
| `5D` / `01011101`  | all         | `jne dst, src, off`  | `dst != src`
| `65` / `01100101`  | all         | `jsgt dst, imm, off` | `(dst as i64) > (imm as i32 as i64)`
| `6D` / `01101101`  | all         | `jsgt dst, src, off` | `(dst as i64) > (src as i64)`
| `75` / `01110101`  | all         | `jsge dst, imm, off` | `(dst as i64) >= (imm as i32 as i64)`
| `7D` / `01111101`  | all         | `jsge dst, src, off` | `(dst as i64) >= (src as i64)`
| `85` / `10000101`  | until v3    | `call imm` or `syscall imm`
| `85` / `10000101`  | from v3     | `call off`
| `8D` / `10001101`  | until v2    | `callx imm`
| `8D` / `10001101`  | from v2     | `callx src`
| `95` / `10010101`  | until v3    | `exit` or `return`
| `95` / `10010101`  | from v3     | `syscall imm`
| `9D` / `10011101`  | until v3    | -- reserved --
| `9D` / `10011101`  | from v3     | `exit` or `return`
| `A5` / `10100101`  | all         | `jlt dst, imm, off`  | `dst < imm as i32 as i64 as u64`
| `AD` / `10101101`  | all         | `jlt dst, src, off`  | `dst < src`
| `B5` / `10110101`  | all         | `jle dst, imm, off`  | `dst <= imm as i32 as i64 as u64`
| `BD` / `10111101`  | all         | `jle dst, src, off`  | `dst <= src`
| `C5` / `11000101`  | all         | `jslt dst, imm, off` | `(dst as i64) < (imm as i32 as i64)`
| `CD` / `11001101`  | all         | `jslt dst, src, off` | `(dst as i64) < (src as i64)`
| `D5` / `11010101`  | all         | `jsle dst, imm, off` | `(dst as i64) <= (imm as i32 as i64)`
| `DD` / `11011101`  | all         | `jsle dst, src, off` | `(dst as i64) <= (src as i64)`
| `E5` to `FD`       | all         | -- reserved --
