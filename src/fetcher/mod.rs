use anyhow::Result;
use base64::{engine::general_purpose, Engine};
use reqwest::Client;
use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use std::{fs, path::Path};

/// Default RPC endpoint (mainnet‑beta).
pub const MAINNET_RPC: &str = "https://api.mainnet-beta.solana.com";

/// Container returned by [`fetch_account_contents`].
#[derive(Debug)]
pub struct AccountFetch {
    /// Raw or trimmed bytes of the on‑chain account.
    pub data: Vec<u8>,
    /// `true` when the account is flagged executable (i.e. holds a BPF program).
    pub executable: bool,
}

/// Slice the bytecode starting at the ELF header (0x7F 'E' 'L' 'F') (removing programdata metadata things [should be offset = 45 in https://github.com/anza-xyz/solana-sdk/blob/master/loader-v3-interface/src/state.rs#L47])
fn slice_from_elf_header(bytecode: &[u8]) -> Option<&[u8]> {
    bytecode
        .windows(4)
        .position(|w| w == [0x7F, b'E', b'L', b'F'])
        .map(|idx| &bytecode[idx..])
}

/// If the buffer *might* be an Anchor account, print its potential discriminator.
/// There is no fool‑proof way without the IDL, but dumping the first 8 bytes is handy.
/// https://www.anchor-lang.com/docs/basics/program-structure#account-discriminator
/// https://github.com/solana-foundation/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/attribute/account/src/lib.rs#L30-L34
/// https://github.com/solana-foundation/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/attribute/account/src/lib.rs#L122
fn report_anchor_discriminator(data: &[u8]) -> &[u8] {
    if data.len() >= 8 {
        let disc = &data[..8];
        eprintln!(
            "[fetcher] First 8 bytes (possible Anchor discriminator): {}",
            hex::encode(disc)
        );
        return disc;
    }
    return &[];
}


/// Fetches an arbitrary Solana account.
///
/// * If the account is executable, the function resolves potential `ProgramData` indirection
///   and returns a `Vec<u8>` starting exactly at the ELF header.
/// * Otherwise, the raw account data is returned unmodified.
async fn fetch_account_contents(rpc_url: &str, account: &str) -> Result<AccountFetch> {
    let client = Client::new();

    // Single round‑trip: getAccountInfo
    let request_body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getAccountInfo",
        "params": [
            account,
            { "encoding": "base64" }
        ]
    });

    let res = client.post(rpc_url).json(&request_body).send().await?;
    let res_json: serde_json::Value = res.json().await?;
    let value = &res_json["result"]["value"];

    if value.is_null() {
        return Err(anyhow::anyhow!("Account not found: can't fetch any value using this pubkey, probably invalid pubkey"));
    }

    let executable = value["executable"].as_bool()
        .ok_or_else(|| anyhow::anyhow!("Missing `executable` flag"))?;

    let data_base64 = value["data"][0]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("No data in account response"))?;
    let mut decoded_data = general_purpose::STANDARD.decode(data_base64)?;

    // Upgradeable loader indirection (program -> ProgramData)
    if executable && value["owner"] == "BPFLoaderUpgradeab1e11111111111111111111111" {
        if decoded_data.len() < 36 {
            return Err(anyhow::anyhow!("Upgradeable program account too small"));
        }

        // Bytes [4..36] hold the ProgramData pubkey
        let programdata_pubkey = Pubkey::new_from_array(decoded_data[4..36].try_into().unwrap()); // will not crash since len >= 36 and it is sliced for 32 bytes

        let request_body = json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "getAccountInfo",
            "params": [
                programdata_pubkey.to_string(),
                { "encoding": "base64" }
            ]
        });

        let res = client.post(rpc_url).json(&request_body).send().await?;
        let res_json: serde_json::Value = res.json().await?;
        let value = &res_json["result"]["value"];
        let data_base64 = value["data"][0]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("No data in ProgramData response"))?;
        decoded_data = general_purpose::STANDARD.decode(data_base64)?;
    }

    if executable {
        let elf_slice = slice_from_elf_header(&decoded_data)
            .ok_or_else(|| anyhow::anyhow!("Missing ELF header"))?;
        Ok(AccountFetch { data: elf_slice.to_vec(), executable })
    } else {
        report_anchor_discriminator(&decoded_data);
        Ok(AccountFetch { data: decoded_data, executable })
    }
}

/// High‑level helper: fetches an account and writes it to disk.
///
/// * Executable account -> `fetched_program.so`
/// * Non‑executable account -> `fetched_account.bin`
pub async fn fetch_to<P: AsRef<Path>>(out_dir: P, rpc_url: Option<String>, account: &str) -> Result<()> {
    let rpc_url = rpc_url.unwrap_or_else(|| MAINNET_RPC.to_string());
    let fetched = fetch_account_contents(&rpc_url, account).await?;

    let filename = if fetched.executable { "fetched_program.so" } else { "fetched_account.bin" };
    fs::write(out_dir.as_ref().join(filename), fetched.data)?;
    Ok(())
}

/// Fetches the bytecode of a Solana program from the blockchain and writes it to a `.so` file.
///
/// This function retrieves the program bytecode using the provided `program_id` and RPC endpoint,
/// then writes it to a file named `fetched_program.so` in the specified output directory.
///
/// # Arguments
///
/// * `out_dir` - Path to the output directory where the bytecode file will be saved.
/// * `rpc_url` - Optional Solana RPC endpoint; defaults to `https://api.mainnet-beta.solana.com` if `None`.
/// * `program_id` - The program ID on Solana to fetch the bytecode from.
///
/// # Returns
///
/// * `Ok(())` if the bytecode was successfully fetched and written.
/// * `Err(anyhow::Error)` if any step fails (network error, invalid program ID, write failure, etc.).
///
/// # Output
///
/// The resulting file is saved as:
/// `<out_dir>/fetched_program.so`
///
/// # Errors
///
/// Returns an error if:
/// - The program ID is invalid or not found on-chain.
/// - The bytecode could not be fetched from the RPC.
/// - Writing the output file fails.
///
/// # Requirements
///
/// This function is asynchronous and should be `.await`ed within an async context.
pub async fn fetch_bytecode_to<P: AsRef<Path>>(out_dir: P, rpc_url: Option<String>, program_id: &str) -> Result<()> {
    fetch_to(out_dir, rpc_url, program_id).await
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_EXECUTABLE_PROG: &str = "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX"; // Serum DEX v3 (immutable)
    const TEST_UPGRADEABLE_PROG: &str = "4MangoMjqJ2firMokCjjGgoK8d4MXcrgL7XJaL3w6fVg"; // Mango v4 (upgradeable)
    const TEST_SYSVAR_RENT: &str = "SysvarRent111111111111111111111111111111111";   // Non‑executable Sysvar
    const TEST_INVALID_PUBKEY: &str = "InvalidPubkey1111111111111111111111111111111111"; // Invalid length (47 chars)
    const TEST_MARINADE_STATE_ACCOUNT: &str = "8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC"; // Known AccountInfo name (https://github.com/marinade-finance/liquid-staking-program/blob/main/programs/marinade-finance/src/state/mod.rs)

    /// Ensure we can fetch an immutable BPF program and obtain a valid ELF
    #[tokio::test]
    async fn test_fetch_executable() {
        let res = fetch_account_contents(MAINNET_RPC, TEST_EXECUTABLE_PROG)
            .await
            .expect("Fetch executable program");
        assert!(res.executable, "Account must be flagged executable");
        assert!(
            res.data.starts_with(b"\x7FELF"),
            "Executable must start with ELF header (0x7F 45 4C 46)"
        );
        assert!(
            res.data.len() > 1_000,
            "Executable payload seems unexpectedly small: {} bytes",
            res.data.len()
        );
    }

    /// Ensure we can follow Program -> ProgramData indirection and still retrieve a valid ELF
    #[tokio::test]
    async fn test_fetch_upgradeable() {
        let res = fetch_account_contents(MAINNET_RPC, TEST_UPGRADEABLE_PROG)
            .await
            .expect("Fetch upgradeable program");
        assert!(res.executable, "Account must be executable");
        assert!(res.data.starts_with(b"\x7FELF"), "Missing ELF header after resolution");
    }

    /// Validate behaviour on a standard Sysvar (non‑executable). Expected size is 17 bytes
    #[tokio::test]
    async fn test_fetch_non_executable_sysvar() {
        let res = fetch_account_contents(MAINNET_RPC, TEST_SYSVAR_RENT)
            .await
            .expect("Fetch Sysvar Rent");
        assert!(!res.executable, "Sysvar Rent should not be executable");
        assert!(
            !res.data.starts_with(b"\x7FELF"),
            "Sysvar Rent data should not start with ELF header"
        );

        // pub lamports_per_byte_year: u64,   // 8 bytes
        // pub exemption_threshold: f64,      // 8 bytes
        // pub burn_percent: u8,              // 1 byte
        // = 17 bytes
        assert_eq!(
            res.data.len(),
            17,
            "Sysvar Rent account size changed (expected 17 bytes, got {})",
            res.data.len()
        );
    }

    /// Ensure the function returns a readable error on an invalid pubkey
    #[tokio::test]
    async fn test_invalid_pubkey_error() {
        let _err = fetch_account_contents(MAINNET_RPC, TEST_INVALID_PUBKEY)
            .await
            .expect_err("Account not found: can't fetch any value using this pubkey, probably invalid pubkey");
    }

    /// Test to verify Anchor discriminator for an on-chain accountInfo
    #[tokio::test]
    async fn test_anchor_discriminator_for_onchain_account_info() {
        use sha2::{Digest, Sha256};

        // Account struct name according https://github.com/marinade-finance/liquid-staking-program/blob/main/programs/marinade-finance/src/state/mod.rs
        let account_name = "State";
        // This is how Anchor computes the discriminator: sha256(\"account:<name>\")
        let mut hasher = Sha256::new();
        hasher.update(format!("account:{}", account_name));
        let hash_result = hasher.finalize();
        let wanted_discriminator: [u8; 8] = hash_result[0..8].try_into().unwrap();

        let res = fetch_account_contents(MAINNET_RPC, TEST_MARINADE_STATE_ACCOUNT)
            .await
            .expect("Fetch marinade state account");
        assert!(!res.executable, "Marinade state account should not be executable");
        assert!(
            !res.data.starts_with(b"\x7FELF"),
            "Marinade state account data should not start with ELF header"
        );

        assert_eq!(hex::encode(wanted_discriminator), hex::encode(report_anchor_discriminator(res.data.as_slice())));
    }

}

