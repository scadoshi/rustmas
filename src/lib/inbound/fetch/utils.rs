use anyhow::{Context, bail};
use std::{fs::create_dir_all, path::Path};

/// Creates `path` and its parents, erroring rather than clobbering a file.
///
/// `name` is for narration only.
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
