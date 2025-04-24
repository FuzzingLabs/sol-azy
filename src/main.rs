//! Entry point for the `sol-azy` CLI application.
//!
//! This CLI tool provides commands for building Solana programs, running SAST (static analysis),
//! and performing reverse engineering (disassembly and CFG generation) on compiled bytecode.
//!
//! Commands are parsed using `clap`, and executed through the central `AppState` dispatcher.

mod commands;
mod state;
mod helpers;
mod parsers;
mod engines;
mod printers;
mod reverse;

use clap::{Parser, Subcommand};
use tracing_subscriber::fmt;
use crate::state::app_state::AppState;

#[derive(Parser)]
#[clap(name = "sol-azy", version = "0.1", author = "FuzzingLabs")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build {
        #[clap(short = 'd', long = "target-dir")]
        target_dir: String,
        #[clap(short = 'r', long = "out-dir")]
        out_dir: String,
    },
    Sast {
        #[clap(short = 'd', long = "target-dir")]
        target_dir: String,
        #[clap(short = 'r', long = "rules-dir")]
        rules_dir: String,
        #[clap(short = 's', long = "syn-scan-only", default_value_t = false)]
        syn_scan_only: bool,
        // TODO: use Build out-dir in options
    },
    Fuzz {
    },
    Test {
    },
    Clean {
    },
    // example: cargo run -- reverse --mode both --out-dir test_cases/base_sbf_addition_checker/out1/  --bytecodes-file ./test_cases/base_sbf_addition_checker/bytecodes/addition_checker.so --labeling 
    Reverse {
        #[clap(long = "mode", value_parser = clap::builder::PossibleValuesParser::new(["disass", "cfg", "both", "rusteq"]))]
        mode: String,

        #[clap(long = "out-dir")]
        out_dir: String,

        #[clap(long = "bytecodes-file")]
        bytecodes_file: String,

        #[clap(long = "labeling", action)]
        labeling: bool,

        // Note: rusteq mode is not supported yet
    }
}

fn main() {
    fmt::Subscriber::builder()
        .with_env_filter("sol_azy=debug")
        .pretty()
        .init();

    let mut app = AppState {
        cli: Cli::parse(),
        build_states: vec![],
        sast_states: vec![]
    };

    app.run_cli()
}
