use anyhow::{Context, bail};
use rustmas::{day::Day, session::Session};
use std::{fs::create_dir_all, path::Path};

/// Ensures a directory exists at `path`, creating parents as needed.
///
/// Idempotent: no-ops if the directory already exists. Errors (rather than
/// clobbering) if a non-directory already sits at `path`. `name` is used only
/// for narration.
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

/// Downloads the puzzle input for `day` to `path`, unless it is already there.
///
/// Inputs never change once published, so an existing file is treated as a
/// cached download and no request is made (AOC asks that you not re-download).
/// Errors (rather than clobbering) if a non-file already sits at `path`.
pub fn download_input(session: &Session, day: &Day, path: &Path) -> anyhow::Result<()> {
    if path.is_file() {
        println!("input already cached: {}", path.display());
        return Ok(());
    }
    if path.exists() {
        bail!("input path exists but is not a file: {}", path.display());
    }
    let input = session
        .get_input(day)
        .with_context(|| format!("failed to download input: {}", path.display()))?;
    std::fs::write(path, input)
        .with_context(|| format!("failed to write input: {}", path.display()))?;
    println!("downloaded input: {}", path.display());
    Ok(())
}
