use std::path::Path;
use crate::Commands;
use crate::helpers::BeforeCheck;
use log::{debug, error};

pub struct RecapCmd {
    pub anchor_path: Option<String>,
}

impl RecapCmd {
    pub fn new_from_clap(cmd: &Commands) -> Self {
        match cmd {
            Commands::Recap { anchor_path } => Self {
                anchor_path: anchor_path.clone(),
            },
            _ => unreachable!(),
        }
    }
}

pub(crate) fn checks_before_recap(anchor_path: &Option<String>) -> bool {
    if let Some(p) = anchor_path {
        let checks_passed = [
            BeforeCheck {
                error_msg: format!("Anchor path '{}' does not exist.", p),
                result: Path::new(p).exists(),
            },
            BeforeCheck {
                error_msg: format!("No Anchor.toml at '{}'.", p),
                result: Path::new(p).join("Anchor.toml").exists(),
            },
        ]
        .iter()
        .map(|check| {
            if !check.result {
                error!("{}", check.error_msg);
                return false;
            }
            true
        })
        .all(|x| x);

        if !checks_passed {
            return false;
        }

        if let Err(e) = std::env::set_current_dir(p) {
            error!("Failed to set current directory to '{}': {}", p, e);
            return false;
        }
        debug!("Working directory set to '{}'", p);
    } else {
        debug!("Will try to use the current working directory");
    }

    true
}

pub fn run(cmd: &RecapCmd) -> anyhow::Result<()> {
    debug!("Starting recap process for {:?}", cmd.anchor_path);

    // quick precheck just to see if the optionnally supplied path is ok
    if !checks_before_recap(&cmd.anchor_path) {
        return Err(anyhow::anyhow!("Can't launch recap, see errors above."));
    }
    
    crate::recap::recap_project(cmd.anchor_path.clone())
}