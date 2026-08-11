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

/// `day`'s directory under an arbitrary cache root: `<year>/<NN>/`, zero padded.
fn day_path_in(root: &Path, day: &Day) -> PathBuf {
    root.join(day.year().to_string())
        .join(format!("{:02}", day.value()))
}

/// Reads `day`'s cache, or `None` when nothing has been downloaded.
///
/// Returns what is on disk whatever session it came from, but a missing session
/// file reads as `None`: an input nothing can vouch for is one to fetch again.
pub fn read_entry(day: &Day) -> anyhow::Result<Option<Entry>> {
    read_entry_in(&project_root()?.join(CACHE_PATH), day)
}

fn read_entry_in(root: &Path, day: &Day) -> anyhow::Result<Option<Entry>> {
    let dir = day_path_in(root, day);

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
    write_entry_in(&project_root()?.join(CACHE_PATH), day, entry)
}

fn write_entry_in(root: &Path, day: &Day, entry: &Entry) -> anyhow::Result<()> {
    let dir = day_path_in(root, day);
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

/// Creates `path` and its parents, erroring rather than clobbering a file.
pub fn ensure_dir(path: &Path) -> anyhow::Result<()> {
    if path.is_dir() {
        return Ok(());
    }
    if path.exists() {
        bail!("path exists but is not a dir: {}", path.display());
    }
    create_dir_all(path).with_context(|| format!("failed to create dir: {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::address::Year;
    use std::fs::remove_dir_all;

    /// A root per test, so nothing touches the real cache or another test.
    struct Temp(PathBuf);

    impl Temp {
        fn new(name: &str) -> Self {
            let path = std::env::temp_dir().join(format!("rustmas-test-{name}"));
            let _ = remove_dir_all(&path);
            Self(path)
        }
    }

    impl Drop for Temp {
        fn drop(&mut self) {
            let _ = remove_dir_all(&self.0);
        }
    }

    fn entry(cookie: &str) -> Entry {
        Entry {
            input: Input::new(cookie, "()()"),
            instructions: Instructions {
                part_one: "## one".to_string(),
                part_two: Some("## two".to_string()),
            },
        }
    }

    #[test]
    fn round_trips() {
        let temp = Temp::new("round-trips");
        let day = Day::new(1, Year::new(2015).unwrap()).unwrap();

        assert!(read_entry_in(&temp.0, &day).unwrap().is_none());

        write_entry_in(&temp.0, &day, &entry("cookie")).unwrap();
        let read = read_entry_in(&temp.0, &day).unwrap().unwrap();

        assert_eq!(read.input.data(), "()()");
        assert!(read.input.is_from("cookie"));
        assert_eq!(read.instructions.part_one, "## one");
        assert_eq!(read.instructions.part_two.as_deref(), Some("## two"));
    }

    /// Zero padded so a directory listing sorts the way a human reads it.
    #[test]
    fn pads_the_day() {
        let day = Day::new(1, Year::new(2015).unwrap()).unwrap();
        let path = day_path_in(Path::new("/cache"), &day);
        assert!(path.ends_with("2015/01"));
    }

    #[test]
    fn missing_part_two_reads_as_none() {
        let temp = Temp::new("no-part-two");
        let day = Day::new(1, Year::new(2015).unwrap()).unwrap();

        let mut entry = entry("cookie");
        entry.instructions.part_two = None;
        write_entry_in(&temp.0, &day, &entry).unwrap();

        let read = read_entry_in(&temp.0, &day).unwrap().unwrap();
        assert!(read.instructions.part_two.is_none());
    }

    /// An input nothing can vouch for is one to fetch again.
    #[test]
    fn entry_without_a_session_reads_as_missing() {
        let temp = Temp::new("no-session");
        let day = Day::new(1, Year::new(2015).unwrap()).unwrap();

        write_entry_in(&temp.0, &day, &entry("cookie")).unwrap();
        std::fs::remove_file(day_path_in(&temp.0, &day).join(SESSION_FILE)).unwrap();

        assert!(read_entry_in(&temp.0, &day).unwrap().is_none());
    }

    #[test]
    fn ensure_dir_refuses_to_clobber_a_file() {
        let temp = Temp::new("clobber");
        create_dir_all(&temp.0).unwrap();
        let path = temp.0.join("in-the-way");
        write(&path, "").unwrap();

        assert!(ensure_dir(&path).is_err());
    }
}
