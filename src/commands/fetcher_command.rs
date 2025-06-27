use crate::fetcher::fetch_bytecode_to;
use crate::fetcher::MAINNET_RPC;
use anyhow::Result;
use log::{debug, error};
use reqwest::Client;
use serde_json::json;
use std::path::Path;

/// Represents possible validation errors when preparing to fetch a program's bytecode.
///
/// This enum is used by the `checks_before_fetch` function to signal distinct failure modes
/// before performing a fetch operation:
/// - creation error for output directory,
/// - nonexistent program ID on-chain,
/// - or a non-executable program account.
#[derive(thiserror::Error, Debug)]
enum FetchPrecheckError {
    /// Could not create the specified output directory.
    #[error("Failed to create output directory '{0}'.")]
    OutputDirCreationFailed(String),

    /// The provided program ID does not correspond to any account on the Solana blockchain.
    #[error("Program ID '{0}' does not exist on-chain.")]
    ProgramAccountNotFound(String),

    /// The account exists but is not marked as executable.
    #[error("Program ID '{0}' exists but is not executable.")]
    ProgramNotExecutable(String),
}

/// Validates all necessary preconditions before attempting to fetch a Solana program's bytecode.
///
/// This includes:
/// - Creating the output directory if it does not exist.
/// - Verifying that the provided `program_id` exists on-chain via RPC.
/// - Ensuring the program account is marked as executable.
///
/// # Returns
///
/// * `Ok(())` if all checks pass.
/// * `Err(FetchPrecheckError)` if any validation fails.
async fn checks_before_fetch(
    out_dir: &str,
    program_id: &str,
    rpc_url: &str,
) -> Result<(), FetchPrecheckError> {
    let out_path = Path::new(out_dir);
    if !out_path.is_dir() {
        std::fs::create_dir_all(out_path)
            .map_err(|_| FetchPrecheckError::OutputDirCreationFailed(out_dir.to_string()))?;
        debug!("Output directory '{}' created successfully.", out_dir);
    }

    let client = Client::new();

    let request_body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getAccountInfo",
        "params": [
            program_id,
            { "encoding": "jsonParsed" }
        ]
    });

    let res = client
        .post(rpc_url)
        .json(&request_body)
        .send()
        .await
        .map_err(|_| FetchPrecheckError::ProgramAccountNotFound(program_id.to_string()))?;

    let res_json: serde_json::Value = res
        .json()
        .await
        .map_err(|_| FetchPrecheckError::ProgramAccountNotFound(program_id.to_string()))?;

    let account = &res_json["result"]["value"];
    if account.is_null() {
        return Err(FetchPrecheckError::ProgramAccountNotFound(
            program_id.to_string(),
        ));
    }

    let executable = account["executable"].as_bool().unwrap_or(false);
    if !executable {
        return Err(FetchPrecheckError::ProgramNotExecutable(
            program_id.to_string(),
        ));
    }

    Ok(())
}

/// Runs the fetcher command to download bytecode of a program from the Solana blockchain.
///
/// This function validates the program's existence, ensures the output directory exists
/// (creating it if necessary), and writes the bytecode to `<out_dir>/fetched_program.so`.
///
/// # Arguments
///
/// * `program_id` - The Solana program ID to fetch.
/// * `out_dir` - Directory where `fetched_program.so` will be written.
/// * `rpc_url` - Optional Solana RPC endpoint. If `None`, defaults to mainnet.
///
/// # Returns
///
/// * `Ok(())` if fetching and writing succeed.
/// * `Err(anyhow::Error)` if the program doesn't exist, isn't executable,
///   the RPC fails, or the output file can't be written.
pub async fn run(
    program_id: String,
    out_dir: String,
    rpc_url: Option<String>,
) -> anyhow::Result<()> {
    let rpc_url_unwrapped = rpc_url.clone().unwrap_or_else(|| MAINNET_RPC.to_string());

    debug!("Starting fetch for program ID '{}'", program_id);

    match checks_before_fetch(&out_dir, &program_id, &rpc_url_unwrapped).await {
        Ok(_) => {} // continue
        Err(FetchPrecheckError::OutputDirCreationFailed(dir)) => {
            return Err(anyhow::anyhow!(
                "Failed to create output directory '{}'",
                dir
            ));
        }
        Err(FetchPrecheckError::ProgramAccountNotFound(pid)) => {
            error!("Program ID not found on-chain: {}", pid);
            return Err(anyhow::anyhow!("Program '{}' not found on-chain", pid));
        }
        Err(FetchPrecheckError::ProgramNotExecutable(pid)) => {
            error!("Program exists but is not executable: {}", pid);
            return Err(anyhow::anyhow!("Program '{}' is not executable", pid));
        }
    }

    fetch_bytecode_to(&out_dir, Some(rpc_url_unwrapped.clone()), &program_id).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[tokio::test]
    async fn test_check_program_id_missing_on_chain() {
        let out_dir = "temp_test_dir_missing_program";
        fs::create_dir_all(out_dir).unwrap();

        let fake_program = "Missing11111111111111111111111111111111111111";

        let result = checks_before_fetch(out_dir, fake_program, MAINNET_RPC).await;
        assert!(matches!(
            result,
            Err(FetchPrecheckError::ProgramAccountNotFound(_))
        ));

        fs::remove_dir_all(out_dir).unwrap();
    }

    #[tokio::test]
    async fn test_check_program_not_executable() {
        let out_dir = "temp_test_dir_not_exec";
        fs::create_dir_all(out_dir).unwrap();

        // A known non-executable account on-chain (e.g., a system account or buffer)
        let non_exec_account = "SysvarC1ock11111111111111111111111111111111"; // Clock sysvar is not executable

        let result = checks_before_fetch(out_dir, non_exec_account, MAINNET_RPC).await;
        assert!(matches!(
            result,
            Err(FetchPrecheckError::ProgramNotExecutable(_))
        ));

        fs::remove_dir_all(out_dir).unwrap();
    }

    #[tokio::test]
    async fn test_check_success_with_valid_program() {
        let out_dir = "temp_test_dir_success";
        fs::create_dir_all(out_dir).unwrap();

        let valid_program = "4MangoMjqJ2firMokCjjGgoK8d4MXcrgL7XJaL3w6fVg"; // Mango V4 proxy program (randomly choosen)

        let result = checks_before_fetch(out_dir, valid_program, MAINNET_RPC).await;
        assert!(result.is_ok());

        fs::remove_dir_all(out_dir).unwrap();
    }
}
