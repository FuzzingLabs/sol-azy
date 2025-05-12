use std::fmt::Write as _;
use solana_sbpf::ebpf::Insn;

/// Formats a byte slice into a Rust-style byte string literal (`b"..."`).
///
/// Printable ASCII characters (including spaces) are rendered as-is.
/// Non-printable or non-ASCII bytes are rendered using hexadecimal escapes (`\xNN`).
///
/// # Arguments
///
/// * `slice` - The byte slice to format.
///
/// # Returns
///
/// A `String` formatted as a byte string literal with proper escaping.
/// 
pub fn format_bytes(slice: &[u8]) -> String {
    let mut bytes_repr = String::from("b\"");

    // Render printable ASCII as-is, otherwise use hex escape
    for &b in slice {
        if b.is_ascii_graphic() || b == b' ' {
            bytes_repr.push(b as char);
        } else {
            // Add hexadecimal escape sequence for non-printable bytes
            write!(&mut bytes_repr, "\\x{:02x}", b).unwrap();
        }
    }

    bytes_repr.push('"');
    bytes_repr
}


/// Maximum number of bytes used to represents the extracted string representation
/// from a load immediate instruction (useful if no explicit length is provided).
pub const MAX_BYTES_USED_TO_READ_FOR_IMMEDIATE_STRING_REPR: u8 = 50;

/// Attempts to retrieve a string representation from a `LD_DW_IMM` instruction,
/// optionally followed by a `MOV64_IMM` or `MOV32_IMM` that gives the string length.
///
/// # Arguments
///
/// * `program` - Raw program bytes
/// * `insn` - The instruction to analyze
/// * `next_insn_wrapped` - Optional next instruction that might contain length info
///
/// # Returns
///
/// A string representation of the bytes referenced by the instruction, or an empty string.
pub fn get_string_repr(
    program: &[u8],
    insn: &Insn,
    next_insn_wrapped: Option<&Insn>,
) -> String {
    match insn.opc {
        solana_sbpf::ebpf::LD_DW_IMM => {
            let offset_base = solana_sbpf::ebpf::MM_RODATA_START as usize;
            let start = if insn.imm > 0 && insn.imm as usize > offset_base {
                insn.imm as usize - offset_base
            } else {
                return "".to_string();
            };

            if start >= program.len() {
                return "".to_string();
            }

            let mut length = MAX_BYTES_USED_TO_READ_FOR_IMMEDIATE_STRING_REPR as usize;

            if let Some(next_insn) = next_insn_wrapped {
                if next_insn.opc == solana_sbpf::ebpf::MOV64_IMM
                    || next_insn.opc == solana_sbpf::ebpf::MOV32_IMM
                {
                    let maybe_len = next_insn.imm as usize;
                    if maybe_len > 0 {
                        length = maybe_len;
                    }
                }
            }

            let end = usize::min(start + length, program.len());
            let slice = &program[start..end];
            format_bytes(slice)
        }

        _ => "".to_string(),
    }
}