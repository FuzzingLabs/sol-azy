use solana_sbpf::{ebpf, ebpf::Insn, program::SBPFVersion};
use std::collections::HashMap;
use std::fmt::Write as _;

/// Maximum number of bytes used to represents the extracted string representation
/// from a load immediate instruction (useful if no explicit length is provided).
pub const MAX_BYTES_USED_TO_READ_FOR_IMMEDIATE_STRING_REPR: u8 = 50;

/// Returns the base address of the memory region containing the .rodata section.
///
/// In SBPF V1/V2, the .rodata section is mapped within the BYTECODE region (after the code).
/// This function returns `MM_BYTECODE_START`, which is the start of the entire region.
///
/// In SBPF V3+, the .rodata section has its own dedicated RODATA region starting at address 0.
///
/// # Arguments
///
/// * `sbpf_version` - The SBPF version from the executable.
///
/// # Returns
///
/// The starting virtual address of the memory region containing .rodata.
pub(crate) fn get_rodata_region_start(sbpf_version: SBPFVersion) -> u64 {
    if sbpf_version < SBPFVersion::V3 {
        ebpf::MM_BYTECODE_START
    } else {
        ebpf::MM_RODATA_START
    }
}

/// Checks if an address points to the .rodata section based on SBPF version.
///
/// In SBPF V1/V2, .rodata is mapped within the BYTECODE region.
/// In SBPF V3+, .rodata has its own dedicated RODATA region.
///
/// # Arguments
///
/// * `addr` - The virtual address to check
/// * `sbpf_version` - The SBPF version from the executable
///
/// # Returns
///
/// `true` if the address maps to the .rodata section, `false` otherwise.
pub(crate) fn is_rodata_address(addr: u64, sbpf_version: SBPFVersion) -> bool {
    if sbpf_version < SBPFVersion::V3 {
        // V1/V2: .rodata is within BYTECODE region
        addr >= ebpf::MM_BYTECODE_START && addr < ebpf::MM_STACK_START
    } else {
        // V3+: .rodata has dedicated RODATA region
        addr >= ebpf::MM_RODATA_START && addr < ebpf::MM_BYTECODE_START
    }
}

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
            ebpf::MOV32_IMM => {
                // used for string repr and low bits of an address can only be > 0 (see issue #45)
                self.registers
                    .insert(insn.dst, Value::Const(insn.imm as u32 as u64));
            }
            ebpf::MOV64_IMM => {
                self.registers
                    .insert(insn.dst, Value::Const(insn.imm as u64));
            }
            ebpf::HOR64_IMM => {
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
/// * `sbpf_version` - The SBPF version from the executable.
///
/// # Returns
///
/// A formatted string representation (`b"..."`) of the resolved memory slice,
/// or an empty string if resolution fails or is not applicable.
pub fn update_string_resolution(
    program: &[u8],
    insn: &Insn,
    next_insn_wrapped: Option<&Insn>,
    register_tracker: &mut RegisterTracker,
    sbpf_version: SBPFVersion,
) -> String {
    register_tracker.update(insn);

    let rodata_region_start = get_rodata_region_start(sbpf_version);

    match insn.opc {
        // used for sBPF_version >= 2
        ebpf::LD_DW_REG | ebpf::LD_B_REG | ebpf::LD_H_REG | ebpf::LD_W_REG => {
            let reg_value = register_tracker.get(insn.src);
            let offset = insn.off as i32; // avoiding potential panics due to overflowing while getting absolute value
            match reg_value {
                Some(Value::Const(value)) => {
                    if *value < offset.abs() as u64 {
                        return "".to_string();
                    }
                    let addr = value.wrapping_add(offset as i64 as u64);

                    // Verify the address is in the .rodata section
                    if !is_rodata_address(addr, sbpf_version) {
                        return "".to_string();
                    }

                    // Convert virtual address to offset into program bytecode array
                    // Safe: is_rodata_address() guarantees addr >= rodata_region_start
                    let start = (addr - rodata_region_start) as usize;

                    if start >= program.len() {
                        return "".to_string();
                    }

                    let mut length = MAX_BYTES_USED_TO_READ_FOR_IMMEDIATE_STRING_REPR as usize;

                    if let Some(next_insn) = next_insn_wrapped {
                        if next_insn.opc == ebpf::MOV64_IMM || next_insn.opc == ebpf::MOV32_IMM {
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
        ebpf::LD_DW_IMM => {
            let addr = insn.imm as u64;

            // Verify the address is in the .rodata section
            if !is_rodata_address(addr, sbpf_version) {
                return "".to_string();
            }

            // Convert virtual address to offset into program bytecode array
            // Safe: is_rodata_address() guarantees addr >= rodata_region_start
            let start = ((insn.imm as u64) - rodata_region_start) as usize;

            if start >= program.len() {
                return "".to_string();
            }

            let mut length = MAX_BYTES_USED_TO_READ_FOR_IMMEDIATE_STRING_REPR as usize;

            if let Some(next_insn) = next_insn_wrapped {
                if next_insn.opc == ebpf::MOV64_IMM || next_insn.opc == ebpf::MOV32_IMM {
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
