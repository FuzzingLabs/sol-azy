//! Solana syscall registration for SBPF analysis.
//!
//! This module registers all Solana syscalls as stub implementations with the
//! SBPF loader. This allows the disassembler to automatically resolve syscall
//! names instead of showing raw hash values.

use solana_sbpf::{
    declare_builtin_function, error::EbpfError, memory_region::MemoryMapping,
    program::BuiltinProgram,
};
use test_utils::TestContextObject;

// Declare a single stub syscall that does nothing for all syscalls.
declare_builtin_function!(
    SyscallStub,
    fn rust(
        _context: &mut TestContextObject,
        _arg1: u64,
        _arg2: u64,
        _arg3: u64,
        _arg4: u64,
        _arg5: u64,
        _memory_mapping: &mut MemoryMapping,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        unreachable!("not used by disassembly and not intended to be called");
    }
);

/// Register all Solana syscalls as stubs on an existing loader.
///
/// This allows the SBPF disassembler to resolve syscall names automatically
/// instead of showing raw hash values in the output.
///
/// # Arguments
///
/// * `loader` - The BuiltinProgram loader to register syscalls with
///
/// # Returns
///
/// `Ok(())` if all syscalls were registered successfully, otherwise an error.
pub fn register_solana_syscalls(
    loader: &mut BuiltinProgram<TestContextObject>,
) -> Result<(), EbpfError> {
    // Reuse the same stub implementation for all syscalls
    // Sorted alphabetically for maintainability
    loader.register_function("abort", SyscallStub::vm)?;
    loader.register_function("sol_alloc_free_", SyscallStub::vm)?;
    loader.register_function("sol_alt_bn128_compression", SyscallStub::vm)?;
    loader.register_function("sol_alt_bn128_group_op", SyscallStub::vm)?;
    loader.register_function("sol_big_mod_exp", SyscallStub::vm)?;
    loader.register_function("sol_blake3", SyscallStub::vm)?;
    loader.register_function("sol_create_program_address", SyscallStub::vm)?;
    loader.register_function("sol_curve_group_op", SyscallStub::vm)?;
    loader.register_function("sol_curve_validate_point", SyscallStub::vm)?;
    loader.register_function("sol_get_clock_sysvar", SyscallStub::vm)?;
    loader.register_function("sol_get_epoch_schedule_sysvar", SyscallStub::vm)?;
    loader.register_function("sol_get_epoch_stake", SyscallStub::vm)?;
    loader.register_function("sol_get_fees_sysvar", SyscallStub::vm)?;
    loader.register_function("sol_get_last_restart_slot", SyscallStub::vm)?;
    loader.register_function("sol_get_processed_sibling_instruction", SyscallStub::vm)?;
    loader.register_function("sol_get_rent_sysvar", SyscallStub::vm)?;
    loader.register_function("sol_get_return_data", SyscallStub::vm)?;
    loader.register_function("sol_get_stack_height", SyscallStub::vm)?;
    loader.register_function("sol_invoke_signed_c", SyscallStub::vm)?;
    loader.register_function("sol_invoke_signed_rust", SyscallStub::vm)?;
    loader.register_function("sol_keccak256", SyscallStub::vm)?;
    loader.register_function("sol_log_", SyscallStub::vm)?;
    loader.register_function("sol_log_64_", SyscallStub::vm)?;
    loader.register_function("sol_log_compute_units_", SyscallStub::vm)?;
    loader.register_function("sol_log_data", SyscallStub::vm)?;
    loader.register_function("sol_log_pubkey", SyscallStub::vm)?;
    loader.register_function("sol_memcmp_", SyscallStub::vm)?;
    loader.register_function("sol_memcpy_", SyscallStub::vm)?;
    loader.register_function("sol_memmove_", SyscallStub::vm)?;
    loader.register_function("sol_memset_", SyscallStub::vm)?;
    loader.register_function("sol_panic_", SyscallStub::vm)?;
    loader.register_function("sol_poseidon", SyscallStub::vm)?;
    loader.register_function("sol_remaining_compute_units", SyscallStub::vm)?;
    loader.register_function("sol_secp256k1_recover", SyscallStub::vm)?;
    loader.register_function("sol_set_return_data", SyscallStub::vm)?;
    loader.register_function("sol_sha256", SyscallStub::vm)?;
    loader.register_function("sol_try_find_program_address", SyscallStub::vm)?;

    Ok(())
}
