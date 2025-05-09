use crate::reverse::{analyze_program, ReverseOutputMode};
use crate::helpers::BeforeCheck;
use log::{error, debug, info};
use anyhow::Result;


/// Verifies that the required files and directories exist before running reverse analysis.
///
/// If the output directory does not exist, it will attempt to create it.
/// Logs all outcomes.
///
/// # Arguments
///
/// * `bytecodes_file` - Path to the compiled eBPF bytecode (.so file).
/// * `out_dir` - Directory where output files should be written.
///
/// # Returns
///
/// `true` if all checks pass, `false` otherwise.
fn checks_before_reverse(bytecodes_file: &String, out_dir: &String) -> bool {
    let checks_passed = [
        BeforeCheck {
            error_msg: format!("Target bytecodes file '{}' does not exist.", bytecodes_file),
            result: std::path::Path::new(bytecodes_file).exists(),
        },
        // .. could add some checks like verifying or logging if that's well formatted binary or if it is stripped ... (not so relevant atm)
    ]
    .iter()
    .map(|check| {
        if !check.result {
            error!("{}", check.error_msg);
            return false;
        }
        true
    })
    .all(|check| check);

    if !checks_passed {
        return false;
    }

    let out_dir_path = std::path::Path::new(out_dir);
    if !out_dir_path.exists() {
        match std::fs::create_dir_all(out_dir_path) {
            Ok(_) => {
                info!("Output directory '{}' created successfully.", out_dir);
            }
            Err(e) => {
                error!("Failed to create output directory '{}': {}", out_dir, e);
                return false;
            }
        }
    } else {
        debug!("Output directory '{}' already exists.", out_dir);
    }

    true
}

/// Dispatches the reverse engineering workflow based on a user-specified mode.
///
/// Converts a string-based mode (`"disass"`, `"cfg"`, `"both"`)
/// into a `ReverseOutputMode` enum and calls `analyze_program` accordingly.
///
/// # Arguments
///
/// * `mode` - A string indicating which analysis mode to use.
/// * `out_dir` - The path to the directory where output files will be written.
/// * `bytecodes_file` - Path to the compiled eBPF bytecode (.so file).
/// * `labeling` - Whether to enable symbol and section labeling in the analysis.
/// * `reduced` - If enabled, limits CFG generation to functions defined after the program entrypoint,
///   which helps reduce noise from unrelated or prelinked functions in the bytecode.
/// * `only_entrypoint` - If true, generates a minimal CFG containing only the entrypoint function (`cluster_{entry}`), 
///   allowing manual expansion afterward using tools like the `dotting` module.
///
/// # Returns
///
/// A `Result<()>` that is `Ok` if the analysis succeeded, or an error if the mode was unknown
/// or analysis failed.
///
/// # Errors
///
/// Returns an error if the provided `mode` string does not match any known `ReverseOutputMode`,
/// or if the reverse analysis fails to initialize properly.
pub fn run(mode: String, out_dir: String, bytecodes_file: String, labeling: bool, reduced: bool, only_entrypoint: bool) -> Result<()> {
    debug!("Starting reverse process for {}", bytecodes_file);

    if !checks_before_reverse(&bytecodes_file, &out_dir) {
        error!(
            "Can't launch reverse analysis on '{}', see errors above.",
            bytecodes_file
        );
        return Err(anyhow::anyhow!(
            "Can't launch reverse analysis on '{}', see errors above.",
            bytecodes_file
        ));
    }

    let output_mode = match mode.as_str() {
        "disass" => ReverseOutputMode::Disassembly(out_dir),
        "cfg" => ReverseOutputMode::ControlFlowGraph(out_dir),
        "both" => ReverseOutputMode::DisassemblyAndCFG(out_dir),
        other => {
            return Err(anyhow::anyhow!("Unknown reverse mode: {}", other));
        }
    };

    analyze_program(output_mode, bytecodes_file, labeling, reduced, only_entrypoint)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_checks_before_reverse_success() {
        // Create a temporary bytecode file
        let temp_bytecode = "temp_test_bytecode.so";
        fs::write(temp_bytecode, b"dummy").unwrap();

        // Define a non-existing output directory
        let temp_output_dir = "temp_test_output_dir";

        if Path::new(temp_output_dir).exists() {
            error!(
                "Can't do the unit test because '{}' already exists.",
                temp_output_dir
            );
        }

        assert!(checks_before_reverse(
            &temp_bytecode.to_string(),
            &temp_output_dir.to_string()
        ));

        // Output dir should exist now
        assert!(Path::new(temp_output_dir).exists());

        // Clean up
        fs::remove_file(temp_bytecode).unwrap();
        fs::remove_dir_all(temp_output_dir).unwrap();
    }

    #[test]
    fn test_checks_before_reverse_bytecode_missing() {
        let missing_bytecode = "missing_bytecode.so";
        let temp_output_dir = "temp_output_should_not_be_created";

        // Make sure file and directory do not exist
        if Path::new(missing_bytecode).exists() {
            fs::remove_file(missing_bytecode).unwrap();
        }
        if Path::new(temp_output_dir).exists() {
            fs::remove_dir_all(temp_output_dir).unwrap();
        }

        // Should fail (false) because the bytecode file is missing
        assert!(!checks_before_reverse(
            &missing_bytecode.to_string(),
            &temp_output_dir.to_string()
        ));

        // Output dir should still NOT exist
        assert!(!Path::new(temp_output_dir).exists());
    }
}
