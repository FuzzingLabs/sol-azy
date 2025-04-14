use std::any::TypeId;
use crate::state;
use crate::state::sast_state::{SynAst, SynAstMap};
use anyhow::{Context, Result};
use log::error;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::{fmt, fs};
use std::fmt::Formatter;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::Path;
use proc_macro2::Span;
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
        .file_name()
        .unwrap_or_default()
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

#[derive(Eq)]
pub struct NodeRef {
    ptr: *const (),
    type_id: TypeId,
}

impl Clone for NodeRef {
    fn clone(&self) -> Self {
        NodeRef {
            ptr: self.ptr,
            type_id: self.type_id,
        }
    }
}

impl fmt::Debug for NodeRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeRef")
            .field("ptr", &(self.ptr as usize))
            .field("type_id", &self.type_id)
            .finish()
    }
}

impl PartialEq for NodeRef {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr && self.type_id == other.type_id
    }
}

impl Hash for NodeRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.ptr as usize).hash(state);
        self.type_id.hash(state);
    }
}


pub fn node_to_ref<T: 'static>(node: &T) -> NodeRef {
    NodeRef {
        ptr: node as *const T as *const (),
        type_id: TypeId::of::<T>(),
    }
}

#[derive(Debug, Clone)]
pub struct SourcePosition {
    pub node_span: Span,
}

impl fmt::Display for SourcePosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Span: {:?}", self.node_span)
    }
}


#[derive(Debug, Clone)]
pub struct AstPositions {
    pub positions: HashMap<NodeRef, SourcePosition>,
}

impl AstPositions {
    pub fn new() -> Self {
        Self { positions: HashMap::new() }
    }

    pub fn add_position<T: 'static>(&mut self, node: &T, position: SourcePosition) {
        self.positions.insert(node_to_ref(node), position);
    }

    pub fn get_position<T: 'static>(&self, node: &T) -> Option<&SourcePosition> {
        self.positions.get(&node_to_ref(node))
    }
}


struct SpanCollector<'a> {
    rust_code: &'a str,
    source_file_path: &'a Path,
    positions: AstPositions,
}

impl<'a, 'ast> Visit<'ast> for SpanCollector<'a> {
    fn visit_ident(&mut self, node: &'ast syn::Ident) {
        self.positions.add_position(node, SourcePosition {
            node_span: node.span(),
        });
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
