//! A day's input and instructions, from the cache when it has them and the
//! site when it does not.

use std::path::Path;

use crate::domain::address::Day;
use crate::outbound::client::aoc_client::LazyAocClient;
use crate::outbound::client::environment::Environment;
use crate::outbound::client::session_cookie::SessionCookie;
use crate::outbound::store;
use crate::outbound::store::cache::{Entry, Input, Instructions};

/// Returns `day`'s cached input and instructions, downloading what is missing.
///
/// Missing `part_two.md` means incomplete, so it is rechecked every run until
/// part one is solved. An event's final day is the exception, its second star
/// being awarded rather than puzzled. An input from another session is refetched.
#[expect(
    clippy::print_stdout,
    reason = "a CLI: each download is reported as it lands"
)]
pub(crate) fn ensure_entry(client: &mut LazyAocClient, day: &Day) -> anyhow::Result<Entry> {
    // Absent when no cookie is configured, leaving a cached entry usable.
    let cookie = Environment::cookie_if_set()?;

    let Some(cached) = store::read_entry(day)? else {
        let aoc = client.connected()?;
        let entry = fetch_new_entry(
            &store::cache_root()?,
            day,
            cookie.as_ref(),
            || tagged(aoc.fetch_input(day)?, cookie.as_ref()),
            || aoc.fetch_instructions(day),
        )?;
        println!("fetched {day}");
        return Ok(entry);
    };

    let stale_session = cookie
        .as_ref()
        .is_some_and(|cookie| !cached.input.is_from(cookie));
    // No cookie means nothing to ask with, so an incomplete cache stays.
    let chase_part_two =
        cached.instructions.part_two.is_none() && day.has_second_puzzle() && cookie.is_some();

    if !stale_session && !chase_part_two {
        return Ok(cached);
    }

    let input = if stale_session {
        let input = tagged(client.connected()?.fetch_input(day)?, cookie.as_ref())?;
        println!("refetched input for {day}");
        input
    } else {
        cached.input
    };

    let instructions = if chase_part_two {
        let (part_one, part_two) = client.connected()?.fetch_instructions(day)?;
        if part_two.is_some() {
            println!("part two unlocked for {day}");
        }
        Instructions { part_one, part_two }
    } else {
        cached.instructions
    };

    let entry = Entry {
        input,
        instructions,
    };
    store::write_entry(day, &entry)?;
    Ok(entry)
}

/// Downloads a day with nothing complete cached under `root` and stores it.
///
/// The input is stored as soon as it arrives, before the page is asked for,
/// so a failed page leaves it on disk and the next run downloads only the
/// page. A stored input from another session is downloaded again.
/// `fetch_input` and `fetch_instructions` do the downloading, which keeps the
/// network out of what gets stored when.
fn fetch_new_entry(
    root: &Path,
    day: &Day,
    cookie: Option<&SessionCookie>,
    fetch_input: impl FnOnce() -> anyhow::Result<Input>,
    fetch_instructions: impl FnOnce() -> anyhow::Result<(String, Option<String>)>,
) -> anyhow::Result<Entry> {
    let stored = store::read_input_in(root, day)?
        .filter(|input| cookie.is_some_and(|cookie| input.is_from(cookie)));
    let input = if let Some(input) = stored {
        input
    } else {
        let input = fetch_input()?;
        store::write_input_in(root, day, &input)?;
        input
    };

    let (part_one, part_two) = fetch_instructions()?;
    let entry = Entry {
        input,
        instructions: Instructions { part_one, part_two },
    };
    store::write_entry_in(root, day, &entry)?;
    Ok(entry)
}

/// Tags downloaded input `data` with the session that got it.
fn tagged(data: String, cookie: Option<&SessionCookie>) -> anyhow::Result<Input> {
    Ok(match cookie {
        Some(cookie) => Input::new(cookie, data),
        None => Input::new(Environment::cookie()?, data),
    })
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::*;
    use crate::domain::address::Year;

    /// A cache root per test, removed on drop.
    struct Temp(PathBuf);

    impl Temp {
        fn new(name: &str) -> Self {
            let path = std::env::temp_dir().join(format!("rustmas-input-test-{name}"));
            let _ = fs::remove_dir_all(&path);
            Self(path)
        }
    }

    impl Drop for Temp {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn day() -> Day {
        Day::new(1, Year::new(2015).unwrap()).unwrap()
    }

    fn cookie(fill: &str) -> SessionCookie {
        SessionCookie::try_from(fill.repeat(128).as_str()).unwrap()
    }

    fn page() -> (String, Option<String>) {
        ("## one".to_string(), None)
    }

    /// AOC asks that inputs not be downloaded twice, so one that arrived is
    /// kept even when the page after it fails, and the next run fetches only
    /// the page.
    #[test]
    fn a_failed_page_fetch_keeps_the_input() {
        let temp = Temp::new("failed-page");
        let cookie = cookie("a");

        let first = fetch_new_entry(
            &temp.0,
            &day(),
            Some(&cookie),
            || Ok(Input::new(&cookie, "()()")),
            || Err(anyhow::anyhow!("page down")),
        );
        assert!(first.is_err());

        let second = fetch_new_entry(
            &temp.0,
            &day(),
            Some(&cookie),
            || panic!("the stored input is downloaded again"),
            || Ok(page()),
        )
        .unwrap();
        assert_eq!(second.input.data(), "()()");
        assert_eq!(second.instructions.part_one, "## one");
    }

    /// A stored input from another session is not this account's, so it is
    /// downloaded again.
    #[test]
    fn a_stored_input_from_another_session_is_refetched() {
        let temp = Temp::new("other-session");
        let (old, new) = (cookie("a"), cookie("b"));
        store::write_input_in(&temp.0, &day(), &Input::new(&old, "()()")).unwrap();

        let entry = fetch_new_entry(
            &temp.0,
            &day(),
            Some(&new),
            || Ok(Input::new(&new, "))((")),
            || Ok(page()),
        )
        .unwrap();
        assert_eq!(entry.input.data(), "))((");
        assert!(entry.input.is_from(&new));
    }
}
