use crate::dotting::editor::editor_add_functions;
use crate::helpers::BeforeCheck;
use anyhow::Result;
use log::{debug, error};
use std::path::Path;

/// Verifies that all necessary files exist before performing any dotting operation.
///
/// # Arguments
///
/// * `config_path` - Path to the user-provided JSON config listing functions to be added.
/// * `reduced_path` - Path to the reduced DOT file that will be updated.
/// * `full_path` - Path to the full DOT file used as a reference.
///
/// # Returns
///
/// `true` if all files are present and accessible; `false` otherwise.
fn checks_before_dotting(config_path: &str, reduced_path: &str, full_path: &str) -> bool {
    [
        BeforeCheck {
            error_msg: format!("Config file '{}' does not exist.", config_path),
            result: Path::new(config_path).exists(),
        },
        BeforeCheck {
            error_msg: format!("Reduced dot file '{}' does not exist.", reduced_path),
            result: Path::new(reduced_path).exists(),
        },
        BeforeCheck {
            error_msg: format!("Full dot file '{}' does not exist.", full_path),
            result: Path::new(full_path).exists(),
        },
    ]
    .iter()
    .map(|check| {
        if !check.result {
            error!("{}", check.error_msg);
            return false;
        }
        true
    })
    .all(|check| check)
}

/// Runs the dotting command, which updates a reduced `.dot` file with
/// additional functions specified in a user-supplied configuration file.
///
/// # Arguments
///
/// * `config_path` - Path to the JSON configuration file containing function identifiers.
/// * `reduced_dot_path` - Path to the reduced DOT file to be edited.
/// * `full_dot_path` - Path to the full DOT file used to retrieve missing nodes/edges.
///
/// # Returns
///
/// `Ok(())` if the update is successful, or an error if any step fails.
///
/// # Errors
///
/// Returns an error if:
/// - One or more input files are missing.
/// - The configuration format is invalid.
/// - The update process fails internally.
pub fn run(config_path: String, reduced_dot_path: String, full_dot_path: String) -> Result<()> {
    debug!("Starting dotting from config '{}'", config_path);

    if !checks_before_dotting(&config_path, &reduced_dot_path, &full_dot_path) {
        return Err(anyhow::anyhow!(
            "Dotting prerequisites failed. Check that all paths exist."
        ));
    }

    editor_add_functions(config_path, reduced_dot_path, full_dot_path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_checks_before_dotting_success() {
        let config_file = "temp_config.json";
        let reduced_file = "temp_reduced.dot";
        let full_file = "temp_full.dot";

        fs::write(config_file, "{}").unwrap();
        fs::write(reduced_file, "").unwrap();
        fs::write(full_file, "").unwrap();

        assert!(checks_before_dotting(config_file, reduced_file, full_file));

        fs::remove_file(config_file).unwrap();
        fs::remove_file(reduced_file).unwrap();
        fs::remove_file(full_file).unwrap();
    }

    #[test]
    fn test_checks_before_dotting_missing_files() {
        let config_file = "missing_config.json";
        let reduced_file = "missing_reduced.dot";
        let full_file = "missing_full.dot";

        assert!(!checks_before_dotting(config_file, reduced_file, full_file));
    }
}
