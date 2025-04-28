use solana_sbpf::{ebpf::LD_DW_IMM, static_analysis::Analysis};
use std::u8;

use crate::reverse::immediate_tracker::ImmediateTracker;
use crate::reverse::utils::format_bytes;
use crate::reverse::OutputFile;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use super::utils::{get_string_repr, MAX_BYTES_USED_TO_READ_FOR_IMMEDIATE_STRING_REPR};

/// Performs the core disassembly process of the program based on a provided static analysis.
///
/// This function prints disassembled instructions into the output file, annotating
/// each instruction and registering immediate values when encountered via `LD_DW_IMM`.
///
/// # Arguments
///
/// * `analysis` - The static analysis object containing instructions and metadata.
/// * `imm_tracker_wrapped` - An optional mutable reference to an `ImmediateTracker`
///   used to track offsets of immediate values.
/// * `path` - Base path where the disassembly file should be written.
///
/// # Returns
///
/// A `Result` indicating the success or failure of the disassembly file write operation.
///
/// # Note
///
/// This is a modified version of `disassemble` from `sbpf-solana`, adapted to support
/// enhanced static analysis features.
fn disassemble<P: AsRef<Path>>(
    program: &[u8],
    analysis: &mut Analysis,
    mut imm_tracker_wrapped: Option<&mut ImmediateTracker>,
    path: P,
) -> std::io::Result<()> {
    let mut disass_path = PathBuf::from(path.as_ref());
    disass_path.push(OutputFile::Disassembly.default_filename());
    let mut output = File::create(disass_path)?;
    let mut last_basic_block = usize::MAX;

    for (pc, insn) in analysis.instructions.iter().enumerate() {
        analysis.disassemble_label(
            &mut output,
            Some(insn) == analysis.instructions.first(),
            insn.ptr,
            &mut last_basic_block,
        )?;

        if insn.opc == LD_DW_IMM {
            if let Some(ref mut imm_tracker) = imm_tracker_wrapped {
                imm_tracker.register_offset(insn.imm as usize);
            }
        }
        
        // next instruction lookup to gather information (like for string and their length when it uses MOV64_IMM)
        let next_insn = analysis.instructions.get(pc + 1);
        let mut insn_line = analysis.disassemble_instruction(insn, pc);
        // add immediate string repr if it does exists on bytecode 
        let str_repr = get_string_repr(program, insn, next_insn);
        if str_repr != "" {
            insn_line.push_str(" --> ");
            insn_line.push_str(&str_repr);
            if insn_line.len() > 2 * (MAX_BYTES_USED_TO_READ_FOR_IMMEDIATE_STRING_REPR as usize) + 1 {
                insn_line.truncate(2 * (MAX_BYTES_USED_TO_READ_FOR_IMMEDIATE_STRING_REPR as usize));
                insn_line = format!("{insn_line}…");
            }
        }
        writeln!(output, "    {}", insn_line)?;
    }
    Ok(())
}

/// Wrapper function that performs disassembly and optionally generates an immediate data table.
///
/// The disassembly output is written to its output file. If an `ImmediateTracker` is provided,
/// an other file is also created, listing readable representations of tracked immediate byte slices.
///
/// # Arguments
///
/// * `program` - The raw bytecode of the eBPF program.
/// * `analysis` - The static analysis object containing instructions and metadata.
/// * `imm_tracker_wrapped` - Optional mutable reference to an `ImmediateTracker` for tracking.
/// * `path` - Base path for writing output files (`disassembly.out`, `immediate_data_table.out`).
///
/// # Returns
///
/// A `Result` indicating the success or failure of the disassembly and table exports.
pub fn disassemble_wrapper<P: AsRef<Path>>(
    program: &[u8],
    analysis: &mut Analysis,
    mut imm_tracker_wrapped: Option<&mut ImmediateTracker>,
    path: P,
) -> std::io::Result<()> {
    disassemble(program, analysis, imm_tracker_wrapped.as_deref_mut(), &path)?;

    if let Some(imm_tracker) = imm_tracker_wrapped {
        let mut table_path = PathBuf::from(path.as_ref());
        table_path.push(OutputFile::ImmediateDataTable.default_filename());
        let mut output = File::create(table_path)?;
        
        let offset_base = solana_sbpf::ebpf::MM_RODATA_START as usize;

        for (&start, &end) in imm_tracker.get_ranges() {
            assert!(start >= offset_base, "start address and end address should be > than the RODATA MemoryMapping section");
            let start_idx = start - offset_base;
            let end_idx = if end > offset_base {
                end - offset_base
            } else {
                end
            };
            if start_idx >= program.len() || end_idx > program.len() || start_idx >= end_idx {
                continue;
            }
            let slice = &program[start_idx..end_idx];
            let repr = format_bytes(slice);
            writeln!(output, "0x{:x} (+ 0x{:x}): {}", start, start_idx, repr)?;
        }
    }

    Ok(())
}
