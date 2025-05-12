use anyhow::{Context, Result};
use include_dir::{include_dir, Dir, File};

/// Embedded directory statically included at compile time from `./src/static`.
///
/// Used to provide access to rule files, templates, or other resources packaged into the binary.
pub static STATIC_DIR: Dir<'_> = include_dir!("./src/static");

/// Lists all file paths in a given subdirectory inside the embedded static directory.
///
/// # Arguments
///
/// * `path` - Path relative to the embedded static root.
///
/// # Returns
///
/// A vector of displayable paths as strings, or an error if the directory does not exist.
pub fn list_files(path: &str) -> Result<Vec<String>> {
    STATIC_DIR
        .get_dir(path)
        .map(|dir| dir.files().map(|f| f.path().display().to_string()).collect())
        .context("Failed to list static files")
}

/// Retrieves all `File` objects in a given embedded subdirectory.
///
/// # Arguments
///
/// * `path` - Path to a directory inside the embedded static directory.
///
/// # Returns
///
/// A vector of `File` references cloned from the directory, or an error if the directory is invalid.
pub fn get_all_files(path: &str) -> Result<Vec<File>> {
    STATIC_DIR
        .get_dir(path)
        .map(|dir| dir.files().cloned().collect())
        .context("Failed to get all static files")
}

/// Reads the contents of a single file inside the embedded static directory.
///
/// # Arguments
///
/// * `path` - Relative path to the file inside the embedded directory.
///
/// # Returns
///
/// File contents as a `String`, or an error if the file does not exist or is not UTF-8 encoded.
pub fn read_file(path: &str) -> Result<String> {
    STATIC_DIR
        .get_file(path)
        .and_then(|file| file.contents_utf8().map(String::from))
        .context("Failed to read static file")
}

/// Reads all UTF-8 file contents in a specified embedded directory.
///
/// # Arguments
///
/// * `path` - Path to the directory inside the embedded static directory.
///
/// # Returns
///
/// A vector of tuples, each containing the file path and its string content.
/// Returns an error if the directory cannot be found or read.
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
