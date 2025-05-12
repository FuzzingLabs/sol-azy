//! Output utilities for formatting and displaying analysis results.
//!
//! This module is responsible for presenting the results of static analysis
//! in a readable way, either through tables or JSON.
//!
//! - [`sast_printer`] — Pretty-prints SAST rule results in the terminal and can serialize them as JSON.
//!
//! These tools are used after analysis to help users interpret and act on findings.

pub mod sast_printer;