//! Parsers and AST utilities for Rust source files.
//!
//! This module provides utilities to parse Rust code and enrich it with additional metadata,
//! especially useful for static analysis workflows.
//!
//! - [`syn_ast`] — Parses `.rs` files into `syn::File` ASTs and tracks spans for diagnostics.
//!
//! These parsers are used by rule engines to apply checks and extract semantic information from source code.

pub mod syn_ast;
