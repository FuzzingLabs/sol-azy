use crate::commands::build_command;
use crate::commands::build_command::build_sbf_project;
use crate::helpers;
use crate::helpers::{
    check_binary_installed, create_dir_if_not_exists, get_project_type, BeforeCheck, ProjectType,
};
use crate::parsers::syn_ast;
use crate::state::build_state::BuildState;
use crate::state::sast_state::{SastState, SynAstMap, SynAstMapExt};
use log::{debug, error, info};
use std::process::{Command, Stdio};

fn checks_before_sast(target_dir: &String, rules_dir: &String, syn_scan_only: bool) -> bool {
    [
        BeforeCheck {
            error_msg: format!("Target directory {} doesn't exist", target_dir),
            result: std::path::Path::new(target_dir).exists(),
        },
        BeforeCheck {
            error_msg: format!("Rules directory {} doesn't exist", rules_dir),
            result: std::path::Path::new(rules_dir).exists(),
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

pub fn run(
    target_dir: &String,
    rules_dir: &String,
    syn_scan_only: bool,
) -> anyhow::Result<SastState> {
    debug!("Starting build process for {}", target_dir);

    if !checks_before_sast(target_dir, rules_dir, syn_scan_only) {
        error!(
            "Can't launch sast on project {}, see errors above.",
            target_dir
        );
        return Err(anyhow::anyhow!(
            "Can't launch sast on project {}, see errors above.",
            target_dir
        ));
    }

    match get_project_type(target_dir) {
        ProjectType::Anchor => sast_anchor_project(target_dir, rules_dir, syn_scan_only),
        ProjectType::Sbf => sast_sbf_project(target_dir, rules_dir, syn_scan_only),
        ProjectType::Unknown => Err(anyhow::anyhow!("Unknown project type.")),
    }
}

fn sast_anchor_project(
    target_dir: &String,
    rules_dir: &String,
    syn_scan_only: bool,
) -> anyhow::Result<SastState> {
    // ? FUTURE: Use Anchor.toml to get programs paths?
    let mut sast_state = SastState::new(
        syn_ast::get_syn_ast_recursive(&format!("{}/programs", target_dir))?,
        rules_dir
    )?;

    match sast_state.apply_rules() {
        Ok(_) => {}
        Err(e) => {
            error!("Cannot apply rules to the project: {}", target_dir);
            return Err(anyhow::anyhow!("Cannot apply rules to the project: {}", target_dir));
        }
    }

    sast_state.print_results()?;

    if syn_scan_only {
        return Ok(sast_state);
    }
    Ok(sast_state)
}

fn sast_sbf_project(
    target_dir: &String,
    rules_dir: &String,
    syn_scan_only: bool,
) -> anyhow::Result<SastState> {
    // ? FUTURE: Use Cargo.toml to get programs paths?
    let mut sast_state = SastState::new(
        syn_ast::get_syn_ast_recursive(&format!("{}/src", target_dir))?,
        rules_dir
    )?;

    match sast_state.apply_rules() {
        Ok(_) => {}
        Err(e) => {
            error!("Cannot apply rules to the project: {}", target_dir);
            return Err(anyhow::anyhow!("Cannot apply rules to the project: {}", target_dir));
        }
    }

    sast_state.print_results()?;

    if syn_scan_only {
        return Ok(sast_state);
    }
    Ok(sast_state)
}
