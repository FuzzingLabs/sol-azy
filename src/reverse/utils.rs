use solana_sbpf::ebpf::{Insn, HOR64_IMM, LD_B_REG, LD_DW_IMM, LD_DW_REG, LD_H_REG, LD_W_REG, MM_RODATA_START, MOV32_IMM, MOV64_IMM};
use std::collections::HashMap;
use std::fmt::Write as _;

/// Maximum number of bytes used to represents the extracted string representation
/// from a load immediate instruction (useful if no explicit length is provided).
pub const MAX_BYTES_USED_TO_READ_FOR_IMMEDIATE_STRING_REPR: u8 = 50;

#[derive(Clone, Debug)]
pub enum Value {
    Const(u64),
    Unknown,
}

#[derive(Clone, Debug)]
pub struct RegisterTracker {
    registers: HashMap<u8, Value>,
}

impl RegisterTracker {
    pub fn new() -> Self {
        Self {
            registers: HashMap::new(),
        }
    }

    pub fn update(&mut self, insn: &Insn) {
        match insn.opc {
            MOV32_IMM => {
                // used for string repr and low bits of an address can only be > 0 (see issue #45)
                self.registers
                    .insert(insn.dst, Value::Const(insn.imm as u32 as u64));
            }
            MOV64_IMM => {
                self.registers
                    .insert(insn.dst, Value::Const(insn.imm as u64));
            }
            HOR64_IMM => {
                if let Some(Value::Const(low)) = self.registers.get(&insn.dst) {
                    // used for string repr and high bits of an address can, also, only be > 0
                    let high = (insn.imm as u32 as u64) << 32;
                    let combined = high | (*low & 0xFFFF_FFFF);
                    self.registers.insert(insn.dst, Value::Const(combined));
                } else {
                    self.registers.insert(insn.dst, Value::Unknown);
                }
            }
            _ => {
                self.registers.insert(insn.dst, Value::Unknown);
            }
        }
    }

    pub fn get(&self, reg: u8) -> Option<&Value> {
        self.registers.get(&reg)
    }
}

/// Attempts to resolve a string representation from memory based on the current instruction context
/// and register state, supporting both legacy (`LD_DW_IMM`) and v2+ (`LD_*_REG`) sBPF formats.
///
/// This function updates the provided [`RegisterTracker`] with the current instruction,
/// and tries to resolve a string if the instruction:
/// - Loads a constant directly (`LD_DW_IMM`)
/// - Loads a value indirectly using a register address (`LD_DW_REG`, `LD_B_REG`, `LD_H_REG`, `LD_W_REG`)
///
/// If the next instruction is a `MOV64_IMM` or `MOV32_IMM`, it may be interpreted as the string length.
///
/// # Arguments
///
/// * `program` - Raw bytecode of the eBPF program.
/// * `insn` - The current instruction being processed.
/// * `next_insn_wrapped` - Optional reference to the next instruction, possibly providing string length.
/// * `register_tracker` - Mutable reference to a [`RegisterTracker`] that maintains register state.
///
/// # Returns
///
/// A formatted string representation (`b"..."`) of the resolved memory slice,
/// or an empty string if resolution fails or is not applicable.
pub fn update_string_resolution(program: &[u8], insn: &Insn, next_insn_wrapped: Option<&Insn>, register_tracker: &mut RegisterTracker) -> String {
    register_tracker.update(insn);
    match insn.opc {
        // used for sBPF_version >= 2
        LD_DW_REG | LD_B_REG | LD_H_REG | LD_W_REG => {
            let reg_value = register_tracker.get(insn.dst);
            let offset = insn.off as i32; // avoiding potential panics due to overflowing while getting absolute value 
            match reg_value {
                Some(Value::Const(value)) => {
                    let offset_base = MM_RODATA_START as usize;
                    if *value < offset.abs() as u64 {
                        return "".to_string();
                    }
                    let addr = value.wrapping_add(offset as i64 as u64);
                    let start = if addr as usize > offset_base {
                        addr as usize - offset_base
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
        },
        LD_DW_IMM => {
            let offset_base = MM_RODATA_START as usize;
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
