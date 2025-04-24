//! Command dispatchers for the `sol-azy` CLI tool.
//!
//! This module provides submodules for each top-level command supported by the CLI:
//!
//! - [`build_command`] — Handles building Anchor or SBF Solana programs.
//! - [`sast_command`] — Runs SAST (static analysis) using custom Starlark rules.
//! - [`reverse_command`] — Performs reverse engineering on compiled eBPF bytecode
//!   (disassembly, CFG generation, etc.).
//!
//! Each subcommand encapsulates its logic, parsing, validation, and execution paths.
//! These are used internally by [`AppState`](crate::state::app_state::AppState) to handle `clap` commands.

pub mod build_command;
pub mod sast_command;
pub mod reverse_command;