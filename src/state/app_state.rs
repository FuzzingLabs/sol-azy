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
            cmd @ Commands::Build { .. } => {
                self.build_project(&commands::build_command::BuildCmd::new_from_clap(cmd))
            }
            cmd @ Commands::Sast { .. } => {
                self.run_sast(&commands::sast_command::SastCmd::new_from_clap(cmd))
            }
            _ => info!("No command selected"),
        }
    }

    pub fn build_project(&mut self, cmd: &commands::build_command::BuildCmd) {
        match commands::build_command::run(cmd) {
            Ok(bs) => self.build_states.push(bs),
            Err(e) => error!("An error occurred during build of {} {}", cmd.target_dir, e),
        }
    }

    fn run_sast(&mut self, cmd: &commands::sast_command::SastCmd) {
        match commands::sast_command::run(cmd) {
            Ok(ss) => {
                ss.iter().for_each(|s| {
                    if let Err(e) = s.print_results() {
                        error!("Failed to print results: {}", e);
                    }
                });
                self.sast_states.extend(ss)
            },
            Err(e) => error!("An error occurred during SAST of {} {}", cmd.target_dir, e),
        }
    }
}
