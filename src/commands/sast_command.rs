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

/// Runs a series of checks before launching SAST analysis.
///
/// Verifies that the target project directory and rules directory exist.
///
/// # Arguments
///
/// * `target_dir` - Path to the project to be analyzed.
/// * `rules_dir` - Path to the directory containing SAST rules.
/// * `syn_scan_only` - If true, only perform syntactic scanning (no build required).
///
/// # Returns
///
/// `true` if all checks passed, otherwise `false`.
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

/// Launches the static analysis (SAST) workflow on the given project using the provided rules.
///
/// Automatically detects the project type and dispatches to the appropriate SAST handler.
///
/// # Arguments
///
/// * `target_dir` - The path to the project root directory.
/// * `rules_dir` - The directory where rule definitions are stored.
/// * `syn_scan_only` - If true, only perform syntax tree analysis without full project build.
///
/// # Returns
///
/// A `SastState` object on success, or an error if any checks fail or the project type is unsupported.
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

/// Performs static analysis on an Anchor-based project using rule files.
///
/// Syntax trees are generated from the `programs/` directory. If `syn_scan_only` is false,
/// this function could later support additional build-based analysis.
///
/// # Arguments
///
/// * `target_dir` - The path to the root of the Anchor project.
/// * `rules_dir` - The path to the rule definitions directory.
/// * `syn_scan_only` - If true, skips any future deep analysis beyond syntax trees.
///
/// # Returns
///
/// A populated `SastState` if analysis succeeds, or an error if rule application fails.
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

/// Performs static analysis on an SBF (non-Anchor) project using rule files.
///
/// Syntax trees are generated from the `src/` directory. If `syn_scan_only` is false,
/// this function could be extended to support build-time inspection.
///
/// # Arguments
///
/// * `target_dir` - The path to the root of the SBF project.
/// * `rules_dir` - The path to the rule definitions directory.
/// * `syn_scan_only` - If true, skips deeper analysis stages.
///
/// # Returns
///
/// A `SastState` if the rule application and syntax scanning succeed, or an error otherwise.
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
