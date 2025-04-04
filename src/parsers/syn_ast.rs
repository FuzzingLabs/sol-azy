use crate::state;
use crate::state::sast_state::{SynAst, SynAstMap};
use anyhow::{Context, Result};
use log::error;
use regex::Regex;
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
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
                    enriched_ast: enrich_ast_with_source_lines(&ast, &file_content, path),
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

fn find_ident_positions(
    code: &str,
    ident: &str,
    source_file_path: &Path,
) -> Vec<serde_json::Value> {
    let mut positions = Vec::new();
    let pattern = format!(r"\b{}\b", regex::escape(ident));
    let re = Regex::new(&pattern).unwrap();
    for mat in re.find_iter(code) {
        let start_pos = mat.start();
        let line_num = code[..start_pos].matches('\n').count() + 1;
        let line_start = code[..start_pos].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let end_pos = mat.end();
        let start_col = start_pos - line_start + 1;
        let end_col = end_pos - line_start + 1;
        positions.push(json!({
            "file": source_file_path.to_string_lossy().to_string(),
            "line": line_num,
            "start_col": start_col,
            "end_col": end_col
        }));
    }
    positions
}

struct IdentCollector<'a> {
    rust_code: &'a str,
    source_file_path: &'a Path,
    scanned_idents: HashMap<String, serde_json::Value>,
}

impl<'a, 'ast> Visit<'ast> for IdentCollector<'a> {
    fn visit_ident(&mut self, i: &'ast syn::Ident) {
        let ident = i.to_string();
        if !self.scanned_idents.contains_key(&ident) {
            let positions = find_ident_positions(self.rust_code, &ident, self.source_file_path);
            if !positions.is_empty() {
                self.scanned_idents.insert(ident, positions[0].clone());
            }
        }
    }
}

pub fn enrich_ast_with_source_lines(
    ast: &syn::File,
    rust_code: &str,
    source_file_path: &Path,
) -> HashMap<String, serde_json::Value> {
    let mut collector = IdentCollector {
        rust_code,
        source_file_path,
        scanned_idents: HashMap::new(),
    };
    collector.visit_file(ast);
    collector.scanned_idents
}
