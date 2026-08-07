//! Where downloaded things live on disk.
//!
//! One directory per day, every file readable on its own:
//!
//! ```text
//! cache/2015/01/input.txt     the puzzle input, verbatim
//! cache/2015/01/session       hash of the cookie that fetched it
//! cache/2015/01/part_one.md   puzzle text
//! cache/2015/01/part_two.md   puzzle text, absent until part one is solved
//! ```

pub mod cache;

use crate::{
    domain::address::Day,
    outbound::store::cache::{Entry, Input, Instructions},
};
use anyhow::{Context, bail};
use std::{
    fs::{create_dir_all, read_to_string, write},
    path::{Path, PathBuf},
};

pub const CACHE_PATH: &str = "cache";

const INPUT_FILE: &str = "input.txt";
const SESSION_FILE: &str = "session";
const PART_ONE_FILE: &str = "part_one.md";
const PART_TWO_FILE: &str = "part_two.md";

/// The repo root, from the env var cargo sets. Only present under `cargo run`.
pub fn project_root() -> anyhow::Result<PathBuf> {
    Ok(PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").context(
        "CARGO_MANIFEST_DIR is unset, so the project root is unknown",
    )?))
}

/// Where `day`'s cache lives: `cache/<year>/<NN>/`, zero padded.
pub fn day_path(day: &Day) -> anyhow::Result<PathBuf> {
    Ok(project_root()?
        .join(CACHE_PATH)
        .join(day.year().to_string())
        .join(format!("{:02}", day.value())))
}

/// Reads `day`'s cache, or `None` when nothing has been downloaded.
///
/// Returns what is on disk whatever session it came from, so instructions stay
/// usable across a cookie change. A missing session file reads as `None` too,
/// since an input nothing can vouch for is one to fetch again.
pub fn read_entry(day: &Day) -> anyhow::Result<Option<Entry>> {
    let dir = day_path(day)?;

    let (Some(data), Some(hash), Some(part_one)) = (
        read_opt(&dir.join(INPUT_FILE))?,
        read_opt(&dir.join(SESSION_FILE))?,
        read_opt(&dir.join(PART_ONE_FILE))?,
    ) else {
        return Ok(None);
    };

    Ok(Some(Entry {
        input: Input::from_parts(hash.trim(), data),
        instructions: Instructions {
            part_one,
            part_two: read_opt(&dir.join(PART_TWO_FILE))?,
        },
    }))
}

/// Writes `day`'s cache, creating the directory if needed.
pub fn write_entry(day: &Day, entry: &Entry) -> anyhow::Result<()> {
    let dir = day_path(day)?;
    ensure_dir(&dir)?;

    write_file(&dir.join(INPUT_FILE), entry.input.data())?;
    write_file(&dir.join(SESSION_FILE), entry.input.hash())?;
    write_file(&dir.join(PART_ONE_FILE), &entry.instructions.part_one)?;
    if let Some(part_two) = &entry.instructions.part_two {
        write_file(&dir.join(PART_TWO_FILE), part_two)?;
    }
    Ok(())
}

/// Reads a file, or `None` when it is not there.
fn read_opt(path: &Path) -> anyhow::Result<Option<String>> {
    if !path.is_file() {
        return Ok(None);
    }
    read_to_string(path)
        .map(Some)
        .with_context(|| format!("failed to read {}", path.display()))
}

fn write_file(path: &Path, contents: &str) -> anyhow::Result<()> {
    write(path, contents).with_context(|| format!("failed to write {}", path.display()))
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
