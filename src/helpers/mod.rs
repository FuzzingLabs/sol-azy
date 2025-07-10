//! General-purpose helper functions and types used across the project.
//!
//! This module provides utility functionality for:
//! - Checking the presence of required binaries (`check_binary_installed`)
//! - Creating directories (`create_dir_if_not_exists`)
//! - Detecting project type (Anchor vs SBF)
//! - Running shell commands with optional environment variables (`run_command`)
//!
//! It also defines helper types like `ProjectType` and `BeforeCheck` used in build and analysis workflows.

pub mod static_dir;
pub mod spinner;

use log::{debug, error};
use std::fmt::Formatter;
use std::process::Stdio;
use std::{fmt, fs, path::Path, process::Command};
use toml::Value;

/// Checks if a binary is available in the system's `$PATH`.
///
/// # Arguments
///
/// * `bin_name` - Name of the binary to check (e.g., `"cargo"`, `"anchor"`).
///
/// # Returns
///
/// `true` if the binary is found, otherwise `false`.
pub fn check_binary_installed(bin_name: &String) -> bool {
    Command::new("which")
        .arg(bin_name)
        .output()
        .map_or(false, |output| output.status.success())
}

/// Ensures a directory exists, creating it if necessary.
///
/// # Arguments
///
/// * `dir` - Path to the directory.
///
/// # Returns
///
/// `true` if the directory exists or was created successfully, otherwise `false`.
pub fn create_dir_if_not_exists(dir: &String) -> bool {
    let path = Path::new(dir);
    if path.exists() {
        return true;
    }
    fs::create_dir_all(path).is_ok()
}

/// Enum representing the detected type of Solana-based project.
///
/// - `Anchor`: Project contains an `Anchor.toml` file.
/// - `Sbf`: Project is identified as a native Solana SBF crate.
/// - `Unknown`: Type could not be determined.
#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub enum ProjectType {
    Anchor,
    Sbf,
    Unknown,
}

/// Attempts to determine the type of Solana project based on its configuration files.
///
/// Checks for presence of `Anchor.toml` or a `Cargo.toml` containing a `solana-program` dependency.
///
/// # Arguments
///
/// * `project_dir` - Path to the root of the project.
///
/// # Returns
///
/// A `ProjectType` variant (`Anchor`, `Sbf`, or `Unknown`).
impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ProjectType::Anchor => write!(f, "Anchor"),
            ProjectType::Sbf => write!(f, "Solana BPF"),
            ProjectType::Unknown => write!(f, "Unknown"),
        }
    }
}

pub fn get_project_type(project_dir: &String) -> ProjectType {
    let anchor_toml = Path::new(project_dir).join("Anchor.toml");
    if anchor_toml.exists() {
        return ProjectType::Anchor;
    }

    let cargo_toml = Path::new(project_dir).join("Cargo.toml");
    fs::read_to_string(cargo_toml)
        .ok()
        .and_then(|content| content.parse::<Value>().ok())
        .and_then(|parsed| parsed.get("dependencies").cloned())
        .and_then(|dependencies| dependencies.get("solana-program").cloned())
        .map_or(ProjectType::Unknown, |_| ProjectType::Sbf)
}

/// Represents a single pre-check step before a build or analysis,
/// consisting of an error message and a success result.
pub struct BeforeCheck {
    pub error_msg: String,
    pub result: bool,
}

/// Executes a command with given arguments and optional environment variables.
///
/// Captures and returns the standard output on success, or logs and returns an error on failure.
///
/// # Arguments
///
/// * `command_name` - Name of the command to run (e.g., `"cargo"`).
/// * `args` - List of arguments to pass to the command.
/// * `env_vars` - Optional list of environment variables to set for the command.
///
/// # Returns
///
/// A `Result<String>` containing the command's stdout if successful, or an error.
pub fn run_command(
    command_name: &str,
    args: &[&str],
    env_vars: Vec<(&str, &str)>,
) -> Result<String, anyhow::Error> {
    let mut bind = Command::new(command_name);
    let command = bind
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    for (key, value) in env_vars {
        command.env(key, value);
    }

    let output = command
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to run `{}`: {}", command_name, e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!(
            "Error while running `{}`\nStderr:\n{}",
            command_name, stderr
        );
        return Err(anyhow::anyhow!(
            "Error while running `{}`. Check the logs above for details.",
            command_name
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    debug!("Command output:\n{}", stdout);

    Ok(stdout.into())
}

/// Switches the Anchor CLI version by installing a specific version from the official repository.
///
/// This function executes `cargo install --git https://github.com/coral-xyz/anchor --tag vXXX anchor-cli --force`
/// where XXX is the provided version parameter.
///
/// # Arguments
///
/// * `version` - The version string to install (e.g., "0.31.0"). The "v" prefix will be added automatically.
///
/// # Returns
///
/// Returns `Ok(String)` with the command output on success, or an `anyhow::Error` on failure.
///
/// # Example
///
/// ```rust
/// // Install Anchor CLI version 0.31.0
/// switch_anchor_version("0.31.0")?;
/// ```
pub fn switch_anchor_version(version: &str) -> Result<String, anyhow::Error> {
    let command_name = "cargo";
    let args = &[
        "install",
        "--git",
        "https://github.com/coral-xyz/anchor",
        "--tag",
        &format!("v{}", version),
        "anchor-cli",
        "--force",
    ];
    let env_vars = vec![];

    run_command(command_name, args, env_vars)
}

/// Retrieves the Anchor version from an Anchor.toml file in the specified directory.
///
/// This function looks for an `Anchor.toml` file in the given path and attempts to parse
/// the anchor version from the `[toolchain]` section using generic TOML parsing.
///
/// # Arguments
///
/// * `project_path` - Path to the directory containing the Anchor.toml file
///
/// # Returns
///
/// Returns `Ok(Some(String))` with the anchor version if found, `Ok(None)` if no version
/// is specified in the file, or an `anyhow::Error` if the file doesn't exist or cannot be parsed.
///
/// # Example
///
/// ```rust
/// use std::path::Path;
///
/// // Get anchor version from current directory
/// let version = get_anchor_version(Path::new("."))?;
/// match version {
///     Some(v) => println!("Anchor version: {}", v),
///     None => println!("No anchor version specified in Anchor.toml"),
/// }
/// ```
pub fn get_anchor_version(project_path: &Path) -> Result<Option<String>, anyhow::Error> {
    let anchor_toml_path = project_path.join("Anchor.toml");

    if !anchor_toml_path.exists() {
        return Err(anyhow::anyhow!("Anchor.toml file not found in {}", project_path.display()));
    }

    let content = fs::read_to_string(&anchor_toml_path)
        .map_err(|e| anyhow::anyhow!("Failed to read {}: {}", anchor_toml_path.display(), e))?;

    let value = toml::from_str::<Value>(&content)
        .map_err(|e| anyhow::anyhow!("Failed to parse Anchor.toml: {}", e))?;

    if let Some(toolchain) = value.get("toolchain").and_then(|t| t.as_table()) {
        if let Some(anchor_ver) = toolchain.get("anchor_version").and_then(|v| v.as_str()) {
            return Ok(Some(anchor_ver.to_string()));
        }
    }

    Ok(None)
}