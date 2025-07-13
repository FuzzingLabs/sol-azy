use crate::Commands;
use anyhow::{Result, Context};
use log::info;
use std::fs;
use std::path::Path;
use crate::engines::starlark_engine::StarlarkEngine;
use crate::parsers::syn_ast::{ast_to_json_with_positions, enrich_ast_with_source_lines};
use crate::state::sast_state::SynAst;

pub struct AstUtilsCmd {
    pub file_path: String,
    pub starlark_syn_ast: bool,
}

impl AstUtilsCmd {
    pub fn new_from_clap(cmd: &Commands) -> Self {
        match cmd {
            Commands::AstUtils { file_path, starlark_syn_ast } => Self {
                file_path: file_path.clone(),
                starlark_syn_ast: *starlark_syn_ast,
            },
            _ => unreachable!(),
        }
    }
}

fn generate_ast_from_file(file_path: &str) -> Result<syn::File> {
    info!("Generating AST for file: {}", file_path);
    let file_contents = fs::read_to_string(file_path)
        .with_context(|| format!("Unable to read file: {}", file_path))?;
    syn::parse_file(&file_contents)
        .with_context(|| format!("Unable to parse file: {}", file_path))
}

pub fn run(cmd: &AstUtilsCmd) -> Result<()> {
    let ast = generate_ast_from_file(&cmd.file_path)?;
    if !cmd.starlark_syn_ast {
        println!("{}", syn_serde::json::to_string_pretty(&ast));
        return Ok(())
    }

    let ast_positions = enrich_ast_with_source_lines(&ast, Path::new(cmd.file_path.as_str()));

    let ast_json = ast_to_json_with_positions(&ast, &ast_positions);
    let prepared = StarlarkEngine::new().eval_get_prepared_ast("get_prepared_ast", String::new(), &SynAst {
        ast: ast.clone(),
        ast_positions,
        ast_json,
        results: vec![],
    }).with_context(|| "Failed to evaluate prepared AST with Starlark engine")?;

    // Try to parse and pretty-print as JSON, fall back to raw string if parsing fails
    match serde_json::from_str::<serde_json::Value>(&prepared) {
        Ok(json_value) => {
            match serde_json::to_string_pretty(&json_value) {
                Ok(pretty_json) => println!("{}", pretty_json),
                Err(e) => {
                    eprintln!("Warning: Failed to format JSON: {}", e);
                    println!("{}", prepared);
                }
            }
        },
        Err(e) => {
            eprintln!("Warning: Output is not valid JSON ({}), printing raw output:", e);
            println!("{}", prepared);
        }
    }

    Ok(())
}