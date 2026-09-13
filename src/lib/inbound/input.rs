use crate::{
    domain::address::Day,
    outbound::{
        client::{aoc_client::LazyAocClient, environment::Environment},
        store::{
            self,
            cache::{Entry, Input, Instructions},
        },
    },
};

/// Returns `day`'s cached input and instructions, downloading what is missing.
///
/// Missing `part_two.md` means incomplete, so it is rechecked every run until
/// part one is solved. Day 25 is the exception, its second star being awarded
/// rather than puzzled. An input from another session is refetched.
pub fn ensure_entry(client: &mut LazyAocClient, day: &Day) -> anyhow::Result<Entry> {
    // Absent when no cookie is configured, leaving a cached entry usable.
    let cookie = Environment::cookie_if_set()?;

    let Some(cached) = store::read_entry(day)? else {
        let input = fetch_input(client, day, cookie.as_deref())?;
        let (part_one, part_two) = client.connected()?.get_instructions(day)?;
        let entry = Entry {
            input,
            instructions: Instructions { part_one, part_two },
        };
        store::write_entry(day, &entry)?;
        println!("fetched year {} day {}", day.year(), day.value());
        return Ok(entry);
    };

    let stale_session = cookie
        .as_deref()
        .is_some_and(|cookie| !cached.input.is_from(cookie));
    // No cookie means nothing to ask with, so an incomplete cache stays.
    let chase_part_two =
        cached.instructions.part_two.is_none() && day.has_second_puzzle() && cookie.is_some();

    if !stale_session && !chase_part_two {
        return Ok(cached);
    }

    let input = if stale_session {
        let input = fetch_input(client, day, cookie.as_deref())?;
        println!(
            "refetched input for year {} day {}",
            day.year(),
            day.value()
        );
        input
    } else {
        cached.input
    };

    let instructions = if chase_part_two {
        let (part_one, part_two) = client.connected()?.get_instructions(day)?;
        if part_two.is_some() {
            println!(
                "part two unlocked for year {} day {}",
                day.year(),
                day.value()
            );
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

/// Downloads `day`'s input and tags it with the session that got it.
fn fetch_input(
    client: &mut LazyAocClient,
    day: &Day,
    cookie: Option<&str>,
) -> anyhow::Result<Input> {
    let data = client.connected()?.get_input(day)?;
    let cookie = match cookie {
        Some(cookie) => cookie.to_string(),
        None => Environment::cookie()?,
    };
    Ok(Input::new(cookie, data))
}
