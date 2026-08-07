use crate::{
    domain::address::Day,
    outbound::{client::AocClient, store},
};

/// Returns `day`'s input, downloading it if it is not on disk.
///
/// `client` is built on first download, so a run over cached inputs never needs
/// a cookie. Inputs never change once published, so a cached file is used
/// as-is. AOC asks that you not re-download.
pub fn ensure_input(client: &mut Option<AocClient>, day: &Day) -> anyhow::Result<String> {
    if let Some(input) = store::read_input(day)? {
        return Ok(input);
    }

    if client.is_none() {
        *client = Some(AocClient::from_env()?);
    }
    let client = client.as_ref().expect("built just above");

    let input = client.get_input(day)?;
    store::write_input(day, &input)?;
    println!("fetched year {} day {}", day.year(), day.value());
    Ok(input)
}
