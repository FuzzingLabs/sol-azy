//! Tools for manipulating `.dot` control flow graph (CFG) files after generation.
//!
//! This module contains utilities to post-process and refine CFGs,
//! especially useful when dealing with reduced graphs where only a subset
//! of the full function space is represented initially.
//!
//! The primary use-case is to restore specific functions and edges manually
//! after a `--reduced` or `--only-entrypoint` generation, enabling iterative analysis workflows.
//!
//! ## Components
//!
//! - [`editor`] – Logic to add user-specified function clusters and associated edges
//!   from the full `.dot` graph into a reduced one.
//!
//! ## Example Use Case
//!
//! Suppose you ran Sol-azy in `--reduced` or `--only-entrypoint` mode and the resulting CFG omitted a function
//! you care about (e.g., because it's from a standard library or wasn't reachable from entrypoint).
//! You can use this module by:
//!
//! 1. Creating a `functions.json` file with the cluster IDs you want to restore.
//! 2. Running the `dotting` command with paths to:
//!    - the reduced `.dot` file,
//!    - the full `.dot` file,
//!    - and the JSON config.

pub mod editor;