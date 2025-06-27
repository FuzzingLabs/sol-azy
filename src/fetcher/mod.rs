use anyhow::Result;
use base64::{engine::general_purpose, Engine};
use reqwest::Client;
use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use std::{fs, path::Path};

pub const MAINNET_RPC: &str = "https://api.mainnet-beta.solana.com";

/// Slice the bytecode starting at the ELF header (0x7F 'E' 'L' 'F') (removing programdata metadata things [offset = 45 in https://github.com/anza-xyz/solana-sdk/blob/master/loader-v3-interface/src/state.rs#L47])
fn slice_from_elf_header(bytecode: &[u8]) -> Option<&[u8]> {
    if bytecode.len() < 45 {
        return None;
    }
    Some(&bytecode[45..])
}

/// Fetch the bytecode of a Solana program, handling both upgradeable and non-upgradeable types.
async fn fetch_program_bytecode(rpc_url: &str, program_id: &str) -> Result<Vec<u8>, anyhow::Error> {
    let client = Client::new();

    // Step 1: fetch account info for the program ID
    let request_body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getAccountInfo",
        "params": [
            program_id,
            {
                "encoding": "base64"
            }
        ]
    });

    let res = client.post(rpc_url).json(&request_body).send().await?;

    let res_json: serde_json::Value = res.json().await?;
    let value = &res_json["result"]["value"];
    let data_base64 = value["data"][0]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("No data in account response"))?;
    let mut decoded_data = general_purpose::STANDARD.decode(data_base64)?;

    let owner = value["owner"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing owner field"))?;

    // BPFLoaderUpgradeable programs need to resolve the ProgramData account
    if owner == "BPFLoaderUpgradeab1e11111111111111111111111" {
        // Extract the ProgramData address from the upgradeable account
        if decoded_data.len() < 36 {
            return Err(anyhow::anyhow!(
                "Program account too small to contain ProgramData address"
            ));
        }

        let programdata_pubkey_bytes = &decoded_data[4..36]; // skip first 4 bytes
        let programdata_pubkey =
            Pubkey::new_from_array(programdata_pubkey_bytes.try_into().unwrap()); // will not crash since len >= 36 and it is sliced for 32 bytes
        let programdata_address = programdata_pubkey.to_string();

        // Step 2: fetch the ProgramData account
        let request_body = json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "getAccountInfo",
            "params": [
                programdata_address,
                {
                    "encoding": "base64"
                }
            ]
        });

        let res = client.post(rpc_url).json(&request_body).send().await?;

        let res_json: serde_json::Value = res.json().await?;
        let value = &res_json["result"]["value"];
        let data_base64 = value["data"][0]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("No data in ProgramData response"))?;
        decoded_data = general_purpose::STANDARD.decode(data_base64)?;

        if decoded_data.len() <= 8 {
            return Err(anyhow::anyhow!("ProgramData too short"));
        }
    }

    if let Some(elf_data) = slice_from_elf_header(&decoded_data) {
        Ok(elf_data.to_vec())
    } else {
        Err(anyhow::anyhow!(
            "Wrong bytecode (should have at least 45 bytes of metadata)"
        ))
    }
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
pub async fn fetch_bytecode_to<P: AsRef<Path>>(
    out_dir: P,
    rpc_url: Option<String>,
    program_id: &str,
) -> Result<(), anyhow::Error> {
    let rpc_url = rpc_url.unwrap_or_else(|| MAINNET_RPC.to_string());
    let result = fetch_program_bytecode(&rpc_url, program_id).await?;

    let output_path = out_dir.as_ref().join("fetched_program.so");
    fs::write(&output_path, result)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_program_bytecode_ok() {
        let _: Vec<u8> =
            fetch_program_bytecode(MAINNET_RPC, "4MangoMjqJ2firMokCjjGgoK8d4MXcrgL7XJaL3w6fVg")
                .await
                .expect("Should succeed");
    }
}
