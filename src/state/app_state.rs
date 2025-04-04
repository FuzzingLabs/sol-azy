use crate::state::build_state::BuildState;
use crate::state::sast_state::SastState;
use crate::{commands, Cli, Commands};
use log::{error, info};

pub struct AppState {
    pub cli: Cli,
    pub build_states: Vec<BuildState>,
    pub sast_states: Vec<SastState>,
}

impl AppState {
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
            _ => info!("No command selected"),
        }
    }

    pub fn build_project(&mut self, target_dir: String, out_dir: String) {
        match commands::build_command::run(&target_dir, &out_dir) {
            Ok(bs) => self.build_states.push(bs),
            Err(e) => {
                error!("An error occurred during build of {} {}", target_dir, e);
            }
        }
    }

    fn run_sast(&mut self, target_dir: String, rules_dir: String, syn_scan_only: bool) {
        match commands::sast_command::run(&target_dir, &rules_dir, syn_scan_only) {
            Ok(ss) => self.sast_states.push(ss),
            Err(e) => {
                error!("An error occurred during SAST of {} {}", target_dir, e);
            }
        }
    }
}
