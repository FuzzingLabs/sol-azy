//! Execution engines used by the static analysis system.
//!
//! This module currently includes:
//!
//! - [`starlark_engine`] — An engine for evaluating Starlark-based security rules against parsed Rust ASTs.
//!
//! Engines in this module are responsible for interpreting rule files, integrating with
//! the syntax analysis layer, and returning structured results (e.g., matches, metadata).

pub mod starlark_engine;