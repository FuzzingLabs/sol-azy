// Portions of this file are adapted from the `sbpf` project from anza,
// licensed under the MIT license.
// See https://github.com/anza-xyz/sbpf

use indicatif::ProgressIterator;
use log::debug;
use solana_sbpf::{ebpf, program::SBPFVersion, static_analysis::Analysis};

use crate::helpers;
use crate::reverse::immediate_tracker::ImmediateTracker;
use crate::reverse::rusteq::translate_to_rust;
use crate::reverse::syscalls::lookup_syscall;
use crate::reverse::utils::{
    format_bytes, get_rodata_region_start, is_rodata_address, update_string_resolution,
    RegisterTracker, MAX_BYTES_USED_TO_READ_FOR_IMMEDIATE_STRING_REPR,
};
use crate::reverse::OutputFile;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

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
/// * `sbpf_version` - The sBPF version from the executable.
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
    mut reg_tracker_wrapped: Option<&mut RegisterTracker>,
    sbpf_version: SBPFVersion,
    path: P,
) -> std::io::Result<()> {
    debug!("Disassembling...");
    let mut disass_path = PathBuf::from(path.as_ref());
    disass_path.push(OutputFile::Disassembly.default_filename());
    let mut output = File::create(disass_path)?;
    let mut last_basic_block = usize::MAX;

    for (pc, insn) in analysis.instructions.iter().enumerate().progress() {
        analysis.disassemble_label(
            &mut output,
            Some(insn) == analysis.instructions.first(),
            insn.ptr,
            &mut last_basic_block,
        )?;

        // Track immediate data from LD_DW_IMM instructions that point to .rodata section.
        if insn.opc == ebpf::LD_DW_IMM {
            let addr = insn.imm as u64;

            if is_rodata_address(addr, sbpf_version) {
                if let Some(ref mut imm_tracker) = imm_tracker_wrapped {
                    imm_tracker.register_offset(addr as usize)
                }
            }
        }

        // next instruction lookup to gather information (like for string and their length when it uses MOV64_IMM)
        let next_insn = analysis.instructions.get(pc + 1);
        let mut insn_line = analysis.disassemble_instruction(insn, pc);

        // Resolve syscall names if it's a syscall with a hash
        if let Some(hash_str) = insn_line.strip_prefix("syscall ") {
            if let Ok(hash) = hash_str.trim().parse::<i32>() {
                if let Some(name) = lookup_syscall(hash as u32) {
                    insn_line = format!("syscall {name}");
                }
            }
        }

        // append immediate string representation if available
        let str_repr = reg_tracker_wrapped.as_mut().map_or_else(
            || String::new(),
            |reg_tracker| {
                update_string_resolution(program, insn, next_insn, reg_tracker, sbpf_version)
            },
        );

        if !str_repr.is_empty() {
            insn_line.push_str(" --> ");
            insn_line.push_str(&str_repr);
            if insn_line.len() > 2 * (MAX_BYTES_USED_TO_READ_FOR_IMMEDIATE_STRING_REPR as usize) + 1
            {
                insn_line.truncate(2 * (MAX_BYTES_USED_TO_READ_FOR_IMMEDIATE_STRING_REPR as usize));
                insn_line = format!("{insn_line}…");
            }
        }

        // add rust equivalence repr
        if let Some(rust_eq) = translate_to_rust(insn, sbpf_version) {
            let to_write = format!("{:<40}        {}", insn_line, rust_eq);
            writeln!(output, "    {}", to_write)?;
        } else {
            writeln!(output, "    {}", insn_line)?;
        }
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
/// * `sbpf_version` - The sBPF version from the executable.
/// * `path` - Base path for writing output files (`disassembly.out`, `immediate_data_table.out`).
///
/// # Returns
///
/// A `Result` indicating the success or failure of the disassembly and table exports.
pub fn disassemble_wrapper<P: AsRef<Path>>(
    program: &[u8],
    analysis: &mut Analysis,
    mut imm_tracker_wrapped: Option<&mut ImmediateTracker>,
    mut reg_tracker_wrapped: Option<&mut RegisterTracker>,
    sbpf_version: SBPFVersion,
    path: P,
) -> std::io::Result<()> {
    disassemble(
        program,
        analysis,
        imm_tracker_wrapped.as_deref_mut(),
        reg_tracker_wrapped.as_deref_mut(),
        sbpf_version,
        &path,
    )?;
    debug!("Tracking Immediates...");

    let spinner = helpers::spinner::get_new_spinner(String::from("Performing binary analysis..."));

    if let Some(imm_tracker) = imm_tracker_wrapped {
        let mut table_path = PathBuf::from(path.as_ref());
        table_path.push(OutputFile::ImmediateDataTable.default_filename());
        let mut output = File::create(table_path)?;

        // Get the base address of the .rodata region for offset calculations
        let rodata_region_start = get_rodata_region_start(sbpf_version) as usize;

        for (&start, &end) in imm_tracker.get_ranges() {
            if !is_rodata_address(start as u64, sbpf_version)
                || !is_rodata_address(end as u64, sbpf_version)
            {
                // Skip entries where addresses are not in the rodata region
                continue;
            }

            // Convert virtual addresses to offsets into program bytecode array
            // Safe: is_rodata_address() guarantees both are >= rodata_region_start
            let start_idx = start - rodata_region_start;
            let end_idx = end - rodata_region_start;

            if start_idx >= end_idx || end_idx > program.len() {
                continue;
            }

            let slice = &program[start_idx..end_idx];
            let repr = format_bytes(slice);
            writeln!(output, "0x{:x} (+ 0x{:x}): {}", start, start_idx, repr)?;
        }
    }

    spinner.finish_using_style();
    Ok(())
}
