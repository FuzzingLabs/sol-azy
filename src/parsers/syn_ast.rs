use crate::state::sast_state::{SynAst, SynAstMap};
use anyhow::{Context, Result};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::Digest;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::Path;
use std::{fmt, fs};
use syn::spanned::Spanned;
use syn::visit;
use syn::visit::Visit;

/// Recursively parses all `.rs` files in a directory into syntax trees and enriches them with position data.
///
/// # Arguments
///
/// * `dir` - Path to the root directory containing Rust files.
///
/// # Returns
///
/// A `SynAstMap` mapping filenames to their corresponding enriched syntax trees.
pub fn get_syn_ast_recursive(dir: &str) -> Result<SynAstMap> {
    let mut ast_map = HashMap::new();
    visit_dir(Path::new(dir), &mut ast_map)?;
    Ok(ast_map)
}

fn visit_dir(dir_path: &Path, ast_map: &mut SynAstMap) -> Result<()> {
    if !dir_path.exists() {
        return Ok(());
    }

    let dir_entries = fs::read_dir(dir_path).context("Failed to read directory")?;
    for entry in dir_entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                error!("Failed to get directory entry: {}", e);
                continue;
            }
        };
        let path = entry.path();

        if path.is_file() && path.extension().unwrap_or_default() == "rs" {
            if let Err(e) = parse_rust_file(&path, ast_map) {
                error!("Error parsing Rust file {:?}: {}", path, e);
            }
        } else if path.is_dir() {
            if let Err(e) = visit_dir(&path, ast_map) {
                error!("Error visiting directory {:?}: {}", path, e);
            }
        }
    }
    Ok(())
}

pub fn parse_rust_file(path: &Path, ast_map: &mut SynAstMap) -> Result<()> {
    let file_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read Rust file {:?}: {}", path, e);
            return Err(e.into());
        }
    };
    let filename = path
        .to_str()
        .unwrap_or("")
        .to_string();

    match syn::parse_file(&file_content) {
        Ok(ast) => {
            // Generate position info using access paths instead of hashes
            let ast_positions = enrich_ast_with_source_lines(&ast, path);
            
            // Generate enriched JSON with position information
            let ast_json = ast_to_json_with_positions(&ast, &ast_positions);
            
            ast_map.insert(
                filename,
                SynAst {
                    ast: ast.clone(),
                    ast_positions,
                    ast_json,
                    results: vec![],
                },
            );
        }
        Err(error) => {
            error!("Failed to parse Rust file {:?}: {}", path, error);
        }
    };
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourcePosition {
    pub start_line: u32,
    pub start_column: u32,
    pub end_line: u32, 
    pub end_column: u32,
    pub source_file: String,
}

impl SourcePosition {
    pub fn from_span(span: &proc_macro2::Span, source_file: String) -> Self {
        Self {
            start_line: span.start().line as u32,
            start_column: span.start().column as u32,
            end_line: span.end().line as u32,
            end_column: span.end().column as u32,
            source_file,
        }
    }
    
    pub fn get_pretty_string(&self) -> String {
        format!(
            "{}:{}:{}",
            self.source_file, self.start_line, self.start_column
        )
    }
}

impl fmt::Display for SourcePosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_pretty_string())   
    }
}

/// A mapping between `NodeRef` identifiers and their corresponding source code spans.
///
/// Used to enrich parsed syntax trees with source location metadata.
#[derive(Debug, Clone)]
pub struct AstPositions {
    // Store position info directly on nodes, removing the need for a HashMap
    pub nodes_with_positions: Vec<(String, SourcePosition)>, // Path -> Position
}

impl AstPositions {
    pub fn new() -> Self {
        Self {
            nodes_with_positions: Vec::new(),
        }
    }
    
    pub fn add_position(&mut self, access_path: String, position: SourcePosition) {
        self.nodes_with_positions.push((access_path, position));
    }
}

struct SpanCollector<'a> {
    source_file_path: &'a Path,
    positions: AstPositions,
    current_path: Vec<String>, // Track the access path during traversal
}

impl<'a, 'ast> Visit<'ast> for SpanCollector<'a> {
    fn visit_ident(&mut self, node: &'ast syn::Ident) {
        let span = node.span();
        let access_path = self.current_path.join(".");
        
        // Add node position without using hash
        self.positions.add_position(
            node.to_string(),
            SourcePosition::from_span(&span, 
                match self.source_file_path.to_str() {
                    Some(path) => path.to_string(),
                    None => "no_source_path".to_string(),
                },
            )
        );
        visit::visit_ident(self, node);
    }
}

/// Traverses an AST and collects span data for identifiers, storing it in an `AstPositions` map.
///
/// # Arguments
///
/// * `ast` - The parsed syntax tree (`syn::File`) to analyze.
/// * `source_file_path` - Path to the source file (used for logging/debugging).
///
/// # Returns
///
/// An `AstPositions` structure containing span metadata for relevant nodes.
pub fn enrich_ast_with_source_lines(
    ast: &syn::File,
    source_file_path: &Path,
) -> AstPositions {
    let mut collector = SpanCollector {
        source_file_path,
        positions: AstPositions::new(),
        current_path: Vec::new(),
    };
    collector.visit_file(ast);
    collector.positions
}

pub fn ast_to_json_with_positions(
    ast: &syn::File, 
    positions: &AstPositions
) -> serde_json::Value {
    let ast_json_string = syn_serde::json::to_string(ast);

    let mut ast_json: serde_json::Value = serde_json::from_str(&ast_json_string)
        .unwrap_or_else(|_| serde_json::json!({}));


    let positions_map: HashMap<&str, &SourcePosition> = positions.nodes_with_positions
        .iter()
        .map(|(path, pos)| (path.as_str(), pos))
        .collect();
        
    enrich_json_with_positions(&mut ast_json, &positions_map);
    
    ast_json
}

fn enrich_json_with_positions(
    node: &mut serde_json::Value,
    positions: &HashMap<&str, &SourcePosition>
) {
    match node {
        serde_json::Value::Object(map) => {
            if let Some(ident) = map.get("ident").and_then(|v| v.as_str()) {
                if let Some(position) = positions.get(ident) {
                    map.insert("position".to_string(), json!({
                        "start_line": position.start_line,
                        "start_column": position.start_column,
                        "end_line": position.end_line,
                        "end_column": position.end_column,
                        "source_file": position.source_file
                    }));
                }
            }
            
            for (_, value) in map {
                enrich_json_with_positions(value, positions);
            }
        },
        serde_json::Value::Array(arr) => {
            for item in arr {
                enrich_json_with_positions(item, positions);
            }
        },
        _ => {}
    }
}