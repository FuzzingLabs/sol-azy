use super::fs_utils::{read, walk};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub(crate) struct CrateInfo {
    pub(crate) name: String,
    pub(crate) root: PathBuf,
}

pub(crate) fn find_anchor_crates(root: &std::path::Path) -> Vec<CrateInfo> {
    let mut crates = vec![];
    for p in walk(root) {
        if p.file_name().map(|n| n == "Cargo.toml").unwrap_or(false) {
            let toml = read(&p);
            if toml.contains("anchor-lang") {
                let name = toml
                    .lines()
                    .find_map(|l| {
                        let ll = l.trim();
                        if ll.starts_with("name") && ll.contains('=') {
                            Some(ll.split('=').nth(1)?.trim().trim_matches('"').to_string())
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| {
                        p.parent()
                            .unwrap()
                            .file_name()
                            .unwrap()
                            .to_string_lossy()
                            .to_string()
                    });
                crates.push(CrateInfo {
                    name,
                    root: p.parent().unwrap().to_path_buf(),
                });
            }
        }
    }
    crates
}

use super::idl::Idl;
use super::parser::map_instruction_to_struct;

pub(crate) fn pick_crate_for_idl<'a>(idl: &Idl, crates: &'a [CrateInfo]) -> Option<&'a CrateInfo> {
    use super::fs_utils::{read, walk};

    if let Some(idl_name) = idl.name.as_ref() {
        if let Some(c) = crates.iter().find(|c| &c.name == idl_name) {
            return Some(c);
        }
    }
    let mut best: Option<(&CrateInfo, usize)> = None;
    for c in crates {
        let src_dir = c.root.join("src");
        if !src_dir.exists() {
            continue;
        }
        let code = walk(&src_dir)
            .into_iter()
            .filter(|p| p.extension().map(|e| e == "rs").unwrap_or(false))
            .map(|p| read(&p))
            .collect::<Vec<_>>()
            .join("\n/*--file--*/\n");
        let fns = map_instruction_to_struct(&code);
        let overlap = idl
            .instructions
            .iter()
            .filter(|ix| fns.contains_key(&ix.name))
            .count();
        if overlap == 0 {
            continue;
        }
        match best {
            None => best = Some((c, overlap)),
            Some((_, b)) if overlap > b => best = Some((c, overlap)),
            _ => {}
        }
    }
    best.map(|(c, _)| c)
}
