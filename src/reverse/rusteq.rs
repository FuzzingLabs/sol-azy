use solana_sbpf::ebpf::{self, Insn};
use solana_sbpf::program::SBPFVersion;

macro_rules! rust_expr {
    ($expr:expr) => {
        Some($expr.to_string())
    };
}

/// Return the Rust equivalent of an eBPF instruction if available
pub fn translate_to_rust(insn: &ebpf::Insn, sbpf_version: SBPFVersion) -> Option<String> {
    match insn.opc {
        // === Memory Load or 32 bit Arithmetic and Logic ===
        ebpf::ADD32_IMM => add32_imm(insn, sbpf_version),
        ebpf::ADD32_REG => add32_reg(insn, sbpf_version),
        ebpf::SUB32_IMM => sub32_imm(insn, sbpf_version),
        ebpf::SUB32_REG => sub32_reg(insn, sbpf_version),
        ebpf::MUL32_IMM => mul32_imm(insn, sbpf_version),
        ebpf::MUL32_REG => mul32_reg(insn, sbpf_version),
        ebpf::DIV32_IMM => div32_imm(insn, sbpf_version),
        ebpf::DIV32_REG => div32_reg(insn, sbpf_version),
        ebpf::MOD32_IMM => mod32_imm(insn, sbpf_version),
        ebpf::MOD32_REG => mod32_reg(insn, sbpf_version),
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
        ebpf::NEG32 => neg32(insn, sbpf_version),
        ebpf::MOV32_IMM => mov32_imm(insn),
        ebpf::MOV32_REG => mov32_reg(insn, sbpf_version),
        ebpf::LE => le(insn, sbpf_version),
        ebpf::BE => be(insn),

        // === Memory Store or 64 bit Arithmetic and Logic ===
        ebpf::ADD64_IMM => add64_imm(insn),
        ebpf::ADD64_REG => add64_reg(insn),
        ebpf::SUB64_IMM => sub64_imm(insn, sbpf_version),
        ebpf::SUB64_REG => sub64_reg(insn),
        ebpf::MUL64_IMM => mul64_imm(insn, sbpf_version),
        ebpf::MUL64_REG => mul64_reg(insn, sbpf_version),
        ebpf::DIV64_IMM => div64_imm(insn, sbpf_version),
        ebpf::DIV64_REG => div64_reg(insn, sbpf_version),
        ebpf::MOD64_IMM => mod64_imm(insn, sbpf_version),
        ebpf::MOD64_REG => mod64_reg(insn, sbpf_version),

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
        ebpf::NEG64 => neg64(insn, sbpf_version),
        ebpf::MOV64_IMM => mov64_imm(insn),
        ebpf::MOV64_REG => mov64_reg(insn),

        // SBPF v2+ Arithmetic
        ebpf::HOR64_IMM => hor64_imm(insn, sbpf_version),
        ebpf::UHMUL64_IMM => uhmul64_imm(insn, sbpf_version),
        ebpf::UHMUL64_REG => uhmul64_reg(insn, sbpf_version),
        ebpf::UDIV32_IMM => udiv32_imm(insn, sbpf_version),
        ebpf::UDIV32_REG => udiv32_reg(insn, sbpf_version),
        ebpf::UDIV64_IMM => udiv64_imm(insn, sbpf_version),
        ebpf::UDIV64_REG => udiv64_reg(insn, sbpf_version),
        ebpf::UREM32_IMM => urem32_imm(insn, sbpf_version),
        ebpf::UREM32_REG => urem32_reg(insn, sbpf_version),
        ebpf::UREM64_IMM => urem64_imm(insn, sbpf_version),
        ebpf::UREM64_REG => urem64_reg(insn, sbpf_version),
        ebpf::LMUL32_IMM => lmul32_imm(insn, sbpf_version),
        ebpf::LMUL32_REG => lmul32_reg(insn, sbpf_version),
        ebpf::LMUL64_IMM => lmul64_imm(insn, sbpf_version),
        ebpf::LMUL64_REG => lmul64_reg(insn, sbpf_version),
        ebpf::SHMUL64_IMM => shmul64_imm(insn, sbpf_version),
        ebpf::SHMUL64_REG => shmul64_reg(insn, sbpf_version),
        ebpf::SDIV32_IMM => sdiv32_imm(insn, sbpf_version),
        ebpf::SDIV32_REG => sdiv32_reg(insn, sbpf_version),
        ebpf::SDIV64_IMM => sdiv64_imm(insn, sbpf_version),
        ebpf::SDIV64_REG => sdiv64_reg(insn, sbpf_version),
        ebpf::SREM32_IMM => srem32_imm(insn, sbpf_version),
        ebpf::SREM32_REG => srem32_reg(insn, sbpf_version),
        ebpf::SREM64_IMM => srem64_imm(insn, sbpf_version),
        ebpf::SREM64_REG => srem64_reg(insn, sbpf_version),

        // === BPF_LD_IMM (until v2) ===
        ebpf::LD_DW_IMM => ld_dw_imm(insn, sbpf_version),

        // === BPF_JMP class ===
        ebpf::JA => ja(insn),
        ebpf::JEQ_IMM => jeq_imm(insn),
        ebpf::JEQ_REG => jeq_reg(insn),
        ebpf::JGT_IMM => jgt_imm(insn),
        ebpf::JGT_REG => jgt_reg(insn),
        ebpf::JGE_IMM => jge_imm(insn),
        ebpf::JGE_REG => jge_reg(insn),
        ebpf::JSET_IMM => jset_imm(insn),
        ebpf::JSET_REG => jset_reg(insn),
        ebpf::JNE_IMM => jne_imm(insn),
        ebpf::JNE_REG => jne_reg(insn),
        ebpf::JSGT_IMM => jsgt_imm(insn),
        ebpf::JSGT_REG => jsgt_reg(insn),
        ebpf::JSGE_IMM => jsge_imm(insn),
        ebpf::JSGE_REG => jsge_reg(insn),
        ebpf::JLT_IMM => jlt_imm(insn),
        ebpf::JLT_REG => jlt_reg(insn),
        ebpf::JLE_IMM => jle_imm(insn),
        ebpf::JLE_REG => jle_reg(insn),
        ebpf::JSLT_IMM => jslt_imm(insn),
        ebpf::JSLT_REG => jslt_reg(insn),
        ebpf::JSLE_IMM => jsle_imm(insn),
        ebpf::JSLE_REG => jsle_reg(insn),

        _ => None, // Unknown or unsupported instruction
    }
}

fn add32_imm(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    rust_expr!(format!(
        "r{d} += {i}    ///  r{d} = (r{d} as u32).wrapping_add({i}) as {}",
        if sbpf_version < SBPFVersion::V2 {
            "i32 as i64 as u64"
        } else {
            "u64"
        },
        d = insn.dst,
        i = insn.imm
    ))
}

fn add32_reg(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    rust_expr!(format!(
        "r{d} += r{s}   ///  r{d} = (r{d} as u32).wrapping_add(r{s} as u32) as {}",
        if sbpf_version < SBPFVersion::V2 {
            "i32 as i64 as u64"
        } else {
            "u64"
        },
        d = insn.dst,
        s = insn.src
    ))
}

fn sub32_imm(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    if sbpf_version < SBPFVersion::V2 {
        rust_expr!(format!(
            "r{d} = {i} - r{d}   ///  r{d} = (r{d} as u32).wrapping_sub({i}) as u64",
            d = insn.dst,
            i = insn.imm
        ))
    } else {
        rust_expr!(format!(
            "r{d} = {i} - r{d}   ///  r{d} = {i}.wrapping_sub(r{d} as u32) as u64",
            d = insn.dst,
            i = insn.imm
        ))
    }
}

fn sub32_reg(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    rust_expr!(format!(
        "r{d} -= r{s}   ///  r{d} = (r{d} as u32).wrapping_sub(r{s} as u32) as {}",
        if sbpf_version < SBPFVersion::V2 {
            "i32 as i64 as u64"
        } else {
            "u64"
        },
        d = insn.dst,
        s = insn.src
    ))
}

fn mul32_imm(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    if sbpf_version < SBPFVersion::V2 {
        rust_expr!(format!(
            "r{d} *= {i}   ///  r{d} = (r{d} as i32).wrapping_mul({i} as i32) as i64 as u64",
            d = insn.dst,
            i = insn.imm
        ))
    } else {
        None
    }
}

fn mul32_reg(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    if sbpf_version < SBPFVersion::V2 {
        rust_expr!(format!(
            "r{d} *= r{s}   ///  r{d} = (r{d} as i32).wrapping_mul(r{s} as i32) as i64 as u64",
            d = insn.dst,
            s = insn.src
        ))
    } else {
        None
    }
}

fn div32_imm(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    if sbpf_version < SBPFVersion::V2 {
        rust_expr!(format!(
            "r{d} /= {i}   ///  r{d} = ((r{d} as u32) / {i}) as u64",
            d = insn.dst,
            i = insn.imm
        ))
    } else {
        None
    }
}

fn div32_reg(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    if sbpf_version < SBPFVersion::V2 {
        rust_expr!(format!(
            "r{d} /= r{s}   ///  r{d} = ((r{d} as u32) / (r{s} as u32)) as u64",
            d = insn.dst,
            s = insn.src
        ))
    } else {
        None
    }
}

fn mod32_imm(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    if sbpf_version < SBPFVersion::V2 {
        rust_expr!(format!(
            "r{d} %= {i}   ///  r{d} = ((r{d} as u32) % {i}) as u64",
            d = insn.dst,
            i = insn.imm
        ))
    } else {
        None
    }
}

fn mod32_reg(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    if sbpf_version < SBPFVersion::V2 {
        rust_expr!(format!(
            "r{d} %= r{s}   ///  r{d} = ((r{d} as u32) % (r{s} as u32)) as u64",
            d = insn.dst,
            s = insn.src
        ))
    } else {
        None
    }
}

fn or32_imm(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} |= {i}   ///  r{d} = (r{d} as u32).or({i}) as u64",
        d = insn.dst,
        i = insn.imm
    ))
}

fn or32_reg(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} |= r{s}   ///  r{d} = (r{d} as u32).or(r{s} as u32) as u64",
        d = insn.dst,
        s = insn.src
    ))
}

fn and32_imm(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} &= {i}   ///  r{d} = (r{d} as u32).and({i}) as u64",
        d = insn.dst,
        i = insn.imm
    ))
}

fn and32_reg(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} &= r{s}   ///  r{d} = (r{d} as u32).and(r{s} as u32) as u64",
        d = insn.dst,
        s = insn.src
    ))
}

fn xor32_imm(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} ^= {i}   ///  r{d} = (r{d} as u32).xor({i}) as u64",
        d = insn.dst,
        i = insn.imm
    ))
}

fn xor32_reg(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} ^= r{s}   ///  r{d} = (r{d} as u32).xor(r{s} as u32) as u64",
        d = insn.dst,
        s = insn.src
    ))
}

fn lsh32_imm(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} <<= {i}   ///  r{d} = (r{d} as u32).wrapping_shl({i}) as u64",
        d = insn.dst,
        i = insn.imm
    ))
}

fn lsh32_reg(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} <<= r{s}   ///  r{d} = (r{d} as u32).wrapping_shl(r{s} as u32) as u64",
        d = insn.dst,
        s = insn.src
    ))
}

fn rsh32_imm(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} >>= {i}   ///  r{d} = (r{d} as u32).wrapping_shr({i}) as u64",
        d = insn.dst,
        i = insn.imm
    ))
}

fn rsh32_reg(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} >>= r{s}   ///  r{d} = (r{d} as u32).wrapping_shr(r{s} as u32) as u64",
        d = insn.dst,
        s = insn.src
    ))
}

fn arsh32_imm(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} >>= {i} (signed)  ///  r{d} = (r{d} as i32).wrapping_shr({i}) as u32 as u64",
        d = insn.dst,
        i = insn.imm
    ))
}

fn arsh32_reg(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} >>= r{s} (signed)  ///  r{d} = (r{d} as i32).wrapping_shr(r{s} as u32) as u32 as u64",
        d = insn.dst,
        s = insn.src
    ))
}

fn neg32(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    if sbpf_version < SBPFVersion::V2 {
        rust_expr!(format!(
            "r{d} = -r{d}   ///  r{d} = (r{d} as i32).wrapping_neg() as u32 as u64",
            d = insn.dst
        ))
    } else {
        None
    }
}

fn mov32_imm(insn: &Insn) -> Option<String> {
    rust_expr!(format!("r{d} = {i} as u64", d = insn.dst, i = insn.imm))
}

fn mov32_reg(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    rust_expr!(format!(
        "r{d} = r{s} as {}",
        if sbpf_version < SBPFVersion::V2 {
            "u32 as u64"
        } else {
            "i32 as i64 as u64"
        },
        d = insn.dst,
        s = insn.src
    ))
}

fn le(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    if sbpf_version < SBPFVersion::V2 {
        rust_expr!(format!("r{d} = r{d} as u32 as u64", d = insn.dst))
    } else {
        None
    }
}

fn be(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} = match {i} {{ 16 => (r{d} as u16).swap_bytes() as u64, 32 => (r{d} as u32).swap_bytes() as u64, 64 => r{d}.swap_bytes(), _ => r{d} }}",
        d = insn.dst, i = insn.imm
    ))
}

fn add64_imm(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} += {i}   ///  r{d} = r{d}.wrapping_add({i} as i32 as i64 as u64)",
        d = insn.dst,
        i = insn.imm
    ))
}

fn add64_reg(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} += r{s}   ///  r{d} = r{d}.wrapping_add(r{s})",
        d = insn.dst,
        s = insn.src
    ))
}

fn sub64_imm(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    if sbpf_version < SBPFVersion::V2 {
        rust_expr!(format!(
            "r{d} -= {i}   ///  r{d} = r{d}.wrapping_sub({i} as i32 as i64 as u64)",
            d = insn.dst,
            i = insn.imm
        ))
    } else {
        rust_expr!(format!(
            "r{d} -= {i}   ///  r{d} = ({i} as i32 as i64 as u64).wrapping_sub(r{d})",
            d = insn.dst,
            i = insn.imm
        ))
    }
}

fn sub64_reg(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} -= r{s}   ///  r{d} = r{d}.wrapping_sub(r{s})",
        d = insn.dst,
        s = insn.src
    ))
}

fn mul64_imm(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    if sbpf_version < SBPFVersion::V2 {
        rust_expr!(format!(
            "r{d} *= {i}   ///  r{d} = r{d}.wrapping_mul({i} as u64)",
            d = insn.dst,
            i = insn.imm
        ))
    } else {
        None
    }
}

fn mul64_reg(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    if sbpf_version < SBPFVersion::V2 {
        rust_expr!(format!(
            "r{d} *= r{s}   ///  r{d} = r{d}.wrapping_mul(r{s})",
            d = insn.dst,
            s = insn.src
        ))
    } else {
        None
    }
}

fn div64_imm(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    if sbpf_version < SBPFVersion::V2 {
        rust_expr!(format!(
            "r{d} /= {i}   ///  r{d} = r{d} / ({i} as u64)",
            d = insn.dst,
            i = insn.imm
        ))
    } else {
        None
    }
}

fn div64_reg(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    if sbpf_version < SBPFVersion::V2 {
        rust_expr!(format!(
            "r{d} /= r{s}   ///  r{d} = r{d} / r{s}",
            d = insn.dst,
            s = insn.src
        ))
    } else {
        None
    }
}

fn mod64_imm(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    if sbpf_version < SBPFVersion::V2 {
        rust_expr!(format!(
            "r{d} %= {i}   ///  r{d} = r{d} % ({i} as u64)",
            d = insn.dst,
            i = insn.imm
        ))
    } else {
        None
    }
}

fn mod64_reg(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    if sbpf_version < SBPFVersion::V2 {
        rust_expr!(format!(
            "r{d} %= r{s}   ///  r{d} = r{d} % r{s}",
            d = insn.dst,
            s = insn.src
        ))
    } else {
        None
    }
}

fn or64_imm(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} |= {i}   ///  r{d} = r{d}.or({i})",
        d = insn.dst,
        i = insn.imm
    ))
}

fn or64_reg(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} |= r{s}   ///  r{d} = r{d}.or(r{s})",
        d = insn.dst,
        s = insn.src
    ))
}

fn and64_imm(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} &= {i}   ///  r{d} = r{d}.and({i})",
        d = insn.dst,
        i = insn.imm
    ))
}

fn and64_reg(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} &= r{s}   ///  r{d} = r{d}.and(r{s})",
        d = insn.dst,
        s = insn.src
    ))
}

fn xor64_imm(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} ^= {i}   ///  r{d} = r{d}.xor({i})",
        d = insn.dst,
        i = insn.imm
    ))
}

fn xor64_reg(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} ^= r{s}   ///  r{d} = r{d}.xor(r{s})",
        d = insn.dst,
        s = insn.src
    ))
}

fn lsh64_imm(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} <<= {i}   ///  r{d} = r{d}.wrapping_shl({i})",
        d = insn.dst,
        i = insn.imm
    ))
}

fn lsh64_reg(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} <<= r{s}   ///  r{d} = r{d}.wrapping_shl(r{s} as u32)",
        d = insn.dst,
        s = insn.src
    ))
}

fn rsh64_imm(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} >>= {i}   ///  r{d} = r{d}.wrapping_shr({i})",
        d = insn.dst,
        i = insn.imm
    ))
}

fn rsh64_reg(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} >>= r{s}   ///  r{d} = r{d}.wrapping_shr(r{s} as u32)",
        d = insn.dst,
        s = insn.src
    ))
}

fn arsh64_imm(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} >>= {i} (signed)   ///  r{d} = (r{d} as i64).wrapping_shr({i})",
        d = insn.dst,
        i = insn.imm
    ))
}

fn arsh64_reg(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} >>= r{s} (signed)  ///  r{d} = (r{d} as i64).wrapping_shr(r{s} as u32)",
        d = insn.dst,
        s = insn.src
    ))
}

fn neg64(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    if sbpf_version < SBPFVersion::V2 {
        rust_expr!(format!(
            "r{d} = -r{d}   ///  r{d} = (r{d} as i64).wrapping_neg() as u64",
            d = insn.dst
        ))
    } else {
        None
    }
}

fn mov64_imm(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "r{d} = {i} as i32 as i64 as u64",
        d = insn.dst,
        i = insn.imm
    ))
}

fn mov64_reg(insn: &Insn) -> Option<String> {
    rust_expr!(format!("r{d} = r{s}", d = insn.dst, s = insn.src))
}

fn hor64_imm(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    (sbpf_version >= SBPFVersion::V2).then(|| {
        rust_expr!(format!(
            "r{d} = r{d} | (({i} as u64) << 32)   ///  r{d} = r{d}.or(({i} as u64).wrapping_shl(32))",
            d = insn.dst, i = insn.imm
        ))
    }).flatten()
}

fn uhmul64_imm(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    (sbpf_version >= SBPFVersion::V2).then(|| {
        rust_expr!(format!(
            "r{d} = (r{d} * {i}) >> 64   ///  r{d} = (r{d} as u128).wrapping_mul({i} as u128).wrapping_shr(64) as u64",
            d = insn.dst, i = insn.imm
        ))
    }).flatten()
}

fn uhmul64_reg(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    (sbpf_version >= SBPFVersion::V2).then(|| {
        rust_expr!(format!(
            "r{d} = (r{d} * r{s}) >> 64   ///  r{d} = (r{d} as u128).wrapping_mul(r{s} as u128).wrapping_shr(64) as u64",
            d = insn.dst, s = insn.src
        ))
    }).flatten()
}

fn udiv32_imm(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    (sbpf_version >= SBPFVersion::V2)
        .then(|| {
            rust_expr!(format!(
                "r{d} = ((r{d} as u32) / {i}) as u64",
                d = insn.dst,
                i = insn.imm
            ))
        })
        .flatten()
}

fn udiv32_reg(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    (sbpf_version >= SBPFVersion::V2)
        .then(|| {
            rust_expr!(format!(
                "r{d} = ((r{d} as u32) / (r{s} as u32)) as u64",
                d = insn.dst,
                s = insn.src
            ))
        })
        .flatten()
}

fn udiv64_imm(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    (sbpf_version >= SBPFVersion::V2)
        .then(|| {
            rust_expr!(format!(
                "r{d} = r{d} / ({i} as u64)",
                d = insn.dst,
                i = insn.imm
            ))
        })
        .flatten()
}

fn udiv64_reg(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    (sbpf_version >= SBPFVersion::V2)
        .then(|| rust_expr!(format!("r{d} = r{d} / r{s}", d = insn.dst, s = insn.src)))
        .flatten()
}

fn urem32_imm(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    (sbpf_version >= SBPFVersion::V2)
        .then(|| {
            rust_expr!(format!(
                "r{d} = ((r{d} as u32) % {i}) as u64",
                d = insn.dst,
                i = insn.imm
            ))
        })
        .flatten()
}

fn urem32_reg(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    (sbpf_version >= SBPFVersion::V2)
        .then(|| {
            rust_expr!(format!(
                "r{d} = ((r{d} as u32) % (r{s} as u32)) as u64",
                d = insn.dst,
                s = insn.src
            ))
        })
        .flatten()
}

fn urem64_imm(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    (sbpf_version >= SBPFVersion::V2)
        .then(|| {
            rust_expr!(format!(
                "r{d} = r{d} % ({i} as u64)",
                d = insn.dst,
                i = insn.imm
            ))
        })
        .flatten()
}

fn urem64_reg(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    (sbpf_version >= SBPFVersion::V2)
        .then(|| rust_expr!(format!("r{d} = r{d} % r{s}", d = insn.dst, s = insn.src)))
        .flatten()
}

fn lmul32_imm(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    (sbpf_version >= SBPFVersion::V2).then(|| {
        rust_expr!(format!(
            "r{d} = (r{d} * {i}) as u32   ///  r{d} = (r{d} as i32).wrapping_mul({i} as i32) as u32 as u64",
            d = insn.dst, i = insn.imm
        ))
    }).flatten()
}

fn lmul32_reg(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    (sbpf_version >= SBPFVersion::V2).then(|| {
        rust_expr!(format!(
            "r{d} = (r{d} * r{s}) as u32   ///  r{d} = (r{d} as i32).wrapping_mul(r{s} as i32) as u32 as u64",
            d = insn.dst, s = insn.src
        ))
    }).flatten()
}

fn lmul64_imm(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    (sbpf_version >= SBPFVersion::V2)
        .then(|| {
            rust_expr!(format!(
                "r{d} = (r{d} * {i}) as u64   ///  r{d} = r{d}.wrapping_mul({i} as u64)",
                d = insn.dst,
                i = insn.imm
            ))
        })
        .flatten()
}

fn lmul64_reg(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    (sbpf_version >= SBPFVersion::V2)
        .then(|| {
            rust_expr!(format!(
                "r{d} = (r{d} * r{s}) as u64   ///  r{d} = r{d}.wrapping_mul(r{s})",
                d = insn.dst,
                s = insn.src
            ))
        })
        .flatten()
}

fn shmul64_imm(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    (sbpf_version >= SBPFVersion::V2).then(|| {
        rust_expr!(format!(
            "r{d} = (r{d} * {i}) >> 64   ///  r{d} = (r{d} as i128).wrapping_mul({i} as i32 as i128).wrapping_shr(64) as i64 as u64",
            d = insn.dst, i = insn.imm
        ))
    }).flatten()
}

fn shmul64_reg(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    (sbpf_version >= SBPFVersion::V2).then(|| {
        rust_expr!(format!(
            "r{d} = (r{d} * r{s}) >> 64   ///  r{d} = (r{d} as i128).wrapping_mul(r{s} as i64 as i128).wrapping_shr(64) as i64 as u64",
            d = insn.dst, s = insn.src
        ))
    }).flatten()
}

fn sdiv32_imm(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    (sbpf_version >= SBPFVersion::V2)
        .then(|| {
            rust_expr!(format!(
                "r{d} = ((r{d} as i32) / ({i} as i32)) as u32 as u64",
                d = insn.dst,
                i = insn.imm
            ))
        })
        .flatten()
}

fn sdiv32_reg(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    (sbpf_version >= SBPFVersion::V2)
        .then(|| {
            rust_expr!(format!(
                "r{d} = ((r{d} as i32) / (r{s} as i32)) as u32 as u64",
                d = insn.dst,
                s = insn.src
            ))
        })
        .flatten()
}

fn sdiv64_imm(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    (sbpf_version >= SBPFVersion::V2)
        .then(|| {
            rust_expr!(format!(
                "r{d} = ((r{d} as i64) / ({i} as i64)) as u64",
                d = insn.dst,
                i = insn.imm
            ))
        })
        .flatten()
}

fn sdiv64_reg(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    (sbpf_version >= SBPFVersion::V2)
        .then(|| {
            rust_expr!(format!(
                "r{d} = ((r{d} as i64) / (r{s} as i64)) as u64",
                d = insn.dst,
                s = insn.src
            ))
        })
        .flatten()
}

fn srem32_imm(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    (sbpf_version >= SBPFVersion::V2)
        .then(|| {
            rust_expr!(format!(
                "r{d} = ((r{d} as i32) % ({i} as i32)) as u32 as u64",
                d = insn.dst,
                i = insn.imm
            ))
        })
        .flatten()
}

fn srem32_reg(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    (sbpf_version >= SBPFVersion::V2)
        .then(|| {
            rust_expr!(format!(
                "r{d} = ((r{d} as i32) % (r{s} as i32)) as u32 as u64",
                d = insn.dst,
                s = insn.src
            ))
        })
        .flatten()
}

fn srem64_imm(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    (sbpf_version >= SBPFVersion::V2)
        .then(|| {
            rust_expr!(format!(
                "r{d} = ((r{d} as i64) % ({i} as i64)) as u64",
                d = insn.dst,
                i = insn.imm
            ))
        })
        .flatten()
}

fn srem64_reg(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    (sbpf_version >= SBPFVersion::V2)
        .then(|| {
            rust_expr!(format!(
                "r{d} = ((r{d} as i64) % (r{s} as i64)) as u64",
                d = insn.dst,
                s = insn.src
            ))
        })
        .flatten()
}

fn ld_dw_imm(insn: &Insn, sbpf_version: SBPFVersion) -> Option<String> {
    if sbpf_version < SBPFVersion::V2 {
        rust_expr!(format!(
            "r{d} load str located at {i}",
            d = insn.dst,
            i = insn.imm
        ))
    } else {
        None
    }
}

fn ja(insn: &Insn) -> Option<String> {
    rust_expr!(format!("if true {{ pc += {} }}", insn.off))
}

fn jeq_imm(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "if r{d} == ({i} as i32 as i64 as u64) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    ))
}

fn jeq_reg(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "if r{d} == r{s} {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    ))
}

fn jgt_imm(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "if r{d} > ({i} as i32 as i64 as u64) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    ))
}

fn jgt_reg(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "if r{d} > r{s} {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    ))
}

fn jge_imm(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "if r{d} >= ({i} as i32 as i64 as u64) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    ))
}

fn jge_reg(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "if r{d} >= r{s} {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    ))
}

fn jset_imm(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "if r{d} & {i} {{ pc += {o} }}   ///  if r{d}.and({i} as i32 as i64 as u64) != 0 {{ pc += {o} }}",
        d = insn.dst, i = insn.imm, o = insn.off
    ))
}

fn jset_reg(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "if r{d} & r{s} {{ pc += {o} }}   ///  if r{d}.and(r{s}) != 0 {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    ))
}

fn jne_imm(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "if r{d} != ({i} as i32 as i64 as u64) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    ))
}

fn jne_reg(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "if r{d} != r{s} {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    ))
}

fn jsgt_imm(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "if (r{d} as i64) > ({i} as i32 as i64) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    ))
}

fn jsgt_reg(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "if (r{d} as i64) > (r{s} as i64) {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    ))
}

fn jsge_imm(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "if (r{d} as i64) >= ({i} as i32 as i64) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    ))
}

fn jsge_reg(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "if (r{d} as i64) >= (r{s} as i64) {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    ))
}

fn jlt_imm(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "if r{d} < ({i} as i32 as i64 as u64) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    ))
}

fn jlt_reg(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "if r{d} < r{s} {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    ))
}

fn jle_imm(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "if r{d} <= ({i} as i32 as i64 as u64) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    ))
}

fn jle_reg(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "if r{d} <= r{s} {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    ))
}

fn jslt_imm(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "if (r{d} as i64) < ({i} as i32 as i64) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    ))
}

fn jslt_reg(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "if (r{d} as i64) < (r{s} as i64) {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    ))
}

fn jsle_imm(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "if (r{d} as i64) <= ({i} as i32 as i64) {{ pc += {o} }}",
        d = insn.dst,
        i = insn.imm,
        o = insn.off
    ))
}

fn jsle_reg(insn: &Insn) -> Option<String> {
    rust_expr!(format!(
        "if (r{d} as i64) <= (r{s} as i64) {{ pc += {o} }}",
        d = insn.dst,
        s = insn.src,
        o = insn.off
    ))
}
