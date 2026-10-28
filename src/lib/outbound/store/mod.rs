//! Where downloaded things live on disk. One directory per day, every file
//! readable on its own. `context/architecture.md` has the layout.

pub(crate) mod cache;

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, bail};

use crate::domain::address::Day;
use crate::outbound::store::cache::{Entry, Input, Instructions};

pub(crate) const CACHE_PATH: &str = "cache";

const INPUT_FILE: &str = "input.txt";
const SESSION_FILE: &str = "session";
const PART_ONE_FILE: &str = "part_one.md";
const PART_TWO_FILE: &str = "part_two.md";
/// Added to a file's name for the copy written before it replaces the file.
const TEMP_SUFFIX: &str = ".tmp";

/// The repo root, from the env var cargo sets. Only present under `cargo run`.
pub(crate) fn project_root() -> anyhow::Result<PathBuf> {
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
/// A missing session file reads as `None`, since an input nothing can vouch
/// for is one to fetch again.
pub(crate) fn read_entry(day: &Day) -> anyhow::Result<Option<Entry>> {
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

/// Writes `day`'s cache, creating the directory if needed. Only files whose
/// contents differ are written.
pub(crate) fn write_entry(day: &Day, entry: &Entry) -> anyhow::Result<()> {
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

/// Reads a file, or `None` when it is missing or blank.
///
/// Blank counts as missing, so a half-written file gets fetched again.
fn read_opt(path: &Path) -> anyhow::Result<Option<String>> {
    if !path.is_file() {
        return Ok(None);
    }
    let contents =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    Ok(Some(contents).filter(|contents| !contents.trim().is_empty()))
}

/// Writes `contents` to `path`, unless it already holds exactly that.
///
/// Goes through a temp file beside `path` and a rename, so a crash leaves the
/// old file or the new one and never half of either.
fn write_file(path: &Path, contents: &str) -> anyhow::Result<()> {
    if fs::read_to_string(path).is_ok_and(|current| current == contents) {
        return Ok(());
    }
    let mut temp = path.as_os_str().to_owned();
    temp.push(TEMP_SUFFIX);
    let temp = PathBuf::from(temp);
    fs::write(&temp, contents).with_context(|| format!("failed to write {}", temp.display()))?;
    fs::rename(&temp, path).with_context(|| format!("failed to write {}", path.display()))
}

/// Creates `path` and its parents, erroring rather than clobbering a file.
pub(crate) fn ensure_dir(path: &Path) -> anyhow::Result<()> {
    if path.is_dir() {
        return Ok(());
    }
    if path.exists() {
        bail!("path exists but is not a dir: {}", path.display());
    }
    fs::create_dir_all(path).with_context(|| format!("failed to create dir: {}", path.display()))
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime};

    use super::*;
    use crate::domain::address::Year;

    /// A root per test, so nothing touches the real cache or another test.
    struct Temp(PathBuf);

    impl Temp {
        fn new(name: &str) -> Self {
            let path = std::env::temp_dir().join(format!("rustmas-test-{name}"));
            let _ = fs::remove_dir_all(&path);
            Self(path)
        }
    }

    impl Drop for Temp {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
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

    /// A blank file is a half-written one, so it reads as absent and gets
    /// fetched again rather than counting as content.
    #[test]
    fn blank_part_two_reads_as_missing() {
        let temp = Temp::new("blank-part-two");
        let day = Day::new(1, Year::new(2015).unwrap()).unwrap();

        write_entry_in(&temp.0, &day, &entry("cookie")).unwrap();
        fs::write(day_path_in(&temp.0, &day).join(PART_TWO_FILE), "  \n").unwrap();

        let read = read_entry_in(&temp.0, &day).unwrap().unwrap();
        assert!(read.instructions.part_two.is_none());
    }

    /// An input nothing can vouch for is one to fetch again.
    #[test]
    fn entry_without_a_session_reads_as_missing() {
        let temp = Temp::new("no-session");
        let day = Day::new(1, Year::new(2015).unwrap()).unwrap();

        write_entry_in(&temp.0, &day, &entry("cookie")).unwrap();
        fs::remove_file(day_path_in(&temp.0, &day).join(SESSION_FILE)).unwrap();

        assert!(read_entry_in(&temp.0, &day).unwrap().is_none());
    }

    #[test]
    fn ensure_dir_refuses_to_clobber_a_file() {
        let temp = Temp::new("clobber");
        fs::create_dir_all(&temp.0).unwrap();
        let path = temp.0.join("in-the-way");
        fs::write(&path, "").unwrap();

        assert!(ensure_dir(&path).is_err());
    }

    /// A part two arriving changes one file, so the input keeps its bytes and
    /// its timestamp.
    #[test]
    fn rewriting_leaves_unchanged_files_alone() {
        let temp = Temp::new("unchanged-files");
        let day = Day::new(1, Year::new(2015).unwrap()).unwrap();

        let mut first = entry("cookie");
        first.instructions.part_two = None;
        write_entry_in(&temp.0, &day, &first).unwrap();

        let input = day_path_in(&temp.0, &day).join(INPUT_FILE);
        let marked = SystemTime::UNIX_EPOCH + Duration::from_secs(1);
        fs::File::options()
            .write(true)
            .open(&input)
            .unwrap()
            .set_modified(marked)
            .unwrap();

        write_entry_in(&temp.0, &day, &entry("cookie")).unwrap();

        assert_eq!(fs::metadata(&input).unwrap().modified().unwrap(), marked);
        let read = read_entry_in(&temp.0, &day).unwrap().unwrap();
        assert_eq!(read.instructions.part_two.as_deref(), Some("## two"));
    }

    /// A changed file is replaced whole by a rename, never truncated in place,
    /// so anything holding the old file still sees it complete.
    #[test]
    fn a_changed_file_is_replaced_rather_than_truncated() {
        let temp = Temp::new("replaced-file");
        let day = Day::new(1, Year::new(2015).unwrap()).unwrap();
        write_entry_in(&temp.0, &day, &entry("cookie")).unwrap();

        let dir = day_path_in(&temp.0, &day);
        let held = temp.0.join("held");
        fs::hard_link(dir.join(INPUT_FILE), &held).unwrap();

        let mut changed = entry("cookie");
        changed.input = Input::new("cookie", "))((");
        write_entry_in(&temp.0, &day, &changed).unwrap();

        assert_eq!(fs::read_to_string(&held).unwrap(), "()()");
        assert_eq!(fs::read_to_string(dir.join(INPUT_FILE)).unwrap(), "))((");
        let leftovers: Vec<_> = fs::read_dir(&dir)
            .unwrap()
            .map(|file| file.unwrap().file_name())
            .filter(|name| name.to_string_lossy().ends_with(TEMP_SUFFIX))
            .collect();
        assert_eq!(leftovers, Vec::<std::ffi::OsString>::new());
    }
}
