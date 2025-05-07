use solana_sbpf::program::SBPFVersion;
use solana_sbpf::ebpf;

/// Return the Rust equivalent of an eBPF instruction if available
pub fn translate_to_rust(insn: &ebpf::Insn, sbpf_version: SBPFVersion) -> Option<String> {

    macro_rules! rust_expr {
        ($expr:expr) => {
            Some($expr.to_string())
        };
    }

    match insn.opc {

        // === Memory Load or 32 bit Arithmetic and Logic ===

        // add32
        ebpf::ADD32_IMM if sbpf_version < SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} += {imm}    ///  r{dst} = (r{dst} as u32).wrapping_add({imm}) as i32 as i64 as u64 ",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::ADD32_IMM => {
            rust_expr!(format!(
                "r{dst} += {imm}    ///  r{dst} = (r{dst} as u32).wrapping_add({imm}) as u64",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::ADD32_REG if sbpf_version < SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} += {src}   ///  r{dst} = (r{dst} as u32).wrapping_add(r{src} as u32) as i32 as i64 as u64",
                dst = insn.dst, src = insn.src
            ))
        }
        ebpf::ADD32_REG => {
            rust_expr!(format!(
                "r{dst} += {src}   ///  r{dst} = (r{dst} as u32).wrapping_add(r{src} as u32) as u64",
                dst = insn.dst, src = insn.src
            ))
        }

        // sub32
        ebpf::SUB32_IMM if sbpf_version < SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = {imm} - {dst}   ///  r{dst} = (r{dst} as u32).wrapping_sub({imm}) as u64",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::SUB32_IMM => {
            rust_expr!(format!(
                "r{dst} = {imm} - {dst}   ///  r{dst} = {imm}.wrapping_sub(r{dst} as u32) as u64",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::SUB32_REG if sbpf_version < SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} -= {src}   ///  r{dst} = (r{dst} as u32).wrapping_sub(r{src} as u32) as i32 as i64 as u64",
                dst = insn.dst, src = insn.src
            ))
        }
        ebpf::SUB32_REG => {
            rust_expr!(format!(
                "r{dst} -= {src}   ///  r{dst} = (r{dst} as u32).wrapping_sub(r{src} as u32) as u64",
                dst = insn.dst, src = insn.src
            ))
        }

        // mul32
        ebpf::MUL32_IMM if sbpf_version < SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} *= {imm}   ///  r{dst} = (r{dst} as i32).wrapping_mul({imm} as i32) as i64 as u64",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::MUL32_REG if sbpf_version < SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} *= {src}   ///  r{dst} = (r{dst} as i32).wrapping_mul(r{src} as i32) as i64 as u64",
                dst = insn.dst, src = insn.src
            ))
        }

        // div32
        ebpf::DIV32_IMM if sbpf_version < SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} /= {imm}   ///  r{dst} = ((r{dst} as u32) / {imm}) as u64",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::DIV32_REG if sbpf_version < SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} /= {src}   ///  r{dst} = ((r{dst} as u32) / (r{src} as u32)) as u64",
                dst = insn.dst, src = insn.src
            ))
        }

        // mod32
        ebpf::MOD32_IMM if sbpf_version < SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} %= {imm}   ///  r{dst} = ((r{dst} as u32) % {imm}) as u64",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::MOD32_REG if sbpf_version < SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} %= {src}   ///  r{dst} = ((r{dst} as u32) % (r{src} as u32)) as u64",
                dst = insn.dst, src = insn.src
            ))
        }

        // or32
        ebpf::OR32_IMM => {
            rust_expr!(format!(
                "r{dst} |= {imm}   ///  r{dst} = (r{dst} as u32).or({imm}) as u64",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::OR32_REG => {
            rust_expr!(format!(
                "r{dst} |= {src}   ///  r{dst} = (r{dst} as u32).or(r{src} as u32) as u64",
                dst = insn.dst, src = insn.src
            ))
        }

        // and32
        ebpf::AND32_IMM => {
            rust_expr!(format!(
                "r{dst} &= {imm}   ///  r{dst} = (r{dst} as u32).and({imm}) as u64",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::AND32_REG => {
            rust_expr!(format!(
                "r{dst} &= {src}   ///  r{dst} = (r{dst} as u32).and(r{src} as u32) as u64",
                dst = insn.dst, src = insn.src
            ))
        }

        // xor32
        ebpf::XOR32_IMM => {
            rust_expr!(format!(
                "r{dst} ^= {imm}   ///  r{dst} = (r{dst} as u32).xor({imm}) as u64",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::XOR32_REG => {
            rust_expr!(format!(
                "r{dst} ^= {src}   ///  r{dst} = (r{dst} as u32).xor(r{src} as u32) as u64",
                dst = insn.dst, src = insn.src
            ))
        }

        // lsh32
        ebpf::LSH32_IMM => {
            rust_expr!(format!(
                "r{dst} <<= {imm}   ///  r{dst} = (r{dst} as u32).wrapping_shl({imm}) as u64",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::LSH32_REG => {
            rust_expr!(format!(
                "r{dst} <<= {src}   ///  r{dst} = (r{dst} as u32).wrapping_shl(r{src} as u32) as u64",
                dst = insn.dst, src = insn.src
            ))
        }

        // rsh32
        ebpf::RSH32_IMM => {
            rust_expr!(format!(
                "r{dst} >>= {imm}   ///  r{dst} = (r{dst} as u32).wrapping_shr({imm}) as u64",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::RSH32_REG => {
            rust_expr!(format!(
                "r{dst} >>= {src}   ///  r{dst} = (r{dst} as u32).wrapping_shr(r{src} as u32) as u64",
                dst = insn.dst, src = insn.src
            ))
        }

        // ash32 (arithmetic shift right)
        ebpf::ARSH32_IMM => {
            rust_expr!(format!(
                "r{dst} >>= {imm} (arithmetic)  ///  r{dst} = (r{dst} as i32).wrapping_shr({imm}) as u32 as u64",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::ARSH32_REG => {
            rust_expr!(format!(
                "r{dst} >>= {src} (arithmetic)  ///  r{dst} = (r{dst} as i32).wrapping_shr(r{src} as u32) as u32 as u64",
                dst = insn.dst, src = insn.src
            ))
        }

        // neg32
        ebpf::NEG32 if sbpf_version < SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = -{dst}   ///  r{dst} = (r{dst} as i32).wrapping_neg() as u32 as u64",
                dst = insn.dst
            ))
        }

        // mov32
        ebpf::MOV32_IMM => {
            rust_expr!(format!(
                "r{dst} = {imm}   ///  r{dst} = {imm} as u64",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::MOV32_REG if sbpf_version < SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = {src}   ///  r{dst} = r{src} as u32 as u64",
                dst = insn.dst, src = insn.src
            ))
        }
        ebpf::MOV32_REG => {
            rust_expr!(format!(
                "r{dst} = {src}   ///  r{dst} = r{src} as i32 as i64 as u64",
                dst = insn.dst, src = insn.src
            ))
        }

        // le / be
        ebpf::LE if sbpf_version < SBPFVersion::V2 => {
            rust_expr!(format!("r{dst} = r{dst} as u32 as u64", dst = insn.dst))
        }
        ebpf::BE => {
            rust_expr!(format!(
                "r{dst} = match {imm} {{ 16 => (r{dst} as u16).swap_bytes() as u64, 32 => (r{dst} as u32).swap_bytes() as u64, 64 => r{dst}.swap_bytes(), _ => r{dst} }}",
                dst = insn.dst, imm = insn.imm
            ))
        }

        // === Memory Store or 64 bit Arithmetic and Logic ===

        // add64
        ebpf::ADD64_IMM => {
            rust_expr!(format!(
                "r{dst} = r{dst}.wrapping_add({imm} as i32 as i64 as u64)",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::ADD64_REG => {
            rust_expr!(format!(
                "r{dst} = r{dst}.wrapping_add(r{src})",
                dst = insn.dst, src = insn.src
            ))
        }

        // sub64
        ebpf::SUB64_IMM if sbpf_version < SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = r{dst}.wrapping_sub({imm} as i32 as i64 as u64)",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::SUB64_IMM => {
            rust_expr!(format!(
                "r{dst} = ({imm} as i32 as i64 as u64).wrapping_sub(r{dst})",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::SUB64_REG => {
            rust_expr!(format!(
                "r{dst} = r{dst}.wrapping_sub(r{src})",
                dst = insn.dst, src = insn.src
            ))
        }

        // mul64
        ebpf::MUL64_IMM if sbpf_version < SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = r{dst}.wrapping_mul({imm} as u64)",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::MUL64_REG if sbpf_version < SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = r{dst}.wrapping_mul(r{src})",
                dst = insn.dst, src = insn.src
            ))
        }

        // div64
        ebpf::DIV64_IMM if sbpf_version < SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = r{dst} / ({imm} as u64)",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::DIV64_REG if sbpf_version < SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = r{dst} / r{src}",
                dst = insn.dst, src = insn.src
            ))
        }

        // mod64
        ebpf::MOD64_IMM if sbpf_version < SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = r{dst} % ({imm} as u64)",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::MOD64_REG if sbpf_version < SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = r{dst} % r{src}",
                dst = insn.dst, src = insn.src
            ))
        }

        // or64
        ebpf::OR64_IMM => {
            rust_expr!(format!(
                "r{dst} = r{dst}.or({imm})",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::OR64_REG => {
            rust_expr!(format!(
                "r{dst} = r{dst}.or(r{src})",
                dst = insn.dst, src = insn.src
            ))
        }

        // and64
        ebpf::AND64_IMM => {
            rust_expr!(format!(
                "r{dst} = r{dst}.and({imm})",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::AND64_REG => {
            rust_expr!(format!(
                "r{dst} = r{dst}.and(r{src})",
                dst = insn.dst, src = insn.src
            ))
        }

        // xor64
        ebpf::XOR64_IMM => {
            rust_expr!(format!(
                "r{dst} = r{dst}.xor({imm})",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::XOR64_REG => {
            rust_expr!(format!(
                "r{dst} = r{dst}.xor(r{src})",
                dst = insn.dst, src = insn.src
            ))
        }

        // lsh64
        ebpf::LSH64_IMM => {
            rust_expr!(format!(
                "r{dst} = r{dst}.wrapping_shl({imm})",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::LSH64_REG => {
            rust_expr!(format!(
                "r{dst} = r{dst}.wrapping_shl(r{src} as u32)",
                dst = insn.dst, src = insn.src
            ))
        }

        // rsh64
        ebpf::RSH64_IMM => {
            rust_expr!(format!(
                "r{dst} = r{dst}.wrapping_shr({imm})",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::RSH64_REG => {
            rust_expr!(format!(
                "r{dst} = r{dst}.wrapping_shr(r{src} as u32)",
                dst = insn.dst, src = insn.src
            ))
        }

        // ash64
        ebpf::ARSH64_IMM => {
            rust_expr!(format!(
                "r{dst} = (r{dst} as i64).wrapping_shr({imm})",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::ARSH64_REG => {
            rust_expr!(format!(
                "r{dst} = (r{dst} as i64).wrapping_shr(r{src} as u32)",
                dst = insn.dst, src = insn.src
            ))
        }

        // neg64
        ebpf::NEG64 if sbpf_version < SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = (r{dst} as i64).wrapping_neg() as u64",
                dst = insn.dst
            ))
        }

        // mov64
        ebpf::MOV64_IMM => {
            rust_expr!(format!(
                "r{dst} = {imm} as i32 as i64 as u64",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::MOV64_REG => {
            rust_expr!(format!(
                "r{dst} = r{src}",
                dst = insn.dst, src = insn.src
            ))
        }
        
        // SBPF v2+ Arithmetic
    
        // hor64 (added in v2)
        ebpf::HOR64_IMM if sbpf_version >= SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = r{dst}.or(({imm} as u64).wrapping_shl(32))",
                dst = insn.dst, imm = insn.imm
            ))
        }

        // uhmul64
        ebpf::UHMUL64_IMM if sbpf_version >= SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = (r{dst} as u128).wrapping_mul({imm} as u128).wrapping_shr(64) as u64",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::UHMUL64_REG if sbpf_version >= SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = (r{dst} as u128).wrapping_mul(r{src} as u128).wrapping_shr(64) as u64",
                dst = insn.dst, src = insn.src
            ))
        }

        // udiv32
        ebpf::UDIV32_IMM if sbpf_version >= SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = ((r{dst} as u32) / {imm}) as u64",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::UDIV32_REG if sbpf_version >= SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = ((r{dst} as u32) / (r{src} as u32)) as u64",
                dst = insn.dst, src = insn.src
            ))
        }

        // udiv64
        ebpf::UDIV64_IMM if sbpf_version >= SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = r{dst} / ({imm} as u64)",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::UDIV64_REG if sbpf_version >= SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = r{dst} / r{src}",
                dst = insn.dst, src = insn.src
            ))
        }

        // urem32
        ebpf::UREM32_IMM if sbpf_version >= SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = ((r{dst} as u32) % {imm}) as u64",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::UREM32_REG if sbpf_version >= SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = ((r{dst} as u32) % (r{src} as u32)) as u64",
                dst = insn.dst, src = insn.src
            ))
        }

        // urem64
        ebpf::UREM64_IMM if sbpf_version >= SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = r{dst} % ({imm} as u64)",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::UREM64_REG if sbpf_version >= SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = r{dst} % r{src}",
                dst = insn.dst, src = insn.src
            ))
        }

        // lmul32
        ebpf::LMUL32_IMM if sbpf_version >= SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = (r{dst} as i32).wrapping_mul({imm} as i32) as u32 as u64",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::LMUL32_REG if sbpf_version >= SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = (r{dst} as i32).wrapping_mul(r{src} as i32) as u32 as u64",
                dst = insn.dst, src = insn.src
            ))
        }

        // lmul64
        ebpf::LMUL64_IMM if sbpf_version >= SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = r{dst}.wrapping_mul({imm} as u64)",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::LMUL64_REG if sbpf_version >= SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = r{dst}.wrapping_mul(r{src})",
                dst = insn.dst, src = insn.src
            ))
        }

        // shmul64
        ebpf::SHMUL64_IMM if sbpf_version >= SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = (r{dst} as i128).wrapping_mul({imm} as i32 as i128).wrapping_shr(64) as i64 as u64",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::SHMUL64_REG if sbpf_version >= SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = (r{dst} as i128).wrapping_mul(r{src} as i64 as i128).wrapping_shr(64) as i64 as u64",
                dst = insn.dst, src = insn.src
            ))
        }

        // sdiv32
        ebpf::SDIV32_IMM if sbpf_version >= SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = ((r{dst} as i32) / ({imm} as i32)) as u32 as u64",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::SDIV32_REG if sbpf_version >= SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = ((r{dst} as i32) / (r{src} as i32)) as u32 as u64",
                dst = insn.dst, src = insn.src
            ))
        }

        // sdiv64
        ebpf::SDIV64_IMM if sbpf_version >= SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = ((r{dst} as i64) / ({imm} as i64)) as u64",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::SDIV64_REG if sbpf_version >= SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = ((r{dst} as i64) / (r{src} as i64)) as u64",
                dst = insn.dst, src = insn.src
            ))
        }

        // srem32
        ebpf::SREM32_IMM if sbpf_version >= SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = ((r{dst} as i32) % ({imm} as i32)) as u32 as u64",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::SREM32_REG if sbpf_version >= SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = ((r{dst} as i32) % (r{src} as i32)) as u32 as u64",
                dst = insn.dst, src = insn.src
            ))
        }

        // srem64
        ebpf::SREM64_IMM if sbpf_version >= SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = ((r{dst} as i64) % ({imm} as i64)) as u64",
                dst = insn.dst, imm = insn.imm
            ))
        }
        ebpf::SREM64_REG if sbpf_version >= SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = ((r{dst} as i64) % (r{src} as i64)) as u64",
                dst = insn.dst, src = insn.src
            ))
        }

        // === BPF_LD_IMM (until v2) ===

        ebpf::LD_DW_IMM if sbpf_version < SBPFVersion::V2 => {
            rust_expr!(format!(
                "r{dst} = {imm} as u64",
                dst = insn.dst, imm = insn.imm
            ))
        }

        // === BPF_JMP class ===

        // ja
        ebpf::JA => {
            rust_expr!(format!(
                "if true {{ pc += {off} + 1 }}",
                off = insn.off
            ))
        }

        // jeq
        ebpf::JEQ_IMM => {
            rust_expr!(format!(
                "if r{dst} == ({imm} as i32 as i64 as u64) {{ pc += {off} + 1 }}",
                dst = insn.dst, imm = insn.imm, off = insn.off
            ))
        }
        ebpf::JEQ_REG => {
            rust_expr!(format!(
                "if r{dst} == r{src} {{ pc += {off} + 1 }}",
                dst = insn.dst, src = insn.src, off = insn.off
            ))
        }

        // jgt
        ebpf::JGT_IMM => {
            rust_expr!(format!(
                "if r{dst} > ({imm} as i32 as i64 as u64) {{ pc += {off} + 1 }}",
                dst = insn.dst, imm = insn.imm, off = insn.off
            ))
        }
        ebpf::JGT_REG => {
            rust_expr!(format!(
                "if r{dst} > r{src} {{ pc += {off} + 1 }}",
                dst = insn.dst, src = insn.src, off = insn.off
            ))
        }

        // jge
        ebpf::JGE_IMM => {
            rust_expr!(format!(
                "if r{dst} >= ({imm} as i32 as i64 as u64) {{ pc += {off} + 1 }}",
                dst = insn.dst, imm = insn.imm, off = insn.off
            ))
        }
        ebpf::JGE_REG => {
            rust_expr!(format!(
                "if r{dst} >= r{src} {{ pc += {off} + 1 }}",
                dst = insn.dst, src = insn.src, off = insn.off
            ))
        }

        // jset
        ebpf::JSET_IMM => {
            rust_expr!(format!(
                "if r{dst}.and({imm} as i32 as i64 as u64) != 0 {{ pc += {off} + 1 }}",
                dst = insn.dst, imm = insn.imm, off = insn.off
            ))
        }
        ebpf::JSET_REG => {
            rust_expr!(format!(
                "if r{dst}.and(r{src}) != 0 {{ pc += {off} + 1 }}",
                dst = insn.dst, src = insn.src, off = insn.off
            ))
        }

        // jne
        ebpf::JNE_IMM => {
            rust_expr!(format!(
                "if r{dst} != ({imm} as i32 as i64 as u64) {{ pc += {off} + 1 }}",
                dst = insn.dst, imm = insn.imm, off = insn.off
            ))
        }
        ebpf::JNE_REG => {
            rust_expr!(format!(
                "if r{dst} != r{src} {{ pc += {off} + 1 }}",
                dst = insn.dst, src = insn.src, off = insn.off
            ))
        }

        // jsgt
        ebpf::JSGT_IMM => {
            rust_expr!(format!(
                "if (r{dst} as i64) > ({imm} as i32 as i64) {{ pc += {off} + 1 }}",
                dst = insn.dst, imm = insn.imm, off = insn.off
            ))
        }
        ebpf::JSGT_REG => {
            rust_expr!(format!(
                "if (r{dst} as i64) > (r{src} as i64) {{ pc += {off} + 1 }}",
                dst = insn.dst, src = insn.src, off = insn.off
            ))
        }

        // jsge
        ebpf::JSGE_IMM => {
            rust_expr!(format!(
                "if (r{dst} as i64) >= ({imm} as i32 as i64) {{ pc += {off} + 1 }}",
                dst = insn.dst, imm = insn.imm, off = insn.off
            ))
        }
        ebpf::JSGE_REG => {
            rust_expr!(format!(
                "if (r{dst} as i64) >= (r{src} as i64) {{ pc += {off} + 1 }}",
                dst = insn.dst, src = insn.src, off = insn.off
            ))
        }

        // jlt
        ebpf::JLT_IMM => {
            rust_expr!(format!(
                "if r{dst} < ({imm} as i32 as i64 as u64) {{ pc += {off} + 1 }}",
                dst = insn.dst, imm = insn.imm, off = insn.off
            ))
        }
        ebpf::JLT_REG => {
            rust_expr!(format!(
                "if r{dst} < r{src} {{ pc += {off} + 1 }}",
                dst = insn.dst, src = insn.src, off = insn.off
            ))
        }

        // jle
        ebpf::JLE_IMM => {
            rust_expr!(format!(
                "if r{dst} <= ({imm} as i32 as i64 as u64) {{ pc += {off} + 1 }}",
                dst = insn.dst, imm = insn.imm, off = insn.off
            ))
        }
        ebpf::JLE_REG => {
            rust_expr!(format!(
                "if r{dst} <= r{src} {{ pc += {off} + 1 }}",
                dst = insn.dst, src = insn.src, off = insn.off
            ))
        }

        // jslt
        ebpf::JSLT_IMM => {
            rust_expr!(format!(
                "if (r{dst} as i64) < ({imm} as i32 as i64) {{ pc += {off} + 1 }}",
                dst = insn.dst, imm = insn.imm, off = insn.off
            ))
        }
        ebpf::JSLT_REG => {
            rust_expr!(format!(
                "if (r{dst} as i64) < (r{src} as i64) {{ pc += {off} + 1 }}",
                dst = insn.dst, src = insn.src, off = insn.off
            ))
        }

        // jsle
        ebpf::JSLE_IMM => {
            rust_expr!(format!(
                "if (r{dst} as i64) <= ({imm} as i32 as i64) {{ pc += {off} + 1 }}",
                dst = insn.dst, imm = insn.imm, off = insn.off
            ))
        }
        ebpf::JSLE_REG => {
            rust_expr!(format!(
                "if (r{dst} as i64) <= (r{src} as i64) {{ pc += {off} + 1 }}",
                dst = insn.dst, src = insn.src, off = insn.off
            ))
        }

        _ => None, // Unknown or unsupported instruction
    }
}
