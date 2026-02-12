use solana_sbpf::ebpf::{self, Insn};
use solana_sbpf::program::SBPFVersion;

/// Return the Rust equivalent of an SBPF instruction if available
pub fn translate_to_rust(insn: &ebpf::Insn, sbpf_version: SBPFVersion) -> Option<String> {
    // Handle version-specific opcodes (conflicts and version-exclusive instructions)
    let versioned = if sbpf_version >= SBPFVersion::V2 {
        match insn.opc {
            // V2+ arithmetic opcodes that were jump instructions in V1
            ebpf::UDIV32_IMM => udiv32_imm(insn),
            ebpf::UDIV32_REG => udiv32_reg(insn),
            ebpf::UREM32_IMM => urem32_imm(insn),
            ebpf::UREM32_REG => urem32_reg(insn),
            ebpf::SDIV32_IMM => sdiv32_imm(insn),
            ebpf::SDIV32_REG => sdiv32_reg(insn),
            ebpf::SREM32_IMM => srem32_imm(insn),
            ebpf::SREM32_REG => srem32_reg(insn),
            ebpf::LMUL32_IMM => lmul32_imm(insn),
            ebpf::LMUL32_REG => lmul32_reg(insn),

            // V2+ only extended arithmetic (did not exist in V1)
            ebpf::HOR64_IMM => hor64_imm(insn),
            ebpf::UHMUL64_IMM => uhmul64_imm(insn),
            ebpf::UHMUL64_REG => uhmul64_reg(insn),
            ebpf::UDIV64_IMM => udiv64_imm(insn),
            ebpf::UDIV64_REG => udiv64_reg(insn),
            ebpf::UREM64_IMM => urem64_imm(insn),
            ebpf::UREM64_REG => urem64_reg(insn),
            ebpf::LMUL64_IMM => lmul64_imm(insn),
            ebpf::LMUL64_REG => lmul64_reg(insn),
            ebpf::SHMUL64_IMM => shmul64_imm(insn),
            ebpf::SHMUL64_REG => shmul64_reg(insn),
            ebpf::SDIV64_IMM => sdiv64_imm(insn),
            ebpf::SDIV64_REG => sdiv64_reg(insn),
            ebpf::SREM64_IMM => srem64_imm(insn),
            ebpf::SREM64_REG => srem64_reg(insn),

            // Not a version-specific instruction, try common instructions
            _ => return translate_common(insn, sbpf_version),
        }
    } else {
        match insn.opc {
            // V1-only 32-bit arithmetic (deprecated in V2+)
            ebpf::MUL32_IMM => mul32_imm(insn),
            ebpf::MUL32_REG => mul32_reg(insn),
            ebpf::DIV32_IMM => div32_imm(insn),
            ebpf::DIV32_REG => div32_reg(insn),
            ebpf::MOD32_IMM => mod32_imm(insn),
            ebpf::MOD32_REG => mod32_reg(insn),
            ebpf::NEG32 => neg32(insn),

            // V1-only 64-bit arithmetic (deprecated in V2+)
            ebpf::MUL64_IMM => mul64_imm(insn),
            ebpf::MUL64_REG => mul64_reg(insn),
            ebpf::DIV64_IMM => div64_imm(insn),
            ebpf::DIV64_REG => div64_reg(insn),
            ebpf::MOD64_IMM => mod64_imm(insn),
            ebpf::MOD64_REG => mod64_reg(insn),
            ebpf::NEG64 => neg64(insn),

            // V1-only endianness conversion (LE instruction deprecated in V2+)
            ebpf::LE => le(insn),

            // V1-only jump instructions (conflict with V2+ arithmetic)
            ebpf::JGE32_IMM => jge32_imm(insn),
            ebpf::JGE32_REG => jge32_reg(insn),
            ebpf::JLE32_IMM => jle32_imm(insn),
            ebpf::JLE32_REG => jle32_reg(insn),
            ebpf::JSET32_IMM => jset32_imm(insn),
            ebpf::JSET32_REG => jset32_reg(insn),
            ebpf::JNE32_IMM => jne32_imm(insn),
            ebpf::JNE32_REG => jne32_reg(insn),
            ebpf::JSGT32_IMM => jsgt32_imm(insn),
            ebpf::JSGT32_REG => jsgt32_reg(insn),
            ebpf::JSGE32_IMM => jsge32_imm(insn),
            ebpf::JSGE32_REG => jsge32_reg(insn),
            ebpf::JSLT32_IMM => jslt32_imm(insn),
            ebpf::JSLT32_REG => jslt32_reg(insn),
            ebpf::JSLE32_IMM => jsle32_imm(insn),
            ebpf::JSLE32_REG => jsle32_reg(insn),

            // Not a version-specific instruction, try common instructions
            _ => return translate_common(insn, sbpf_version),
        }
    };

    Some(versioned)
}

/// Translate instructions that are stable across SBPF versions
fn translate_common(insn: &ebpf::Insn, sbpf_version: SBPFVersion) -> Option<String> {
    let result = match insn.opc {
        // === 32-bit Arithmetic and Logic ===
        ebpf::ADD32_IMM => add32_imm(insn, sbpf_version),
        ebpf::ADD32_REG => add32_reg(insn, sbpf_version),
        ebpf::SUB32_IMM => sub32_imm(insn, sbpf_version),
        ebpf::SUB32_REG => sub32_reg(insn, sbpf_version),
        // Note: MUL32, DIV32, MOD32, NEG32 are V1-only (handled in version-specific block)
        ebpf::OR32_IMM => or32_imm(insn),
        ebpf::OR32_REG => or32_reg(insn),
        ebpf::AND32_IMM => and32_imm(insn),
        ebpf::AND32_REG => and32_reg(insn),
        ebpf::XOR32_IMM => xor32_imm(insn),
        ebpf::XOR32_REG => xor32_reg(insn),
        ebpf::LSH32_IMM => lsh32_imm(insn),
        ebpf::LSH32_REG => lsh32_reg(insn),
        ebpf::RSH32_IMM => rsh32_imm(insn),
        ebpf::RSH32_REG => rsh32_reg(insn),
        ebpf::ARSH32_IMM => arsh32_imm(insn),
        ebpf::ARSH32_REG => arsh32_reg(insn),
        ebpf::MOV32_IMM => mov32_imm(insn),
        ebpf::MOV32_REG => mov32_reg(insn, sbpf_version),
        // Note: LE is V1-only (handled in version-specific block)
        ebpf::BE => be(insn),

        // === 64-bit Arithmetic and Logic ===
        ebpf::ADD64_IMM => add64_imm(insn),
        ebpf::ADD64_REG => add64_reg(insn),
        ebpf::SUB64_IMM => sub64_imm(insn, sbpf_version),
        ebpf::SUB64_REG => sub64_reg(insn),
        // Note: MUL64, DIV64, MOD64, NEG64 are V1-only (handled in version-specific block)
        ebpf::OR64_IMM => or64_imm(insn),
        ebpf::OR64_REG => or64_reg(insn),
        ebpf::AND64_IMM => and64_imm(insn),
        ebpf::AND64_REG => and64_reg(insn),
        ebpf::XOR64_IMM => xor64_imm(insn),
        ebpf::XOR64_REG => xor64_reg(insn),
        ebpf::LSH64_IMM => lsh64_imm(insn),
        ebpf::LSH64_REG => lsh64_reg(insn),
        ebpf::RSH64_IMM => rsh64_imm(insn),
        ebpf::RSH64_REG => rsh64_reg(insn),
        ebpf::ARSH64_IMM => arsh64_imm(insn),
        ebpf::ARSH64_REG => arsh64_reg(insn),
        ebpf::MOV64_IMM => mov64_imm(insn),
        ebpf::MOV64_REG => mov64_reg(insn),

        // === Load/Store ===
        ebpf::LD_DW_IMM => ld_dw_imm(insn, sbpf_version),

        // === 32-bit Jump Instructions (valid in both V1 and V2+) ===
        ebpf::JA => ja(insn),
        ebpf::JEQ32_IMM => jeq32_imm(insn),
        ebpf::JEQ32_REG => jeq32_reg(insn),
        ebpf::JGT32_IMM => jgt32_imm(insn),
        ebpf::JGT32_REG => jgt32_reg(insn),
        ebpf::JLT32_IMM => jlt32_imm(insn),
        ebpf::JLT32_REG => jlt32_reg(insn),

        // === 64-bit Jump Instructions (valid in both V1 and V2+) ===
        ebpf::JEQ64_IMM => jeq64_imm(insn),
        ebpf::JEQ64_REG => jeq64_reg(insn),
        ebpf::JGT64_IMM => jgt64_imm(insn),
        ebpf::JGT64_REG => jgt64_reg(insn),
        ebpf::JGE64_IMM => jge64_imm(insn),
        ebpf::JGE64_REG => jge64_reg(insn),
        ebpf::JLT64_IMM => jlt64_imm(insn),
        ebpf::JLT64_REG => jlt64_reg(insn),
        ebpf::JLE64_IMM => jle64_imm(insn),
        ebpf::JLE64_REG => jle64_reg(insn),
        ebpf::JSET64_IMM => jset64_imm(insn),
        ebpf::JSET64_REG => jset64_reg(insn),
        ebpf::JNE64_IMM => jne64_imm(insn),
        ebpf::JNE64_REG => jne64_reg(insn),
        ebpf::JSGT64_IMM => jsgt64_imm(insn),
        ebpf::JSGT64_REG => jsgt64_reg(insn),
        ebpf::JSGE64_IMM => jsge64_imm(insn),
        ebpf::JSGE64_REG => jsge64_reg(insn),
        ebpf::JSLT64_IMM => jslt64_imm(insn),
        ebpf::JSLT64_REG => jslt64_reg(insn),
        ebpf::JSLE64_IMM => jsle64_imm(insn),
        ebpf::JSLE64_REG => jsle64_reg(insn),

        // Instruction not recognized or unsupported
        _ => return None,
    };

    Some(result)
}

fn add32_imm(insn: &Insn, sbpf_version: SBPFVersion) -> String {
    format!(
        "r{d} += {i}    ///  r{d} = (r{d} as u32).wrapping_add({i}) as {}",
        if sbpf_version < SBPFVersion::V2 {
            "i32 as i64 as u64"
        } else {
            "u64"
        },
        d = insn.dst,
        i = insn.imm
    )
}

fn add32_reg(insn: &Insn, sbpf_version: SBPFVersion) -> String {
    format!(
        "r{d} += r{s}   ///  r{d} = (r{d} as u32).wrapping_add(r{s} as u32) as {}",
        if sbpf_version < SBPFVersion::V2 {
            "i32 as i64 as u64"
        } else {
            "u64"
        },
        d = insn.dst,
        s = insn.src
    )
}

fn sub32_imm(insn: &Insn, sbpf_version: SBPFVersion) -> String {
    if sbpf_version < SBPFVersion::V2 {
        // V0 and V1: dst = dst - imm (normal subtraction order)
        // Sign-extends: negative results set upper 32 bits to 0xFFFFFFFF
        format!(
            "r{d} = r{d} - {i}   ///  r{d} = (r{d} as i32).wrapping_sub({i}) as i64 as u64",
            d = insn.dst,
            i = insn.imm
        )
    } else {
        // V2+: dst = imm - dst (operands swapped per SIMD-0174)
        // Zero-extends: upper 32 bits always 0
        format!(
            "r{d} = {i} - r{d}   ///  r{d} = ({i} as i32).wrapping_sub(r{d} as i32) as u32 as u64",
            d = insn.dst,
            i = insn.imm
        )
    }
}

fn sub32_reg(insn: &Insn, sbpf_version: SBPFVersion) -> String {
    format!(
        "r{d} -= r{s}   ///  r{d} = (r{d} as u32).wrapping_sub(r{s} as u32) as {}",
        if sbpf_version < SBPFVersion::V2 {
            "i32 as i64 as u64"
        } else {
            "u64"
        },
        d = insn.dst,
        s = insn.src
    )
}

fn mul32_imm(insn: &Insn) -> String {
    format!(
        "r{d} *= {i}   ///  r{d} = (r{d} as i32).wrapping_mul({i} as i32) as i64 as u64",
        d = insn.dst,
        i = insn.imm
    )
}

fn mul32_reg(insn: &Insn) -> String {
    format!(
        "r{d} *= r{s}   ///  r{d} = (r{d} as i32).wrapping_mul(r{s} as i32) as i64 as u64",
        d = insn.dst,
        s = insn.src
    )
}

fn div32_imm(insn: &Insn) -> String {
    format!(
        "r{d} /= {i}   ///  r{d} = ((r{d} as u32) / {i}) as u64",
        d = insn.dst,
        i = insn.imm
    )
}

fn div32_reg(insn: &Insn) -> String {
    format!(
        "r{d} /= r{s}   ///  r{d} = ((r{d} as u32) / (r{s} as u32)) as u64",
        d = insn.dst,
        s = insn.src
    )
}

fn mod32_imm(insn: &Insn) -> String {
    format!(
        "r{d} %= {i}   ///  r{d} = ((r{d} as u32) % {i}) as u64",
        d = insn.dst,
        i = insn.imm
    )
}

fn mod32_reg(insn: &Insn) -> String {
    format!(
        "r{d} %= r{s}   ///  r{d} = ((r{d} as u32) % (r{s} as u32)) as u64",
        d = insn.dst,
        s = insn.src
    )
}

fn or32_imm(insn: &Insn) -> String {
    format!(
        "r{d} |= {i}   ///  r{d} = ((r{d} as u32) | ({i} as u32)) as u64",
        d = insn.dst,
        i = insn.imm
    )
}

fn or32_reg(insn: &Insn) -> String {
    format!(
        "r{d} |= r{s}   ///  r{d} = ((r{d} as u32) | (r{s} as u32)) as u64",
        d = insn.dst,
        s = insn.src
    )
}

fn and32_imm(insn: &Insn) -> String {
    format!(
        "r{d} &= {i}   ///  r{d} = ((r{d} as u32) & ({i} as u32)) as u64",
        d = insn.dst,
        i = insn.imm
    )
}

fn and32_reg(insn: &Insn) -> String {
    format!(
        "r{d} &= r{s}   ///  r{d} = ((r{d} as u32) & (r{s} as u32)) as u64",
        d = insn.dst,
        s = insn.src
    )
}

fn xor32_imm(insn: &Insn) -> String {
    format!(
        "r{d} ^= {i}   ///  r{d} = ((r{d} as u32) ^ ({i} as u32)) as u64",
        d = insn.dst,
        i = insn.imm
    )
}

fn xor32_reg(insn: &Insn) -> String {
    format!(
        "r{d} ^= r{s}   ///  r{d} = ((r{d} as u32) ^ (r{s} as u32)) as u64",
        d = insn.dst,
        s = insn.src
    )
}

fn lsh32_imm(insn: &Insn) -> String {
    format!(
        "r{d} <<= {i}   ///  r{d} = (r{d} as u32).wrapping_shl({i}) as u64",
        d = insn.dst,
        i = insn.imm
    )
}

fn lsh32_reg(insn: &Insn) -> String {
    format!(
        "r{d} <<= r{s}   ///  r{d} = (r{d} as u32).wrapping_shl(r{s} as u32) as u64",
        d = insn.dst,
        s = insn.src
    )
}

fn rsh32_imm(insn: &Insn) -> String {
    format!(
        "r{d} >>= {i}   ///  r{d} = (r{d} as u32).wrapping_shr({i}) as u64",
        d = insn.dst,
        i = insn.imm
    )
}

fn rsh32_reg(insn: &Insn) -> String {
    format!(
        "r{d} >>= r{s}   ///  r{d} = (r{d} as u32).wrapping_shr(r{s} as u32) as u64",
        d = insn.dst,
        s = insn.src
    )
}

fn arsh32_imm(insn: &Insn) -> String {
    format!(
        "r{d} >>= {i} (signed)  ///  r{d} = (r{d} as i32).wrapping_shr({i}) as u32 as u64",
        d = insn.dst,
        i = insn.imm
    )
}

fn arsh32_reg(insn: &Insn) -> String {
    format!(
        "r{d} >>= r{s} (signed)  ///  r{d} = (r{d} as i32).wrapping_shr(r{s} as u32) as u32 as u64",
        d = insn.dst,
        s = insn.src
    )
}

fn neg32(insn: &Insn) -> String {
    format!(
        "r{d} = -r{d}   ///  r{d} = (r{d} as i32).wrapping_neg() as u32 as u64",
        d = insn.dst
    )
}

fn mov32_imm(insn: &Insn) -> String {
    format!("r{d} = {i} as u64", d = insn.dst, i = insn.imm)
}

fn mov32_reg(insn: &Insn, sbpf_version: SBPFVersion) -> String {
    format!(
        "r{d} = r{s} as {}",
        if sbpf_version < SBPFVersion::V2 {
            "u32 as u64"
        } else {
            "i32 as i64 as u64"
        },
        d = insn.dst,
        s = insn.src
    )
}

fn le(insn: &Insn) -> String {
    format!("r{d} = r{d} as u32 as u64", d = insn.dst)
}

fn be(insn: &Insn) -> String {
    format!(
        "r{d} = match {i} {{ 16 => (r{d} as u16).swap_bytes() as u64, 32 => (r{d} as u32).swap_bytes() as u64, 64 => r{d}.swap_bytes(), _ => r{d} }}",
        d = insn.dst, i = insn.imm
    )
}

fn add64_imm(insn: &Insn) -> String {
    format!(
        "r{d} += {i}   ///  r{d} = r{d}.wrapping_add({i} as i32 as i64 as u64)",
        d = insn.dst,
        i = insn.imm
    )
}

fn add64_reg(insn: &Insn) -> String {
    format!(
        "r{d} += r{s}   ///  r{d} = r{d}.wrapping_add(r{s})",
        d = insn.dst,
        s = insn.src
    )
}

fn sub64_imm(insn: &Insn, sbpf_version: SBPFVersion) -> String {
    if sbpf_version < SBPFVersion::V2 {
        // V0 and V1: dst = dst - imm (normal subtraction order)
        format!(
            "r{d} -= {i}   ///  r{d} = r{d}.wrapping_sub({i} as i32 as i64 as u64)",
            d = insn.dst,
            i = insn.imm
        )
    } else {
        // V2+: dst = imm - dst (operands swapped per SIMD-0174)
        format!(
            "r{d} = {i} - r{d}   ///  r{d} = ({i} as i32 as i64 as u64).wrapping_sub(r{d})",
            d = insn.dst,
            i = insn.imm
        )
    }
}

fn sub64_reg(insn: &Insn) -> String {
    format!(
        "r{d} -= r{s}   ///  r{d} = r{d}.wrapping_sub(r{s})",
        d = insn.dst,
        s = insn.src
    )
}

fn mul64_imm(insn: &Insn) -> String {
    format!(
        "r{d} *= {i}   ///  r{d} = r{d}.wrapping_mul({i} as u64)",
        d = insn.dst,
        i = insn.imm
    )
}

fn mul64_reg(insn: &Insn) -> String {
    format!(
        "r{d} *= r{s}   ///  r{d} = r{d}.wrapping_mul(r{s})",
        d = insn.dst,
        s = insn.src
    )
}

fn div64_imm(insn: &Insn) -> String {
    format!(
        "r{d} /= {i}   ///  r{d} = r{d} / ({i} as u64)",
        d = insn.dst,
        i = insn.imm
    )
}

fn div64_reg(insn: &Insn) -> String {
    format!(
        "r{d} /= r{s}   ///  r{d} = r{d} / r{s}",
        d = insn.dst,
        s = insn.src
    )
}

fn mod64_imm(insn: &Insn) -> String {
    format!(
        "r{d} %= {i}   ///  r{d} = r{d} % ({i} as u64)",
        d = insn.dst,
        i = insn.imm
    )
}

fn mod64_reg(insn: &Insn) -> String {
    format!(
        "r{d} %= r{s}   ///  r{d} = r{d} % r{s}",
        d = insn.dst,
        s = insn.src
    )
}

fn or64_imm(insn: &Insn) -> String {
    format!(
        "r{d} |= {i}   ///  r{d} = r{d} | ({i} as i32 as i64 as u64)",
        d = insn.dst,
        i = insn.imm
    )
}

fn or64_reg(insn: &Insn) -> String {
    format!(
        "r{d} |= r{s}   ///  r{d} = r{d} | r{s}",
        d = insn.dst,
        s = insn.src
    )
}

fn and64_imm(insn: &Insn) -> String {
    format!(
        "r{d} &= {i}   ///  r{d} = r{d} & ({i} as i32 as i64 as u64)",
        d = insn.dst,
        i = insn.imm
    )
}

fn and64_reg(insn: &Insn) -> String {
    format!(
        "r{d} &= r{s}   ///  r{d} = r{d} & r{s}",
        d = insn.dst,
        s = insn.src
    )
}

fn xor64_imm(insn: &Insn) -> String {
    format!(
        "r{d} ^= {i}   ///  r{d} = r{d} ^ ({i} as i32 as i64 as u64)",
        d = insn.dst,
        i = insn.imm
    )
}

fn xor64_reg(insn: &Insn) -> String {
    format!(
        "r{d} ^= r{s}   ///  r{d} = r{d} ^ r{s}",
        d = insn.dst,
        s = insn.src
    )
}

fn lsh64_imm(insn: &Insn) -> String {
    format!(
        "r{d} <<= {i}   ///  r{d} = r{d}.wrapping_shl({i})",
        d = insn.dst,
        i = insn.imm
    )
}

fn lsh64_reg(insn: &Insn) -> String {
    format!(
        "r{d} <<= r{s}   ///  r{d} = r{d}.wrapping_shl(r{s} as u32)",
        d = insn.dst,
        s = insn.src
    )
}

fn rsh64_imm(insn: &Insn) -> String {
    format!(
        "r{d} >>= {i}   ///  r{d} = r{d}.wrapping_shr({i})",
        d = insn.dst,
        i = insn.imm
    )
}

fn rsh64_reg(insn: &Insn) -> String {
    format!(
        "r{d} >>= r{s}   ///  r{d} = r{d}.wrapping_shr(r{s} as u32)",
        d = insn.dst,
        s = insn.src
    )
}

fn arsh64_imm(insn: &Insn) -> String {
    format!(
        "r{d} >>= {i} (signed)   ///  r{d} = (r{d} as i64).wrapping_shr({i})",
        d = insn.dst,
        i = insn.imm
    )
}

fn arsh64_reg(insn: &Insn) -> String {
    format!(
        "r{d} >>= r{s} (signed)  ///  r{d} = (r{d} as i64).wrapping_shr(r{s} as u32)",
        d = insn.dst,
        s = insn.src
    )
}

fn neg64(insn: &Insn) -> String {
    format!(
        "r{d} = -r{d}   ///  r{d} = (r{d} as i64).wrapping_neg() as u64",
        d = insn.dst
    )
}

fn mov64_imm(insn: &Insn) -> String {
    format!("r{d} = {i} as u64", d = insn.dst, i = insn.imm)
}

fn mov64_reg(insn: &Insn) -> String {
    format!("r{d} = r{s}", d = insn.dst, s = insn.src)
}

fn hor64_imm(insn: &Insn) -> String {
    format!(
        "r{d} |= ({i} as u64) << 32   ///  r{d} = r{d} | ({i} as u64).wrapping_shl(32)",
        d = insn.dst,
        i = insn.imm
    )
}

fn uhmul64_imm(insn: &Insn) -> String {
    format!(
        "r{d} = (r{d} * {i}) >> 64   ///  r{d} = (r{d} as u128).wrapping_mul({i} as u32 as u128).wrapping_shr(64) as u64",
        d = insn.dst, i = insn.imm
    )
}

fn uhmul64_reg(insn: &Insn) -> String {
    format!(
        "r{d} = (r{d} * r{s}) >> 64   ///  r{d} = (r{d} as u128).wrapping_mul(r{s} as u128).wrapping_shr(64) as u64",
        d = insn.dst, s = insn.src
    )
}

fn udiv32_imm(insn: &Insn) -> String {
    format!(
        "r{d} = ((r{d} as u32) / {i}) as u64",
        d = insn.dst,
        i = insn.imm
    )
}

fn udiv32_reg(insn: &Insn) -> String {
    format!(
        "r{d} = ((r{d} as u32) / (r{s} as u32)) as u64",
        d = insn.dst,
        s = insn.src
    )
}

fn udiv64_imm(insn: &Insn) -> String {
    format!(
        "r{d} /= {i}   ///  r{d} = r{d} / ({i} as u32 as u64)",
        d = insn.dst,
        i = insn.imm
    )
}

fn udiv64_reg(insn: &Insn) -> String {
    format!("r{d} = r{d} / r{s}", d = insn.dst, s = insn.src)
}

fn urem32_imm(insn: &Insn) -> String {
    format!(
        "r{d} = ((r{d} as u32) % {i}) as u64",
        d = insn.dst,
        i = insn.imm
    )
}

fn urem32_reg(insn: &Insn) -> String {
    format!(
        "r{d} = ((r{d} as u32) % (r{s} as u32)) as u64",
        d = insn.dst,
        s = insn.src
    )
}

fn urem64_imm(insn: &Insn) -> String {
    format!(
        "r{d} %= {i}   ///  r{d} = r{d} % ({i} as u32 as u64)",
        d = insn.dst,
        i = insn.imm
    )
}

fn urem64_reg(insn: &Insn) -> String {
    format!("r{d} = r{d} % r{s}", d = insn.dst, s = insn.src)
}

fn lmul32_imm(insn: &Insn) -> String {
    format!(
        "r{d} = (r{d} * {i}) as u32   ///  r{d} = (r{d} as i32).wrapping_mul({i} as i32) as u32 as u64",
        d = insn.dst, i = insn.imm
    )
}

fn lmul32_reg(insn: &Insn) -> String {
    format!(
        "r{d} = (r{d} * r{s}) as u32   ///  r{d} = (r{d} as i32).wrapping_mul(r{s} as i32) as u32 as u64",
        d = insn.dst, s = insn.src
    )
}

fn lmul64_imm(insn: &Insn) -> String {
    format!(
        "r{d} = (r{d} * {i}) as u64   ///  r{d} = r{d}.wrapping_mul({i} as u64)",
        d = insn.dst,
        i = insn.imm
    )
}

fn lmul64_reg(insn: &Insn) -> String {
    format!(
        "r{d} = (r{d} * r{s}) as u64   ///  r{d} = r{d}.wrapping_mul(r{s})",
        d = insn.dst,
        s = insn.src
    )
}

fn shmul64_imm(insn: &Insn) -> String {
    format!(
        "r{d} = (r{d} * {i}) >> 64   ///  r{d} = (r{d} as i64 as i128).wrapping_mul({i} as i128).wrapping_shr(64) as u64",
        d = insn.dst, i = insn.imm
    )
}

fn shmul64_reg(insn: &Insn) -> String {
    format!(
        "r{d} = (r{d} * r{s}) >> 64   ///  r{d} = (r{d} as i128).wrapping_mul(r{s} as i64 as i128).wrapping_shr(64) as i64 as u64",
        d = insn.dst, s = insn.src
    )
}

fn sdiv32_imm(insn: &Insn) -> String {
    format!(
        "r{d} = ((r{d} as i32) / ({i} as i32)) as u32 as u64",
        d = insn.dst,
        i = insn.imm
    )
}

fn sdiv32_reg(insn: &Insn) -> String {
    format!(
        "r{d} = ((r{d} as i32) / (r{s} as i32)) as u32 as u64",
        d = insn.dst,
        s = insn.src
    )
}

fn sdiv64_imm(insn: &Insn) -> String {
    format!(
        "r{d} = ((r{d} as i64) / ({i} as i64)) as u64",
        d = insn.dst,
        i = insn.imm
    )
}

fn sdiv64_reg(insn: &Insn) -> String {
    format!(
        "r{d} = ((r{d} as i64) / (r{s} as i64)) as u64",
        d = insn.dst,
        s = insn.src
    )
}

fn srem32_imm(insn: &Insn) -> String {
    format!(
        "r{d} = ((r{d} as i32) % ({i} as i32)) as u32 as u64",
        d = insn.dst,
        i = insn.imm
    )
}

fn srem32_reg(insn: &Insn) -> String {
    format!(
        "r{d} = ((r{d} as i32) % (r{s} as i32)) as u32 as u64",
        d = insn.dst,
        s = insn.src
    )
}

fn srem64_imm(insn: &Insn) -> String {
    format!(
        "r{d} = ((r{d} as i64) % ({i} as i64)) as u64",
        d = insn.dst,
        i = insn.imm
    )
}

fn srem64_reg(insn: &Insn) -> String {
    format!(
        "r{d} = ((r{d} as i64) % (r{s} as i64)) as u64",
        d = insn.dst,
        s = insn.src
    )
}

fn ld_dw_imm(insn: &Insn, sbpf_version: SBPFVersion) -> String {
    if sbpf_version < SBPFVersion::V2 {
        format!("r{d} load str located at {i}", d = insn.dst, i = insn.imm)
    } else {
        format!("r{d} = {i} as u64", d = insn.dst, i = insn.imm)
    }
}

fn ja(insn: &Insn) -> String {
    format!("if true {{ pc += {} }}", insn.off)
}

// === 32-bit Jump Instructions ===

fn jeq32_imm(insn: &Insn) -> String {
    format!(
        "if (r{d} as u32) == ({i} as u32) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    )
}

fn jeq32_reg(insn: &Insn) -> String {
    format!(
        "if (r{d} as u32) == (r{s} as u32) {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    )
}

fn jgt32_imm(insn: &Insn) -> String {
    format!(
        "if (r{d} as u32) > ({i} as u32) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    )
}

fn jgt32_reg(insn: &Insn) -> String {
    format!(
        "if (r{d} as u32) > (r{s} as u32) {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    )
}

fn jge32_imm(insn: &Insn) -> String {
    format!(
        "if (r{d} as u32) >= ({i} as u32) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    )
}

fn jge32_reg(insn: &Insn) -> String {
    format!(
        "if (r{d} as u32) >= (r{s} as u32) {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    )
}

fn jlt32_imm(insn: &Insn) -> String {
    format!(
        "if (r{d} as u32) < ({i} as u32) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    )
}

fn jlt32_reg(insn: &Insn) -> String {
    format!(
        "if (r{d} as u32) < (r{s} as u32) {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    )
}

fn jle32_imm(insn: &Insn) -> String {
    format!(
        "if (r{d} as u32) <= ({i} as u32) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    )
}

fn jle32_reg(insn: &Insn) -> String {
    format!(
        "if (r{d} as u32) <= (r{s} as u32) {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    )
}

fn jset32_imm(insn: &Insn) -> String {
    format!(
        "if (r{d} as u32) & ({i} as u32) != 0 {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    )
}

fn jset32_reg(insn: &Insn) -> String {
    format!(
        "if (r{d} as u32) & (r{s} as u32) != 0 {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    )
}

fn jne32_imm(insn: &Insn) -> String {
    format!(
        "if (r{d} as u32) != ({i} as u32) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    )
}

fn jne32_reg(insn: &Insn) -> String {
    format!(
        "if (r{d} as u32) != (r{s} as u32) {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    )
}

fn jsgt32_imm(insn: &Insn) -> String {
    format!(
        "if (r{d} as i32) > ({i} as i32) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    )
}

fn jsgt32_reg(insn: &Insn) -> String {
    format!(
        "if (r{d} as i32) > (r{s} as i32) {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    )
}

fn jsge32_imm(insn: &Insn) -> String {
    format!(
        "if (r{d} as i32) >= ({i} as i32) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    )
}

fn jsge32_reg(insn: &Insn) -> String {
    format!(
        "if (r{d} as i32) >= (r{s} as i32) {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    )
}

fn jslt32_imm(insn: &Insn) -> String {
    format!(
        "if (r{d} as i32) < ({i} as i32) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    )
}

fn jslt32_reg(insn: &Insn) -> String {
    format!(
        "if (r{d} as i32) < (r{s} as i32) {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    )
}

fn jsle32_imm(insn: &Insn) -> String {
    format!(
        "if (r{d} as i32) <= ({i} as i32) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    )
}

fn jsle32_reg(insn: &Insn) -> String {
    format!(
        "if (r{d} as i32) <= (r{s} as i32) {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    )
}

// === 64-bit Jump Instructions ===

fn jeq64_imm(insn: &Insn) -> String {
    format!(
        "if r{d} == ({i} as i32 as i64 as u64) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    )
}

fn jeq64_reg(insn: &Insn) -> String {
    format!(
        "if r{d} == r{s} {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    )
}

fn jgt64_imm(insn: &Insn) -> String {
    format!(
        "if r{d} > ({i} as i32 as i64 as u64) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    )
}

fn jgt64_reg(insn: &Insn) -> String {
    format!(
        "if r{d} > r{s} {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    )
}

fn jge64_imm(insn: &Insn) -> String {
    format!(
        "if r{d} >= ({i} as i32 as i64 as u64) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    )
}

fn jge64_reg(insn: &Insn) -> String {
    format!(
        "if r{d} >= r{s} {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    )
}

fn jlt64_imm(insn: &Insn) -> String {
    format!(
        "if r{d} < ({i} as i32 as i64 as u64) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    )
}

fn jlt64_reg(insn: &Insn) -> String {
    format!(
        "if r{d} < r{s} {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    )
}

fn jle64_imm(insn: &Insn) -> String {
    format!(
        "if r{d} <= ({i} as i32 as i64 as u64) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    )
}

fn jle64_reg(insn: &Insn) -> String {
    format!(
        "if r{d} <= r{s} {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    )
}

fn jset64_imm(insn: &Insn) -> String {
    format!(
        "if r{d} & ({i} as i32 as i64 as u64) != 0 {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    )
}

fn jset64_reg(insn: &Insn) -> String {
    format!(
        "if r{d} & r{s} != 0 {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    )
}

fn jne64_imm(insn: &Insn) -> String {
    format!(
        "if r{d} != ({i} as i32 as i64 as u64) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    )
}

fn jne64_reg(insn: &Insn) -> String {
    format!(
        "if r{d} != r{s} {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    )
}

fn jsgt64_imm(insn: &Insn) -> String {
    format!(
        "if (r{d} as i64) > ({i} as i32 as i64) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    )
}

fn jsgt64_reg(insn: &Insn) -> String {
    format!(
        "if (r{d} as i64) > (r{s} as i64) {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    )
}

fn jsge64_imm(insn: &Insn) -> String {
    format!(
        "if (r{d} as i64) >= ({i} as i32 as i64) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    )
}

fn jsge64_reg(insn: &Insn) -> String {
    format!(
        "if (r{d} as i64) >= (r{s} as i64) {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    )
}

fn jslt64_imm(insn: &Insn) -> String {
    format!(
        "if (r{d} as i64) < ({i} as i32 as i64) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    )
}

fn jslt64_reg(insn: &Insn) -> String {
    format!(
        "if (r{d} as i64) < (r{s} as i64) {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    )
}

fn jsle64_imm(insn: &Insn) -> String {
    format!(
        "if (r{d} as i64) <= ({i} as i32 as i64) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    )
}

fn jsle64_reg(insn: &Insn) -> String {
    format!(
        "if (r{d} as i64) <= (r{s} as i64) {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    )
}
