use crate::helpers::{
    check_binary_installed, create_dir_if_not_exists, get_project_type, BeforeCheck, ProjectType,
};
use crate::state::build_state::BuildState;
use crate::{helpers, Commands};
use log::{debug, error, info};


pub struct BuildCmd {
    pub target_dir: String,
    pub out_dir: String,
}

impl BuildCmd {
    pub fn new_from_clap(cmd: &Commands) -> Self {
        match cmd {
            Commands::Build {
                target_dir,
                out_dir,
            } => Self {
                target_dir: target_dir.clone(),
                out_dir: out_dir.clone(),
            },
            _ => unreachable!(),
        }
    }
}

/// Runs a series of preconditions before attempting to build the project.
///
/// This includes checking for required binaries (`anchor`, `cargo`),
/// verifying that the target directory exists, and that the output
/// directory exists or can be created.
///
/// # Arguments
///
/// * `target_dir` - The path to the project to be built.
/// * `out_dir` - The output directory for the build artifacts.
///
/// # Returns
///
/// `true` if all checks passed, otherwise `false`.
fn checks_before_build(cmd: &BuildCmd) -> bool {
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
            error_msg: format!("Target directory {} doesn't exist", cmd.target_dir),
            result: std::path::Path::new(&cmd.target_dir).exists(),
        },
        BeforeCheck {
            error_msg: format!(
                "Output directory {} doesn't exist and can't be created",
                cmd.out_dir
            ),
            result: create_dir_if_not_exists(&cmd.out_dir),
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

/// Main entry point to build a project, automatically selecting the build process
/// based on the project type (Anchor or raw SBF).
///
/// # Arguments
///
/// * `target_dir` - The path to the project to build.
/// * `out_dir` - The path to output build artifacts.
///
/// # Returns
///
/// A `BuildState` on success, or an error if the build fails or the project type is unknown.
pub fn run(cmd: &BuildCmd) -> anyhow::Result<BuildState> {
    debug!("Starting build process for {}", cmd.target_dir);

    if !checks_before_build(cmd) {
        error!("Can't build project, see errors above.");
        return Err(anyhow::anyhow!("Can't build project, see errors above."));
    }

    match get_project_type(&cmd.target_dir) {
        ProjectType::Anchor => build_anchor_project(cmd),
        ProjectType::Sbf => build_sbf_project(cmd),
        ProjectType::Unknown => Err(anyhow::anyhow!("Unknown project type.")),
    }
}

/// Builds a project using the Anchor framework by running `anchor build`.
///
/// This function sets the working directory, cleans previous build artifacts,
/// and then runs the Anchor CLI tool with appropriate `RUSTFLAGS`.
///
/// # Arguments
///
/// * `target_dir` - The path to the Anchor project.
/// * `out_dir` - The output directory for build artifacts.
///
/// # Returns
///
/// A `BuildState` object if the build is successful, or an error otherwise.
fn build_anchor_project(cmd: &BuildCmd) -> anyhow::Result<BuildState> {
    debug!("Building anchor project {}", cmd.target_dir);

    let current_dir = std::env::current_dir()?;
    std::env::set_current_dir(cmd.target_dir.clone())?;

    info!("Running `cargo clean` in {}", cmd.target_dir);
    let res = helpers::run_command("cargo", &["clean"], vec![]);

    std::env::set_current_dir(current_dir)?;
    res?;
    let current_dir = std::env::current_dir()?;
    std::env::set_current_dir(cmd.target_dir.clone())?;

    info!("Running `anchor build` in {}", cmd.target_dir);
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
        target_dir: cmd.target_dir.clone(),
        out_dir: cmd.out_dir.clone(),
    })
}

/// Builds a raw Solana SBF project using `cargo build-sbf`.
///
/// Similar to the Anchor build process, this resets the environment,
/// performs a clean, and invokes the build with specific `RUSTFLAGS`.
///
/// # Arguments
///
/// * `target_dir` - The path to the SBF project.
/// * `out_dir` - The output directory for build artifacts.
///
/// # Returns
///
/// A `BuildState` object if the build is successful, or an error otherwise.
pub fn build_sbf_project(cmd: &BuildCmd) -> anyhow::Result<BuildState> {
    debug!("Building sbf project {}", cmd.target_dir);

    let current_dir = std::env::current_dir()?;
    std::env::set_current_dir(cmd.target_dir.clone())?;

    info!("Running `cargo clean` in {}", cmd.target_dir);
    let res = helpers::run_command("cargo", &["clean"], vec![]);

    std::env::set_current_dir(current_dir)?;
    res?;
    let current_dir = std::env::current_dir()?;
    std::env::set_current_dir(cmd.target_dir.clone())?;

    info!("Running `cargo build-sbf` in {}", cmd.target_dir);
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
        target_dir: cmd.target_dir.clone(),
        out_dir: cmd.out_dir.clone(),
    })
}
