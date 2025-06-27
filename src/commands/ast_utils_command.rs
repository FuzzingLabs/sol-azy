use crate::Commands;
use anyhow::{Result};
use log::info;
use std::fs;

pub struct AstUtilsCmd {
    pub file_path: String,
}

impl AstUtilsCmd {
    pub fn new_from_clap(cmd: &Commands) -> Self {
        match cmd {
            Commands::AstUtils { file_path } => Self {
                file_path: file_path.clone(),
            },
            _ => unreachable!(),
        }
    }
}

fn generate_ast_from_file(file_path: &str) -> syn::Result<syn::File> {
    info!("Generating AST for file: {}", file_path);
    let file_contents = fs::read_to_string(file_path).expect("Unable to read file");
    syn::parse_file(&file_contents)
}

pub fn run(cmd: &AstUtilsCmd) -> Result<()> {
    let ast = generate_ast_from_file(&cmd.file_path)?;
    println!("{}", syn_serde::json::to_string_pretty(&ast));
    Ok(())
}
