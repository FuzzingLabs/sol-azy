pub mod static_dir;

use std::{fmt, fs, path::Path, process::Command};
use std::fmt::Formatter;
use toml::Value;
use log::{debug, error, info};
use std::process::Stdio;
use crate::commands::build_command;
use crate::state::build_state::BuildState;

pub fn check_binary_installed(bin_name: &String) -> bool {
    Command::new("which")
        .arg(bin_name)
        .output()
        .map_or(false, |output| output.status.success())
}

pub fn create_dir_if_not_exists(dir: &String) -> bool {
    let path = Path::new(dir);
    if path.exists() {
        return true;
    }
    fs::create_dir_all(path).is_ok()
}

#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub enum ProjectType {
    Anchor,
    Sbf,
    Unknown,
}

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

pub struct BeforeCheck {
    pub error_msg: String,
    pub result: bool,
}

pub fn run_command(command_name: &str, args: &[&str], env_vars: Vec<(&str, &str)>) -> Result<String, anyhow::Error> {
    let mut bind = Command::new(command_name);
    let mut command = bind.args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    for (key, value) in env_vars {
        command.env(key, value);
    }

    let output = command.output()
        .map_err(|e| anyhow::anyhow!("Failed to run `{}`: {}", command_name, e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("Error while running `{}`\nStderr:\n{}", command_name, stderr);
        return Err(anyhow::anyhow!("Error while running `{}`. Check the logs above for details.", command_name));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    debug!("Command output:\n{}", stdout);

    Ok(stdout.into())
}