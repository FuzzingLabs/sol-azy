//! Core data structures representing application state and analysis results.
//!
//! This module defines the persistent and transitional state for the CLI and analysis pipelines:
//!
//! - [`app_state`] — Central dispatcher that holds CLI arguments and accumulated results.
//! - [`build_state`] — Represents the outcome of a build process (e.g., output paths).
//! - [`sast_state`] — Contains static analysis results, syntax trees, and rule evaluations.
//!
//! These types are used throughout the CLI flow to coordinate between command execution and result reporting.

pub mod app_state;
pub mod build_state;
pub mod sast_state;