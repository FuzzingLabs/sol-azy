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
    pub async fn run_cli(&mut self) {
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
                reduced,
                only_entrypoint
            } => self.run_reverse(mode.clone(), out_dir.clone(), bytecodes_file.clone(), *labeling, *reduced, *only_entrypoint),    
            Commands::Dotting { config, reduced_dot_path, full_dot_path } => {
                self.run_dotting(config.clone(), reduced_dot_path.clone(), full_dot_path.clone())
            }        
            Commands::Fetcher { program_id, out_dir, rpc_url } => {
                self.run_fetcher(program_id.clone(), out_dir.clone(), rpc_url.clone()).await;
            }         
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
    fn run_reverse(&mut self, mode: String, out_dir: String, bytecodes_file: String, labeling: bool, reduced: bool, only_entrypoint: bool) {
        match commands::reverse_command::run(mode, out_dir, bytecodes_file, labeling, reduced, only_entrypoint) {
            Ok(_) => info!("Reverse (static analysis) completed."),
            Err(e) => error!("An error occurred during reverse (static analysis): {}", e),
        }
    }

    /// Executes the dotting process to enrich a reduced `.dot` control flow graph file.
    ///
    /// This function reads a list of target function clusters from a JSON config,
    /// and reinserts them (along with valid edges) into the reduced CFG by referencing
    /// the original full CFG `.dot` file.
    ///
    /// # Arguments
    ///
    /// * `config` - Path to the JSON file listing the `cluster_<id>` functions to re-add.
    /// * `reduced_dot_path` - Path to the previously generated reduced CFG file.
    /// * `full_dot_path` - Path to the full CFG file used as source of truth.
    ///
    /// # Behavior
    ///
    /// Logs success if the process completes without error, or prints an error otherwise.
    fn run_dotting(&mut self, config: String, reduced_dot_path: String, full_dot_path: String) {
        match commands::dotting_command::run(config, reduced_dot_path, full_dot_path) {
            Ok(_) => info!("Dotting completed successfully."),
            Err(e) => error!("Dotting failed: {}", e),
        }
    }

    /// Fetches the bytecode of a Solana program and writes it to a local file.
    ///
    /// This function wraps the `fetcher_command::run` logic with appropriate logging,
    /// and resolves the default Solana RPC endpoint if none is provided. It writes
    /// the fetched bytecode to `<output_path>/fetched_program.so`.
    ///
    /// # Arguments
    ///
    /// * `program_id` - The Solana program ID to fetch from the blockchain.
    /// * `output_path` - Path to the directory where the program will be saved.
    /// * `rpc_url` - Optional RPC endpoint; if `None`, defaults to the mainnet RPC (`https://api.mainnet-beta.solana.com`).
    ///
    /// # Logging
    ///
    /// - Logs the RPC used (and indicates if it's the default).
    /// - Logs success or failure with output location.
    ///
    /// # Errors
    ///
    /// This function logs but does not propagate errors. All failure handling is internal.
    async fn run_fetcher(&mut self, program_id: String, output_path: String, rpc_url: Option<String>) {
        let display_rpc_url = match &rpc_url {
            Some(url) => format!("{url}"),
            None => format!("https://api.mainnet-beta.solana.com (by default)"),
        };
    
        match commands::fetcher_command::run(program_id, output_path.clone(), rpc_url.clone()).await {
            Ok(_) => info!(
                "Bytecode successfully fetched from RPC '{}' and saved to '{}/fetched_program.so'",
                display_rpc_url,
                output_path
            ),
            Err(e) => error!("Fetcher failed: {}", e),
        }
    }    

}
