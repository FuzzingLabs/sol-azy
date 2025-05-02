use crate::dotting::editor::editor_add_functions;
use crate::helpers::BeforeCheck;
use log::{error, debug, info};
use anyhow::Result;
use std::path::Path;

/// Checks that required files exist and the output directory is ready.
///
/// # Arguments
///
/// * `config_path` - Path to the JSON config file listing functions.
/// * `reduced_path` - Path to the reduced DOT file (to be updated).
/// * `full_path` - Path to the full DOT file (reference).
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

/// Main entry point for running the dotting command.
///
/// Ensures prerequisites are valid before launching the editing logic.
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
    use std::path::Path;

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