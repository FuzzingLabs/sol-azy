use crate::state::build_state::BuildState;
use crate::state::sast_state::SastState;
use crate::{commands, Cli, Commands};
use log::{error, info};

/// Represents the global application state, including parsed CLI options and collected results.
///
/// This struct drives the main execution logic for CLI subcommands like build, reverse analysis,
/// and static analysis (SAST). It stores results for each executed command for potential post-processing or reporting.
pub struct AppState {
    pub cli: Cli,
    pub build_states: Vec<BuildState>,
    pub sast_states: Vec<SastState>,
}

impl AppState {
    /// Dispatches CLI command execution based on the parsed `Cli` structure.
    ///
    /// This method matches the provided subcommand and calls the corresponding handler
    /// for build, reverse analysis, or SAST.
    ///
    /// # Behavior
    ///
    /// If no command is matched, it logs a message without performing any action.
    pub fn run_cli(&mut self) {
        match &self.cli.command {
            Commands::Build {
                target_dir,
                out_dir,
            } => self.build_project(target_dir.clone(), out_dir.clone()),
            Commands::Sast {
                target_dir,
                rules_dir,
                syn_scan_only,
            } => self.run_sast(target_dir.clone(), rules_dir.clone(), syn_scan_only.clone()),
            Commands::Reverse {
                mode,
                out_dir,
                bytecodes_file,
                labeling,
            } => self.run_reverse(mode.clone(), out_dir.clone(), bytecodes_file.clone(), *labeling),            
            _ => info!("No command selected"),
        }
    }

    /// Executes the build command for the given project path and stores the result.
    ///
    /// # Arguments
    ///
    /// * `target_dir` - Path to the project to build.
    /// * `out_dir` - Output directory for build artifacts.
    ///
    /// # Side Effects
    ///
    /// On success, the resulting `BuildState` is stored in `build_states`.
    /// On failure, an error is logged.
    pub fn build_project(&mut self, target_dir: String, out_dir: String) {
        match commands::build_command::run(&target_dir, &out_dir) {
            Ok(bs) => self.build_states.push(bs),
            Err(e) => {
                error!("An error occurred during build of {} {}", target_dir, e);
            }
        }
    }

    /// Runs static analysis (SAST) on the given project using the provided rule set.
    ///
    /// # Arguments
    ///
    /// * `target_dir` - The root directory of the project to analyze.
    /// * `rules_dir` - Directory containing the analysis rules.
    /// * `syn_scan_only` - Whether to limit analysis to syntax scanning only.
    ///
    /// # Side Effects
    ///
    /// On success, the resulting `SastState` is stored in `sast_states`.
    /// On failure, an error is logged.
    fn run_sast(&mut self, target_dir: String, rules_dir: String, syn_scan_only: bool) {
        match commands::sast_command::run(&target_dir, &rules_dir, syn_scan_only) {
            Ok(ss) => self.sast_states.push(ss),
            Err(e) => {
                error!("An error occurred during SAST of {} {}", target_dir, e);
            }
        }
    }

    /// Runs reverse engineering (static analysis) based on compiled bytecode.
    ///
    /// # Arguments
    ///
    /// * `mode` - The mode of analysis (e.g., disass, cfg, both, rusteq).
    /// * `out_dir` - Directory to write output files.
    /// * `bytecodes_file` - Path to the compiled eBPF bytecode (.so).
    /// * `labeling` - Whether to enable symbol and section labeling.
    ///
    /// # Side Effects
    ///
    /// Logs success or error messages based on the result.
    fn run_reverse(&mut self, mode: String, out_dir: String, bytecodes_file: String, labeling: bool) {
        match commands::reverse_command::run(mode, out_dir, bytecodes_file, labeling) {
            Ok(_) => info!("Reverse (static analysis) completed."),
            Err(e) => error!("An error occurred during reverse (static analysis): {}", e),
        }
    }

}
