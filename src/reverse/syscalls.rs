//! Solana syscall utilities for disassembly and analysis.

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

/// Solana syscall names that we registered with the loader for better disassembly.
#[rustfmt::skip]
const SYSCALL_NAMES: &[&str] = &[
    // Terminal syscalls
    "abort",
    "sol_panic_",

    // Logging
    "sol_log_",
    "sol_log_64_",
    "sol_log_compute_units_",
    "sol_log_pubkey",
    "sol_log_data",

    // Memory
    "sol_memcpy_",
    "sol_memmove_",
    "sol_memset_",
    "sol_memcmp_",

    // Sysvars (Legacy & Unified)
    "sol_get_sysvar",
    "sol_get_clock_sysvar",
    "sol_get_epoch_schedule_sysvar",
    "sol_get_epoch_stake",
    "sol_get_fees_sysvar",
    "sol_get_last_restart_slot",
    "sol_get_rent_sysvar",

    // Runtime Info
    "sol_get_stack_height",
    "sol_remaining_compute_units",
    "sol_get_processed_sibling_instruction",

    // Crypto & Math
    "sol_sha256",
    "sol_keccak256",
    "sol_blake3",
    "sol_poseidon",
    "sol_big_mod_exp",
    "sol_secp256k1_recover",
    "sol_curve_validate_point",
    "sol_curve_group_op",
    "sol_alt_bn128_compression",
    "sol_alt_bn128_group_op",

    // CPI & PDAs
    "sol_invoke_signed_c",
    "sol_invoke_signed_rust",
    "sol_set_return_data",
    "sol_get_return_data",
    "sol_create_program_address",
    "sol_try_find_program_address",

    // Deprecated
    "sol_alloc_free_",
];

/// Register all Solana syscalls as stubs on an existing loader.
pub(crate) fn register_solana_syscalls(
    loader: &mut BuiltinProgram<TestContextObject>,
) -> Result<(), EbpfError> {
    for &name in SYSCALL_NAMES {
        loader.register_function(name, SyscallStub::vm)?;
    }
    Ok(())
}

/// Syscall signatures mapping syscall names to their human-readable calling convention.
/// Linear lookup is likely faster than a HashMap for this small dataset.
#[rustfmt::skip]
const SYSCALL_SIGNATURES: &[(&str, &str)] = &[
    // --- TERMINAL SYSCALLS ---
    ("abort", "abort()"),
    ("sol_panic_", "sol_panic_(r1, r2, r3)"),

    // --- LOGGING ---
    ("sol_log_", "r0 = sol_log_(r1, r2)"),
    ("sol_log_64_", "r0 = sol_log_64_(r1, r2, r3, r4, r5)"),
    ("sol_log_compute_units_", "r0 = sol_log_compute_units_()"),
    ("sol_log_pubkey", "r0 = sol_log_pubkey(r1)"),
    ("sol_log_data", "r0 = sol_log_data(r1, r2)"),

    // --- MEMORY ---
    ("sol_memcpy_", "r0 = sol_memcpy_(r1, r2, r3)"),
    ("sol_memmove_", "r0 = sol_memmove_(r1, r2, r3)"),
    ("sol_memset_", "r0 = sol_memset_(r1, r2, r3)"),
    ("sol_memcmp_", "r0 = sol_memcmp_(r1, r2, r3, r4)"),

    // --- SYSVARS & RUNTIME ---
    ("sol_get_sysvar", "r0 = sol_get_sysvar(r1, r2, r3, r4)"),
    ("sol_get_clock_sysvar", "r0 = sol_get_clock_sysvar(r1)"),
    ("sol_get_rent_sysvar", "r0 = sol_get_rent_sysvar(r1)"),
    ("sol_get_epoch_stake", "r0 = sol_get_epoch_stake(r1)"),
    ("sol_get_fees_sysvar", "r0 = sol_get_fees_sysvar(r1)"),
    ("sol_get_last_restart_slot", "r0 = sol_get_last_restart_slot(r1)"),
    ("sol_get_epoch_schedule_sysvar", "r0 = sol_get_epoch_schedule_sysvar(r1)"),
    ("sol_get_stack_height", "r0 = sol_get_stack_height()"),
    ("sol_remaining_compute_units", "r0 = sol_remaining_compute_units()"),
    ("sol_get_processed_sibling_instruction", "r0 = sol_get_processed_sibling_instruction(r1, r2, r3)"),

    // --- CRYPTO & MATH ---
    ("sol_sha256", "r0 = sol_sha256(r1, r2, r3)"),
    ("sol_keccak256", "r0 = sol_keccak256(r1, r2, r3)"),
    ("sol_blake3", "r0 = sol_blake3(r1, r2, r3)"),
    ("sol_poseidon", "r0 = sol_poseidon(r1, r2, r3, r4)"),
    ("sol_big_mod_exp", "r0 = sol_big_mod_exp(r1, r2, r3)"),
    ("sol_secp256k1_recover", "r0 = sol_secp256k1_recover(r1, r2, r3, r4)"),
    ("sol_curve_validate_point", "r0 = sol_curve_validate_point(r1, r2, r3)"),
    ("sol_curve_group_op", "r0 = sol_curve_group_op(r1, r2, r3, r4)"),
    ("sol_alt_bn128_compression", "r0 = sol_alt_bn128_compression(r1, r2, r3)"),
    ("sol_alt_bn128_group_op", "r0 = sol_alt_bn128_group_op(r1, r2, r3, r4)"),

    // --- CPI & PDAs ---
    ("sol_invoke_signed_c", "r0 = sol_invoke_signed_c(r1, r2, r3, r4, r5)"),
    ("sol_invoke_signed_rust", "r0 = sol_invoke_signed_rust(r1, r2, r3, r4, r5)"),
    ("sol_set_return_data", "r0 = sol_set_return_data(r1, r2)"),
    ("sol_get_return_data", "r0 = sol_get_return_data(r1, r2, r3)"),
    ("sol_create_program_address", "r0 = sol_create_program_address(r1, r2, r3, r4)"),
    ("sol_try_find_program_address", "r0 = sol_try_find_program_address(r1, r2, r3, r4, r5)"),

    // --- DEPRECATED ---
    ("sol_alloc_free_", "r0 = sol_alloc_free_(r1, r2)"),
];

pub(crate) fn get_syscall_signature(name: &str) -> Option<&'static str> {
    SYSCALL_SIGNATURES
        .iter()
        .find(|(syscall_name, _)| *syscall_name == name)
        .map(|(_, sig)| *sig)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registered_names_match_signature_names() {
        let mut names_sorted: Vec<&str> = SYSCALL_NAMES.to_vec(); // registered names
        names_sorted.sort();

        let mut signature_names_sorted: Vec<&str> =
            SYSCALL_SIGNATURES.iter().map(|(name, _)| *name).collect();
        signature_names_sorted.sort();

        assert_eq!(
            names_sorted, signature_names_sorted,
            "SYSCALL_NAMES and SYSCALL_SIGNATURES contain different sets of syscalls"
        );
    }
}
