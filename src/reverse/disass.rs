use solana_sbpf::{ebpf::LD_DW_IMM, static_analysis::Analysis};
use std::u8;

use crate::reverse::immutable_tracker::ImmutableTracker;
use crate::reverse::utils::format_bytes;
use crate::reverse::OutputFile;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

// modified version of "disassemble" from sbpf-solana for better static analysis
fn disassemble<P: AsRef<Path>>(
    analysis: &mut Analysis,
    mut imm_tracker_wrapped: Option<&mut ImmutableTracker>,
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

        let insn_line = analysis.disassemble_instruction(insn, pc);
        writeln!(output, "    {}", insn_line)?;
    }
    Ok(())
}

pub fn disassemble_wrapper<P: AsRef<Path>>(
    program: &'static [u8],
    analysis: &mut Analysis,
    mut imm_tracker_wrapped: Option<&mut ImmutableTracker>,
    path: P,
) -> std::io::Result<()> {
    disassemble(analysis, imm_tracker_wrapped.as_deref_mut(), &path)?;

    if let Some(imm_tracker) = imm_tracker_wrapped {
        let mut table_path = PathBuf::from(path.as_ref());
        table_path.push(OutputFile::ImmutableDataTable.default_filename());
        let mut output = File::create(table_path)?;

        for (&start, &end) in imm_tracker.get_ranges() {
            if start >= program.len() || end > program.len() || start >= end {
                continue;
            }
            let slice = &program[start..end];
            let repr = format_bytes(slice);
            writeln!(output, "0x{:x}: {}", start, repr)?;
        }
    }

    Ok(())
}
