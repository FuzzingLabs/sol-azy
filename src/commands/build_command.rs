use crate::commands::build_command;
use crate::helpers;
use crate::helpers::{
    check_binary_installed, create_dir_if_not_exists, get_project_type, BeforeCheck, ProjectType,
};
use crate::state::build_state::BuildState;
use log::{debug, error, info};
use std::process::{Command, Stdio};

fn checks_before_build(target_dir: &String, out_dir: &String) -> bool {
    [
        BeforeCheck {
            error_msg: "`anchor` isn't installed".to_string(),
            result: check_binary_installed(&"anchor".to_string()),
        },
        BeforeCheck {
            error_msg: "`cargo` isn't installed".to_string(),
            result: check_binary_installed(&"cargo".to_string()),
        },
        BeforeCheck {
            error_msg: format!("Target directory {} doesn't exist", target_dir),
            result: std::path::Path::new(target_dir).exists(),
        },
        BeforeCheck {
            error_msg: format!(
                "Output directory {} doesn't exist and can't be created",
                out_dir
            ),
            result: create_dir_if_not_exists(out_dir),
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

pub fn run(target_dir: &String, out_dir: &String) -> anyhow::Result<BuildState> {
    debug!("Starting build process for {}", target_dir);

    if !checks_before_build(target_dir, out_dir) {
        error!("Can't build project, see errors above.");
        return Err(anyhow::anyhow!("Can't build project, see errors above."));
    }

    match get_project_type(target_dir) {
        ProjectType::Anchor => build_anchor_project(target_dir, out_dir),
        ProjectType::Sbf => build_sbf_project(target_dir, out_dir),
        ProjectType::Unknown => Err(anyhow::anyhow!("Unknown project type.")),
    }
}

fn build_anchor_project(target_dir: &String, out_dir: &String) -> anyhow::Result<BuildState> {
    debug!("Building anchor project {}", target_dir);

    let current_dir = std::env::current_dir()?;
    std::env::set_current_dir(target_dir)?;

    info!("Running `cargo clean` in {}", target_dir);
    let res = helpers::run_command("cargo", &["clean"], vec![]);

    std::env::set_current_dir(current_dir)?;
    res?;
    let current_dir = std::env::current_dir()?;
    std::env::set_current_dir(target_dir)?;

    info!("Running `anchor build` in {}", target_dir);
    let res = helpers::run_command(
        "anchor",
        &["build", "--skip-lint"],
        vec![(
            "RUSTFLAGS",
            "--emit=asm,llvm-bc,llvm-ir,obj,metadata,link,dep-info,mir",
        )],
    );

    std::env::set_current_dir(current_dir)?;
    res?;

    Ok(BuildState {
        name: "".to_string(),
        target_dir: target_dir.clone(),
        out_dir: out_dir.clone(),
    })
}

pub fn build_sbf_project(target_dir: &String, out_dir: &String) -> anyhow::Result<BuildState> {
    debug!("Building sbf project {}", target_dir);

    let current_dir = std::env::current_dir()?;
    std::env::set_current_dir(target_dir)?;

    info!("Running `cargo clean` in {}", target_dir);
    let res = helpers::run_command("cargo", &["clean"], vec![]);

    std::env::set_current_dir(current_dir)?;
    res?;
    let current_dir = std::env::current_dir()?;
    std::env::set_current_dir(target_dir)?;

    info!("Running `cargo build-sbf` in {}", target_dir);
    let res = helpers::run_command(
        "cargo",
        &["build-sbf"],
        vec![(
            "RUSTFLAGS",
            "--emit=asm,llvm-bc,llvm-ir,obj,metadata,link,dep-info,mir",
        )],
    );

    std::env::set_current_dir(current_dir)?;
    res?;

    Ok(BuildState {
        name: "".to_string(),
        target_dir: target_dir.clone(),
        out_dir: out_dir.clone(),
    })
}
