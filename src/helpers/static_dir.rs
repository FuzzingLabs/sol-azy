use anyhow::{Context, Result};
use include_dir::{include_dir, Dir, File};

pub static STATIC_DIR: Dir<'_> = include_dir!("./src/static");

pub fn list_files(path: &str) -> Result<Vec<String>> {
    STATIC_DIR
        .get_dir(path)
        .map(|dir| dir.files().map(|f| f.path().display().to_string()).collect())
        .context("Failed to list static files")
}

pub fn get_all_files(path: &str) -> Result<Vec<File>> {
    STATIC_DIR
        .get_dir(path)
        .map(|dir| dir.files().cloned().collect())
        .context("Failed to get all static files")
}

pub fn read_file(path: &str) -> Result<String> {
    STATIC_DIR
        .get_file(path)
        .and_then(|file| file.contents_utf8().map(String::from))
        .context("Failed to read static file")
}

pub fn read_all_files_in_dir(path: &str) -> Result<Vec<(String, String)>> {
    STATIC_DIR
        .get_dir(path)
        .map(|dir| {
            dir.files()
                .filter_map(|file| {
                    let name = file.path().display().to_string();
                    file.contents_utf8().map(|contents| (name, contents.to_string()))
                })
                .collect()
        })
        .context("Failed to read all files in static directory")
}
