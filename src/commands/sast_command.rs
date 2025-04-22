use std::cmp::PartialEq;
use crate::commands::build_command;
use crate::commands::build_command::build_sbf_project;
use crate::helpers::{
    check_binary_installed, create_dir_if_not_exists, get_project_type, BeforeCheck, ProjectType,
};
use crate::parsers::syn_ast;
use crate::state::build_state::BuildState;
use crate::state::sast_state::{SastState, SynAstMap, SynAstMapExt};
use crate::{helpers, Commands};
use log::{debug, error, info};
use std::process::{Command, Stdio};

pub struct SastCmd {
    pub target_dir: String,
    pub rules_dir: String,
    pub syn_scan_only: bool,
    pub recursive: bool,
    // TODO: use Build out-dir in options
}

impl SastCmd {
    pub fn new_from_clap(cmd: &Commands) -> Self {
        match cmd {
            Commands::Sast {
                target_dir,
                rules_dir,
                syn_scan_only,
                recursive,
            } => Self {
                target_dir: target_dir.clone(),
                rules_dir: rules_dir.clone(),
                syn_scan_only: *syn_scan_only,
                recursive: *recursive,
            },
            _ => unreachable!(),
        }
    }
}

fn checks_before_sast(cmd: &SastCmd) -> bool {
    [
        BeforeCheck {
            error_msg: format!("Target directory {} doesn't exist", cmd.target_dir),
            result: std::path::Path::new(&cmd.target_dir).exists(),
        },
        BeforeCheck {
            error_msg: format!("Rules directory {} doesn't exist", cmd.rules_dir),
            result: std::path::Path::new(&cmd.rules_dir).exists(),
        },
    ]
    .iter()
    .map(|check| {
        if !check.result {
            error!("{}", check.error_msg);
            return false;
        }
        return true;
    })
    .all(|check| check)
}

pub fn run(cmd: &SastCmd) -> anyhow::Result<Vec<SastState>> {
    debug!("Starting SAST process for {}", cmd.target_dir);

    if !checks_before_sast(cmd) {
        error!(
            "Can't launch SAST on directory {}, see errors above.",
            cmd.target_dir
        );
        return Err(anyhow::anyhow!(
            "Can't launch SAST on directory {}, see errors above.",
            cmd.target_dir
        ));
    }

    if cmd.recursive {
        scan_directory_recursively(cmd)
    } else {
        match get_project_type(&cmd.target_dir) {
            ProjectType::Anchor => Ok(vec![sast_anchor_project(cmd)?]),
            ProjectType::Sbf => Ok(vec![sast_sbf_project(cmd)?]),
            ProjectType::Unknown => Err(anyhow::anyhow!("Unknown project type.")),
        }
    }
}

fn scan_directory_recursively(cmd: &SastCmd) -> anyhow::Result<Vec<SastState>> {
    let mut results = Vec::new();
    let path = std::path::Path::new(&cmd.target_dir);

    // Skip certain directories commonly not needed for scanning
    let dir_name = path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");

    if dir_name.starts_with(".") ||
        dir_name == "node_modules" ||
        dir_name == "target" ||
        dir_name == "build" {
        return Ok(results);
    }

    // Check if the current directory is a project
    let project_type = get_project_type(&cmd.target_dir);
    if project_type != ProjectType::Unknown {
        info!("Found {} project at {}", project_type, cmd.target_dir);
        let result = match project_type {
            ProjectType::Anchor => sast_anchor_project(cmd)?,
            ProjectType::Sbf => sast_sbf_project(cmd)?,
            ProjectType::Unknown => unreachable!(),
        };
        results.push(result);
    }

    // Always check subdirectories if recursion is enabled
    if path.is_dir() && cmd.recursive {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let sub_path = entry.path();

            if sub_path.is_dir() {
                let sub_cmd = SastCmd {
                    target_dir: sub_path.to_string_lossy().to_string(),
                    rules_dir: cmd.rules_dir.clone(),
                    syn_scan_only: cmd.syn_scan_only,
                    recursive: true,
                };

                // Continue recursion with subdirectories
                let sub_results = scan_directory_recursively(&sub_cmd)?;
                results.extend(sub_results);
            }
        }
    }

    Ok(results)
}



fn sast_anchor_project(cmd: &SastCmd) -> anyhow::Result<SastState> {
    // ? FUTURE: Use Anchor.toml to get programs paths?
    let mut sast_state = SastState::new(
        syn_ast::get_syn_ast_recursive(&format!("{}/programs", cmd.target_dir))?,
        &cmd.rules_dir,
    )?;

    match sast_state.apply_rules() {
        Ok(_) => {}
        Err(e) => {
            error!("Cannot apply rules to the project: {}", cmd.target_dir);
            return Err(anyhow::anyhow!(
                "Cannot apply rules to the project: {}",
                cmd.target_dir
            ));
        }
    }

    if cmd.syn_scan_only {
        return Ok(sast_state);
    }
    Ok(sast_state)
}

fn sast_sbf_project(cmd: &SastCmd) -> anyhow::Result<SastState> {
    // ? FUTURE: Use Cargo.toml to get programs paths?
    let mut sast_state = SastState::new(
        syn_ast::get_syn_ast_recursive(&format!("{}/src", cmd.target_dir))?,
        &cmd.rules_dir,
    )?;

    match sast_state.apply_rules() {
        Ok(_) => {}
        Err(e) => {
            error!("Cannot apply rules to the project: {}", cmd.target_dir);
            return Err(anyhow::anyhow!(
                "Cannot apply rules to the project: {}",
                cmd.target_dir
            ));
        }
    }

    sast_state.print_results()?;

    if cmd.syn_scan_only {
        return Ok(sast_state);
    }
    Ok(sast_state)
}
