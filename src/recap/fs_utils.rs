use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn walk(dir: &Path) -> Vec<PathBuf> {
    let mut out = vec![];
    if let Ok(rd) = fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                out.extend(walk(&p));
            } else {
                out.push(p);
            }
        }
    }
    out
}

pub(crate) fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

pub(crate) fn find_all_idls(root: &Path) -> Vec<PathBuf> {
    let idl_dir = root.join("target").join("idl");
    if !idl_dir.exists() {
        return vec![];
    }
    let mut out = vec![];
    if let Ok(rd) = fs::read_dir(&idl_dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.extension().map(|x| x == "json").unwrap_or(false) {
                out.push(p);
            }
        }
    }
    out
}
