use crate::state::sast_state::{SynAst, SynAstMap};
use anyhow::{Context, Result};
use log::error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::path::Path;
use std::{fmt, fs};
use syn::spanned::Spanned;
use syn::visit;
use syn::visit::Visit;

/// Recursively traverses a directory, parses all `.rs` files into syntax trees,
/// and enriches them with source code position data.
///
/// # Arguments
///
/// * `dir` - The path to the root directory to scan for Rust files.
///
/// # Returns
///
/// A `Result` containing a `SynAstMap` that maps file paths to their corresponding
/// enriched `SynAst` structures.
pub fn get_syn_ast_recursive(dir: &str) -> Result<SynAstMap> {
    let mut ast_map = HashMap::new();
    visit_dir(Path::new(dir), &mut ast_map)?;
    Ok(ast_map)
}

/// Helper function to recursively visit directories and parse Rust files.
///
/// # Arguments
///
/// * `dir_path` - The path of the directory to visit.
/// * `ast_map` - A mutable reference to the `SynAstMap` to populate.
///
/// # Returns
///
/// An empty `Result` on success, or an error if directory traversal fails.
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

/// Parses a single Rust file into a `SynAst` and adds it to the provided map.
///
/// This function reads the file, parses it into a `syn::File`, enriches it with
/// source code positions, creates a JSON representation, and stores the resulting
/// `SynAst` in the `ast_map`.
///
/// # Arguments
///
/// * `path` - The path to the Rust file to parse.
/// * `ast_map` - A mutable reference to the `SynAstMap` to add the parsed data to.
///
/// # Returns
///
/// An empty `Result` on success, or an error if file reading or parsing fails.
pub fn parse_rust_file(path: &Path, ast_map: &mut SynAstMap) -> Result<()> {
    let file_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read Rust file {:?}: {}", path, e);
            return Err(e.into());
        }
    };
    let filename = path.to_str().unwrap_or("").to_string();

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

/// Represents a location in a source file, including start and end coordinates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourcePosition {
    pub start_line: u32,
    pub start_column: u32,
    pub end_line: u32,
    pub end_column: u32,
    pub source_file: String,
}

impl SourcePosition {
    /// Creates a `SourcePosition` from a `proc_macro2::Span`.
    ///
    /// # Arguments
    ///
    /// * `span` - The `Span` to convert.
    /// * `source_file` - The path to the source file containing the span.
    pub fn from_span(span: &proc_macro2::Span, source_file: String) -> Self {
        Self {
            start_line: span.start().line as u32,
            start_column: span.start().column as u32,
            end_line: span.end().line as u32,
            end_column: span.end().column as u32,
            source_file,
        }
    }

    /// Formats the position into a `file:line:column` string.
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

/// A collection mapping AST node identifiers to their source code positions.
///
/// This structure stores a list of tuples, where each tuple contains a string
/// representation of a node (e.g., an identifier) and its corresponding `SourcePosition`.
#[derive(Debug, Clone)]
pub struct AstPositions {
    // Store position info directly on nodes, removing the need for a HashMap
    pub nodes_with_positions: Vec<(String, SourcePosition)>, // Path -> Position
}

impl AstPositions {
    /// Creates a new, empty `AstPositions` collection.
    pub fn new() -> Self {
        Self {
            nodes_with_positions: Vec::new(),
        }
    }

    /// Adds a new node's position to the collection.
    pub fn add_position(&mut self, access_path: String, position: SourcePosition) {
        self.nodes_with_positions.push((access_path, position));
    }
}

/// A `syn::visit::Visit` implementation that collects source spans for `syn::Ident` nodes.
struct SpanCollector<'a> {
    source_file_path: &'a Path,
    positions: AstPositions,
}


impl<'a, 'ast> SpanCollector<'a> {
    /// Helper method to add position information for a span with a given prefix and name
    fn add_span_position(&mut self, name: &str, span: &proc_macro2::Span) {
        self.positions.add_position(
            name.parse().unwrap(),
            SourcePosition::from_span(
                span,
                match self.source_file_path.to_str() {
                    Some(path) => path.to_string(),
                    None => "no_source_path".to_string(),
                },
            ),
        );
    }

    /// Helper method to extract path as string from syn::Path
    fn path_to_string(path: &syn::Path) -> String {
        path.segments.iter()
            .map(|seg| seg.ident.to_string())
            .collect::<Vec<_>>()
            .join("::")
    }
}

impl<'a, 'ast> Visit<'ast> for SpanCollector<'a> {
    // Basic identifiers
    fn visit_ident(&mut self, node: &'ast syn::Ident) {
        self.add_span_position(&node.to_string(), &node.span());
        visit::visit_ident(self, node);
    }

    // Function definitions
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.add_span_position(&node.sig.ident.to_string(), &node.sig.ident.span());
        visit::visit_item_fn(self, node);
    }

    // Struct definitions
    fn visit_item_struct(&mut self, node: &'ast syn::ItemStruct) {
        self.add_span_position(&node.ident.to_string(), &node.ident.span());
        visit::visit_item_struct(self, node);
    }

    // Enum definitions
    fn visit_item_enum(&mut self, node: &'ast syn::ItemEnum) {
        self.add_span_position(&node.ident.to_string(), &node.ident.span());
        visit::visit_item_enum(self, node);
    }

    // Enum variants
    fn visit_variant(&mut self, node: &'ast syn::Variant) {
        self.add_span_position(&node.ident.to_string(), &node.ident.span());
        visit::visit_variant(self, node);
    }

    // Trait definitions
    fn visit_item_trait(&mut self, node: &'ast syn::ItemTrait) {
        self.add_span_position(&node.ident.to_string(), &node.ident.span());
        visit::visit_item_trait(self, node);
    }

    // Impl blocks
    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        let type_name = match &*node.self_ty {
            syn::Type::Path(type_path) => Self::path_to_string(&type_path.path),
            _ => "unknown".to_string(),
        };
        self.add_span_position(&type_name, &node.impl_token.span);
        visit::visit_item_impl(self, node);
    }

    // Variables and patterns
    fn visit_pat_ident(&mut self, node: &'ast syn::PatIdent) {
        self.add_span_position(&node.ident.to_string(), &node.ident.span());
        visit::visit_pat_ident(self, node);
    }

    // Type definitions
    fn visit_item_type(&mut self, node: &'ast syn::ItemType) {
        self.add_span_position(&node.ident.to_string(), &node.ident.span());
        visit::visit_item_type(self, node);
    }

    // Constants
    fn visit_item_const(&mut self, node: &'ast syn::ItemConst) {
        self.add_span_position(&node.ident.to_string(), &node.ident.span());
        visit::visit_item_const(self, node);
    }

    // Static items
    fn visit_item_static(&mut self, node: &'ast syn::ItemStatic) {
        self.add_span_position(&node.ident.to_string(), &node.ident.span());
        visit::visit_item_static(self, node);
    }

    // Modules
    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        self.add_span_position(&node.ident.to_string(), &node.ident.span());
        visit::visit_item_mod(self, node);
    }

    // Use statements
    fn visit_item_use(&mut self, node: &'ast syn::ItemUse) {
        let use_path = match &node.tree {
            syn::UseTree::Path(path) => path.ident.to_string(),
            syn::UseTree::Name(name) => name.ident.to_string(),
            syn::UseTree::Glob(_) => "*".to_string(),
            syn::UseTree::Group(_) => {
                format!("{{...}}")
            }
            syn::UseTree::Rename(rename) => {
                format!("{} as {}", rename.ident, rename.rename)
            }
        };
        self.add_span_position(&use_path, &node.use_token.span);
        visit::visit_item_use(self, node);
    }

    // Macro calls
    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        let macro_name = Self::path_to_string(&node.path);
        self.add_span_position(&macro_name, &node.path.span());
        visit::visit_macro(self, node);
    }

    // Field definitions
    fn visit_field(&mut self, node: &'ast syn::Field) {
        if let Some(ident) = &node.ident {
            self.add_span_position(&ident.to_string(), &ident.span());
        }
        visit::visit_field(self, node);
    }

    // Path expressions (function calls, variable references, etc.)
    fn visit_expr_path(&mut self, node: &'ast syn::ExprPath) {
        let path_str = Self::path_to_string(&node.path);
        self.add_span_position(&path_str, &node.path.span());
        visit::visit_expr_path(self, node);
    }

    // Function calls
    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        if let syn::Expr::Path(path) = &*node.func {
            let func_name = Self::path_to_string(&path.path);
            self.add_span_position(&func_name, &path.path.span());
        }
        visit::visit_expr_call(self, node);
    }

    // Method calls
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        self.add_span_position(&node.method.to_string(), &node.method.span());
        visit::visit_expr_method_call(self, node);
    }

    // Match expressions
    fn visit_expr_match(&mut self, node: &'ast syn::ExprMatch) {
        self.add_span_position("match_expr", &node.match_token.span);
        visit::visit_expr_match(self, node);
    }

    // If expressions
    fn visit_expr_if(&mut self, node: &'ast syn::ExprIf) {
        self.add_span_position("if_expr", &node.if_token.span);
        visit::visit_expr_if(self, node);
    }

    // For loops
    fn visit_expr_for_loop(&mut self, node: &'ast syn::ExprForLoop) {
        self.add_span_position("for_expr", &node.for_token.span);
        visit::visit_expr_for_loop(self, node);
    }

    // While loops
    fn visit_expr_while(&mut self, node: &'ast syn::ExprWhile) {
        self.add_span_position("while_expr", &node.while_token.span);
        visit::visit_expr_while(self, node);
    }

    // Loop expressions
    fn visit_expr_loop(&mut self, node: &'ast syn::ExprLoop) {
        self.add_span_position("loop_expr", &node.loop_token.span);
        visit::visit_expr_loop(self, node);
    }

    // Let statements
    fn visit_local(&mut self, node: &'ast syn::Local) {
        self.add_span_position("let_stmt", &node.let_token.span);
        visit::visit_local(self, node);
    }

    // Struct field access
    fn visit_expr_field(&mut self, node: &'ast syn::ExprField) {
        if let syn::Member::Named(ident) = &node.member {
            self.add_span_position(&ident.to_string(), &ident.span());
        }
        visit::visit_expr_field(self, node);
    }

    // Struct initialization
    fn visit_expr_struct(&mut self, node: &'ast syn::ExprStruct) {
        let struct_name = Self::path_to_string(&node.path);
        self.add_span_position(&struct_name, &node.path.span());
        visit::visit_expr_struct(self, node);
    }

    // Literals
    fn visit_lit_str(&mut self, node: &'ast syn::LitStr) {
        self.add_span_position(&node.value(), &node.span());
        visit::visit_lit_str(self, node);
    }

    fn visit_lit_int(&mut self, node: &'ast syn::LitInt) {
        self.add_span_position(&node.base10_digits(), &node.span());
        visit::visit_lit_int(self, node);
    }

    fn visit_lit_float(&mut self, node: &'ast syn::LitFloat) {
        self.add_span_position(&node.base10_digits(), &node.span());
        visit::visit_lit_float(self, node);
    }

    fn visit_lit_bool(&mut self, node: &'ast syn::LitBool) {
        self.add_span_position(&node.value.to_string(), &node.span);
        visit::visit_lit_bool(self, node);
    }
}

/// Traverses a `syn::File` AST and collects source code positions for all identifiers.
///
/// # Arguments
///
/// * `ast` - The parsed syntax tree (`syn::File`) to analyze.
/// * `source_file_path` - The path to the source file, used to create full `SourcePosition` data.
///
/// # Returns
///
/// An `AstPositions` structure containing the collected position metadata.
pub fn enrich_ast_with_source_lines(ast: &syn::File, source_file_path: &Path) -> AstPositions {
    let mut collector = SpanCollector {
        source_file_path,
        positions: AstPositions::new(),
    };
    collector.visit_file(ast);
    collector.positions
}

/// Serializes a `syn::File` to a JSON value and injects source position information.
///
/// # Arguments
///
/// * `ast` - The syntax tree to serialize.
/// * `positions` - The collected source positions to embed in the JSON.
///
/// # Returns
///
/// A `serde_json::Value` representing the AST with embedded position data.
pub fn ast_to_json_with_positions(ast: &syn::File, positions: &AstPositions) -> serde_json::Value {
    let ast_json_string = syn_serde::json::to_string(ast);

    let mut ast_json: serde_json::Value =
        serde_json::from_str(&ast_json_string).unwrap_or_else(|_| json!({}));

    let positions_map: HashMap<&str, &SourcePosition> = positions
        .nodes_with_positions
        .iter()
        .map(|(path, pos)| (path.as_str(), pos))
        .collect();

    enrich_json_with_positions(&mut ast_json, &positions_map);

    ast_json
}

/// Recursively traverses a JSON value and adds a "position" field to objects
/// that have an "ident" field found in the positions map.
///
/// # Arguments
///
/// * `node` - A mutable reference to the `serde_json::Value` to traverse.
/// * `positions` - A map from identifier strings to their `SourcePosition`.
fn enrich_json_with_positions(
    node: &mut serde_json::Value,
    positions: &HashMap<&str, &SourcePosition>,
) {
    match node {
        serde_json::Value::Object(map) => {
            if let Some(ident) = map.get("ident").and_then(|v| v.as_str()) {
                if let Some(position) = positions.get(ident) {
                    map.insert(
                        "position".to_string(),
                        json!({
                            "start_line": position.start_line,
                            "start_column": position.start_column,
                            "end_line": position.end_line,
                            "end_column": position.end_column,
                            "source_file": position.source_file
                        }),
                    );
                }
            }

            for (_, value) in map {
                enrich_json_with_positions(value, positions);
            }
        }
        serde_json::Value::Array(arr) => {
            for item in arr {
                enrich_json_with_positions(item, positions);
            }
        }
        _ => {}
    }
}
