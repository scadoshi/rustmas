//! Where downloaded things live on disk.

pub mod cache;

use crate::{domain::address::Day, outbound::store::cache::Entry};
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

/// Where `day`'s cached input and instructions live, zero padded.
pub fn entry_path(day: &Day) -> anyhow::Result<PathBuf> {
    Ok(project_root()?
        .join(INPUT_PATH)
        .join(day.year().to_string())
        .join(format!("{:02}.json", day.value())))
}

/// Reads `day`'s cache entry, or `None` when nothing has been downloaded.
///
/// Returns what is on disk whatever session it came from. Deciding whether the
/// input still belongs to you is [`Entry::input`] and `is_from`, so instructions
/// stay usable across a cookie change.
pub fn read_entry(day: &Day) -> anyhow::Result<Option<Entry>> {
    let path = entry_path(day)?;
    if !path.is_file() {
        return Ok(None);
    }
    let json = read_to_string(&path)
        .with_context(|| format!("failed to read entry: {}", path.display()))?;
    serde_json::from_str(&json)
        .map(Some)
        .with_context(|| format!("failed to parse entry: {}", path.display()))
}

/// Writes `day`'s cache entry, creating the year directory if needed.
pub fn write_entry(day: &Day, entry: &Entry) -> anyhow::Result<()> {
    let path = entry_path(day)?;
    if let Some(parent) = path.parent() {
        ensure_dir(parent)?;
    }
    let json = serde_json::to_string_pretty(entry)
        .with_context(|| format!("failed to serialise entry: {}", path.display()))?;
    write(&path, json).with_context(|| format!("failed to write entry: {}", path.display()))
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
