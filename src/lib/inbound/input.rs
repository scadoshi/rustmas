use crate::{
    domain::address::Day,
    outbound::{
        client::{AocClient, aoc::cookie_from_env},
        store::{
            self,
            cache::{Entry, Input, Instructions},
        },
    },
};

/// Returns `day`'s cached input and instructions, downloading what is missing.
///
/// Nothing cached means both are fetched. An input from a different session is
/// refetched while its instructions are kept, since puzzle text is the same for
/// everyone. `client` is built only when something is actually downloaded, so a
/// run over cached days stays offline.
pub fn ensure_entry(client: &mut Option<AocClient>, day: &Day) -> anyhow::Result<Entry> {
    // Absent when no cookie is configured, which leaves a cached entry usable
    // rather than unverifiable and therefore unusable.
    let cookie = cookie_from_env().ok();

    let cached = store::read_entry(day)?;
    if let Some(entry) = cached {
        let same_session = cookie
            .as_deref()
            .is_none_or(|cookie| entry.input.is_from(cookie));
        if same_session {
            return Ok(entry);
        }
        let entry = Entry {
            input: fetch_input(client, day, cookie.as_deref())?,
            instructions: entry.instructions,
        };
        store::write_entry(day, &entry)?;
        println!(
            "refetched input for year {} day {}",
            day.year(),
            day.value()
        );
        return Ok(entry);
    }

    let input = fetch_input(client, day, cookie.as_deref())?;
    let (part_one, part_two) = connected(client)?.get_instructions(day)?;
    let entry = Entry {
        input,
        instructions: Instructions { part_one, part_two },
    };
    store::write_entry(day, &entry)?;
    println!("fetched year {} day {}", day.year(), day.value());
    Ok(entry)
}

/// Downloads `day`'s input and tags it with the session that got it.
fn fetch_input(
    client: &mut Option<AocClient>,
    day: &Day,
    cookie: Option<&str>,
) -> anyhow::Result<Input> {
    let data = connected(client)?.get_input(day)?;
    let cookie = match cookie {
        Some(cookie) => cookie.to_string(),
        None => cookie_from_env()?,
    };
    Ok(Input::new(cookie, data))
}

/// Builds the client on first use, so nothing offline pays for a connection.
fn connected(client: &mut Option<AocClient>) -> anyhow::Result<&AocClient> {
    if client.is_none() {
        *client = Some(AocClient::from_env()?);
    }
    Ok(client.as_ref().expect("built just above"))
}
