use crate::state;
use crate::state::sast_state::{SynAst, SynAstMap};
use anyhow::{Context, Result};
use log::{debug, error};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::digest::Update;
use sha2::Digest;
use std::any::TypeId;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::Path;
use std::{fmt, fs};
use syn::spanned::Spanned;
use syn::visit;
use syn::visit::Visit;

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
            ast_map.insert(
                filename,
                SynAst {
                    ast: ast.clone(),
                    ast_positions: enrich_ast_with_source_lines(&ast, &file_content, path),
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

#[derive(Debug, Clone)]
pub struct SourcePosition {
    pub node_span: proc_macro2::Span,
    pub start_span: proc_macro2::LineColumn,
    pub end_span: proc_macro2::LineColumn,
    pub source_file: String,
}

impl SourcePosition {
    pub fn get_pretty_string(&self) -> String {
        format!(
            "{}:{}:{}",
            self.source_file, self.start_span.line, self.start_span.column
        )
    }
}

impl fmt::Display for SourcePosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Span: {:?}", self.node_span)
    }
}

#[derive(Debug, Clone)]
pub struct AstPositions {
    pub positions: HashMap<[u8; 32], SourcePosition>,
    pub last_ident_hash: [u8; 32],
}

impl AstPositions {
    pub fn new() -> Self {
        let mut hasher = sha2::Sha256::new();
        Digest::update(&mut hasher, "DEFAULT_STATE".as_bytes());
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&hasher.finalize()[..32]);
        Self {
            positions: HashMap::new(),
            last_ident_hash: hash,
        }
    }

    pub fn add_position<T: 'static>(&mut self, node: &T, position: SourcePosition, ident: String) {
        let mut hasher = sha2::Sha256::new();
        Digest::update(&mut hasher, &self.last_ident_hash);
        Digest::update(&mut hasher, ident.as_bytes());
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&hasher.finalize()[..32]);
        self.last_ident_hash = hash;
        self.positions.insert(self.last_ident_hash, position);
    }

    pub fn get_position<T: 'static>(&self, node: &T) -> Option<&SourcePosition> {
        let mut hasher = sha2::Sha256::new();
        Digest::update(&mut hasher, "DEFAULT_STATE".as_bytes());
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&hasher.finalize()[..32]);
        self.positions.get(&hash)
    }
}

struct SpanCollector<'a> {
    rust_code: &'a str,
    source_file_path: &'a Path,
    positions: AstPositions,
}

impl<'a, 'ast> Visit<'ast> for SpanCollector<'a> {
    fn visit_ident(&mut self, node: &'ast syn::Ident) {
        let span = node.span();
        self.positions.add_position(
            node,
            SourcePosition {
                node_span: span.clone(),
                start_span: span.start(),
                end_span: span.end(),
                source_file: match self.source_file_path.to_str() {
                    Some(path) => path.to_string(),
                    None => "no_source_path".to_string(),
                },
            },
            node.to_string(),
        );
        visit::visit_ident(self, node);
    }
}

pub fn enrich_ast_with_source_lines(
    ast: &syn::File,
    rust_code: &str,
    source_file_path: &Path,
) -> AstPositions {
    let mut collector = SpanCollector {
        rust_code,
        source_file_path,
        positions: AstPositions::new(),
    };
    collector.visit_file(ast);
    collector.positions
}
