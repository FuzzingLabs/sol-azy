//! Solana syscall registry for resolving syscall hashes to the function names
//! their IDs are derived from.

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Registry mapping syscall hash values to their function names.
static SYSCALL_REGISTRY: Lazy<HashMap<u32, &'static str>> = Lazy::new(|| {
    // The values below are generated from the const `sys_hash` function in the
    // link below. We can't use it here as they only build it for BPF targets.
    // https://github.com/anza-xyz/solana-sdk/blob/define-syscall%40v5.0.0/define-syscall/src/lib.rs#L49-L103
    HashMap::from([
        (0x11f49d86, "sol_sha256"),
        (0x174c5122, "sol_blake3"),
        (0x17e40350, "sol_secp256k1_recover"),
        (0x207559bd, "sol_log_"),
        (0x23a29a61, "sol_get_epoch_schedule_sysvar"),
        (0x3770fb22, "sol_memset_"),
        (0x3b97b73c, "sol_get_fees_sysvar"),
        (0x434371f8, "sol_memmove_"),
        (0x48504a38, "sol_try_find_program_address"),
        (0x52ba5096, "sol_log_compute_units_"),
        (0x5c2a3178, "sol_log_64_"),
        (0x5d2245e4, "sol_get_return_data"),
        (0x5fdcde31, "sol_memcmp_"),
        (0x686093bb, "sol_panic_"),
        (0x717cc4a3, "sol_memcpy_"),
        (0x7317b434, "sol_log_data"),
        (0x7ef088ca, "sol_log_pubkey"),
        (0x83f00e8f, "sol_alloc_free_"),
        (0x85532d94, "sol_get_stack_height"),
        (0x9377323c, "sol_create_program_address"),
        (0xa226d3eb, "sol_set_return_data"),
        (0xa22b9c85, "sol_invoke_signed_c"),
        (0xaa2607ca, "sol_curve_validate_point"),
        (0xadb8efc8, "sol_get_processed_sibling_instruction"),
        (0xb6fc1a11, "abort"),
        (0xbf7188f6, "sol_get_rent_sysvar"),
        (0xd56b5fe9, "sol_get_clock_sysvar"),
        (0xd7449092, "sol_invoke_signed_rust"),
        (0xd7793abb, "sol_keccak256"),
        (0xdd1c41a6, "sol_curve_group_op"),
    ])
});

pub(crate) fn lookup_syscall(hash: u32) -> Option<&'static str> {
    SYSCALL_REGISTRY.get(&hash).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_lookup() {
        // Test a known syscall
        assert_eq!(lookup_syscall(0x207559bd), Some("sol_log_"));

        // Test unknown hash
        assert_eq!(lookup_syscall(0xFFFFFFFF), None);
    }
}
