use anyhow::{anyhow, Context, Result};

use crate::helpers;

pub mod project;
pub mod fs_utils;
pub mod idl;
pub mod crates;
pub mod parser;
pub mod rows;
pub mod render;


/// Generates a markdown recap (`recap-solazy.md`) summarizing an Anchor project's structure.
///
/// The function scans the specified Anchor project (or the current directory if none is provided),
/// extracts its IDLs and crates, and analyzes each program's instructions and accounts.
/// For each instruction, it lists the **signers**, **writable accounts**, **constraints**, **seeded accounts**, 
/// and **memory-related attributes** in a markdown table.
///
/// The resulting report is written to a file named `recap-solazy.md` in the directory
/// where the command was launched, and a spinner displays the current progress.
pub fn recap_project(anchor_path: Option<String>) -> Result<()> {
    use project::{detect_project_kind, ProjectKind};
    use fs_utils::find_all_idls;
    use crates::find_anchor_crates;
    use idl::load_idl;
    use render::to_markdown;
    use rows::build_rows_for_program;
    use log::{error, warn};
    use std::path::{Path, PathBuf};

    let launch_dir = std::env::var_os("PWD")
        .map(PathBuf::from)
        .unwrap_or(std::env::current_dir().context("Getting current directory")?);

    let cwd = std::env::current_dir().context("Getting current directory")?;
    let root: PathBuf = match anchor_path.as_deref().filter(|s| !s.trim().is_empty()) {
        Some(p) => {
            let candidate = if Path::new(p).is_absolute() {
                PathBuf::from(p)
            } else {
                cwd
            };
            std::fs::canonicalize(&candidate).unwrap_or(candidate)
        }
        None => cwd,
    };

    if detect_project_kind(&root) != ProjectKind::Anchor {
        error!(
            "Non-Anchor project detected (no Anchor.toml at {}). This tool currently supports Anchor projects only.",
            root.display()
        );
        return Err(anyhow!(
            "Non-Anchor project detected (no Anchor.toml at {}). This tool currently supports Anchor projects only.",
            root.display()
        ));
    }

    let spinner = helpers::spinner::get_new_spinner(format!(
        "Performing recap scan on {:?} anchor project...",
        root
    ));

    let idl_paths = find_all_idls(&root);
    if idl_paths.is_empty() {
        spinner.finish_and_clear();
        error!(
            "No IDL files under {}/target/idl/. Run `anchor build` first.",
            root.display()
        );
        return Err(anyhow!(
            "No IDL files under {}/target/idl/. Run `anchor build` first.",
            root.display()
        ));
    }

    let crates = find_anchor_crates(&root);
    if crates.is_empty() {
        spinner.finish_and_clear();
        error!(
            "No Anchor crates (Cargo.toml with anchor-lang) found under {}.",
            root.display()
        );
        return Err(anyhow!(
            "No Anchor crates (Cargo.toml with anchor-lang) found under {}.",
            root.display()
        ));
    }

    let mut idls: Vec<(String, idl::Idl, PathBuf)> = vec![];
    for p in idl_paths {
        let idl = match load_idl(&p) {
            Ok(i) => i,
            Err(e) => {
                spinner.finish_and_clear();
                error!("Failed to load IDL at {}: {}", p.display(), e);
                return Err(anyhow!("Failed to load IDL at {}: {}", p.display(), e));
            }
        };
        let name = idl
            .name
            .clone()
            .unwrap_or_else(|| p.file_stem().unwrap().to_string_lossy().to_string());
        idls.push((name, idl, p));
    }

    let mut out_all = String::new();

    for (prog_name, idl, idl_path) in idls {
        spinner.set_message(format!("Processing program `{}`...", prog_name));

        let Some(krate) = crates::pick_crate_for_idl(&idl, &crates) else {
            warn!(
                "# Program `{}` (IDL: {}) — no matching Anchor crate found. Skipping.",
                prog_name,
                idl_path.display()
            );
            continue;
        };

        let header = format!(
            "# Program `{}`{}",
            prog_name,
            idl.metadata
                .as_ref()
                .and_then(|m| m.address.as_ref())
                .map(|a| format!(" — {}", a))
                .unwrap_or_default()
        );
        out_all.push_str(&header);
        out_all.push('\n');

        let crate_line = format!("_Crate: {}_\n", krate.root.display());
        out_all.push_str(&crate_line);
        out_all.push('\n');

        let rows = build_rows_for_program(&idl, &krate.root);
        if rows.is_empty() {
            out_all.push_str("(No instructions found)\n\n");
            continue;
        }

        let md = to_markdown(&rows);
        out_all.push_str(&md);
        out_all.push('\n');
    }

    let out_path = launch_dir.join("recap-solazy.md");
    if let Err(e) = std::fs::write(&out_path, out_all)
        .with_context(|| format!("Writing {}", out_path.display()))
    {
        spinner.finish_and_clear();
        error!("Failed to write recap output to {}: {}", out_path.display(), e);
        return Err(anyhow!("Failed to write recap output to {}: {}", out_path.display(), e));
    }

    spinner.finish_with_message("Recap scan completed.");

    Ok(())
}
