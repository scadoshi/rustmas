//! Where downloaded things live on disk.

use crate::domain::address::Day;
use anyhow::{Context, bail};
use std::{
    fs::{create_dir_all, read_to_string, write},
    path::{Path, PathBuf},
};

pub const INPUT_PATH: &str = "inputs";

/// The repo root, from the env var cargo sets. Only present under `cargo run`.
pub fn project_root() -> anyhow::Result<PathBuf> {
    Ok(PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").context(
        "CARGO_MANIFEST_DIR is unset, so the project root is unknown",
    )?))
}

/// Where `day`'s input lives: `inputs/<year>/<NN>.txt`, zero padded.
pub fn input_path(day: &Day) -> anyhow::Result<PathBuf> {
    Ok(project_root()?
        .join(INPUT_PATH)
        .join(day.year().to_string())
        .join(format!("{:02}.txt", day.value())))
}

/// Reads `day`'s input, or `None` when it has not been downloaded.
pub fn read_input(day: &Day) -> anyhow::Result<Option<String>> {
    let path = input_path(day)?;
    if !path.is_file() {
        return Ok(None);
    }
    read_to_string(&path)
        .map(Some)
        .with_context(|| format!("failed to read input: {}", path.display()))
}

/// Writes `day`'s input, creating the year directory if needed.
pub fn write_input(day: &Day, input: &str) -> anyhow::Result<()> {
    let path = input_path(day)?;
    if let Some(parent) = path.parent() {
        ensure_dir(parent)?;
    }
    write(&path, input).with_context(|| format!("failed to write input: {}", path.display()))
}

/// Creates `path` and its parents, no-opping if it already exists. Errors
/// rather than clobbering when a non-directory is in the way.
pub fn ensure_dir(path: &Path) -> anyhow::Result<()> {
    if path.is_dir() {
        return Ok(());
    }
    if path.exists() {
        bail!("path exists but is not a dir: {}", path.display());
    }
    create_dir_all(path).with_context(|| format!("failed to create dir: {}", path.display()))
}
