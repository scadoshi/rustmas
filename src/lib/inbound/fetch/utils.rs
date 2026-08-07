use crate::{domain::address::Day, outbound::client::AocClient};
use anyhow::{Context, bail};
use std::{fs::create_dir_all, path::Path};

/// Creates `path` and its parents, no-opping if it already exists.
///
/// Errors rather than clobbering when a non-directory is in the way. `name` is
/// for narration only.
pub fn ensure_dir(name: &str, path: &Path) -> anyhow::Result<()> {
    if path.is_dir() {
        println!("{name} dir already exists: {}", path.display());
        return Ok(());
    }
    if path.exists() {
        bail!("{name} path exists but is not a dir: {}", path.display());
    }
    create_dir_all(path)
        .with_context(|| format!("failed to create {name} dir: {}", path.display()))?;
    println!("created {name} dir: {}", path.display());
    Ok(())
}

/// Downloads the input for `day` to `path`, unless it is already there.
///
/// Inputs never change, so an existing file skips the request entirely. AOC
/// asks that you not re-download.
pub fn download_input(client: &AocClient, day: &Day, path: &Path) -> anyhow::Result<()> {
    if path.is_file() {
        println!("input already cached: {}", path.display());
        return Ok(());
    }
    if path.exists() {
        bail!("input path exists but is not a file: {}", path.display());
    }
    let input = client
        .get_input(day)
        .with_context(|| format!("failed to download input: {}", path.display()))?;
    std::fs::write(path, input)
        .with_context(|| format!("failed to write input: {}", path.display()))?;
    println!("downloaded input: {}", path.display());
    Ok(())
}
