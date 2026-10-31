//! One line of the guard log and what it records.

use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

/// Returned when a line does not read as a log record.
#[derive(Debug, Error)]
pub(super) enum InvalidRecord {
    /// The line does not start with `[...] `.
    #[error("expected a timestamp in brackets")]
    MissingTimestamp,
    /// The timestamp has no `:` before the minute.
    #[error("expected a minute in the timestamp")]
    MissingMinute,
    /// The text after the timestamp is none of the three events.
    #[error("expected a shift, a sleep or a wake")]
    UnknownEvent,
    /// The minute or the guard id is not a whole number in range.
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

/// What a log line says happened.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Event {
    /// The guard with this id comes on duty. Every sleep after it, until the next shift, is theirs.
    BeginsShift(u32),
    /// The guard on duty is asleep from this minute.
    FallsAsleep,
    /// The guard on duty is awake from this minute, so the last minute asleep is the one before.
    WakesUp,
}

/// One line of the log: when it happened and what it was.
///
/// The timestamp stays as text because it is only ever sorted, and the format
/// puts the fields largest first, so ordering the strings orders the events.
#[derive(Debug, Clone)]
pub(super) struct Record {
    /// The text between the brackets, like `1518-11-01 00:05`.
    pub(super) timestamp: String,
    /// The timestamp's minute past the hour. Not range-checked.
    pub(super) minute: usize,
    /// What happened at that time.
    pub(super) event: Event,
}

impl FromStr for Record {
    type Err = InvalidRecord;

    /// Parses `[1518-11-01 00:05] falls asleep`.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (timestamp, rest) = value
            .trim()
            .strip_prefix('[')
            .and_then(|rest| rest.split_once("] "))
            .ok_or(InvalidRecord::MissingTimestamp)?;
        let minute = timestamp
            .rsplit_once(':')
            .ok_or(InvalidRecord::MissingMinute)?
            .1
            .parse()?;
        let event = if let Some(id) = rest
            .strip_prefix("Guard #")
            .and_then(|rest| rest.strip_suffix(" begins shift"))
        {
            Event::BeginsShift(id.parse()?)
        } else if rest.starts_with("falls asleep") {
            Event::FallsAsleep
        } else if rest.starts_with("wakes up") {
            Event::WakesUp
        } else {
            return Err(InvalidRecord::UnknownEvent);
        };
        Ok(Self {
            timestamp: timestamp.to_owned(),
            minute,
            event,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_from_str_ok() {
        let record = "[1518-11-01 00:05] falls asleep".parse::<Record>().unwrap();
        assert_eq!(record.timestamp, "1518-11-01 00:05");
        assert_eq!(record.minute, 5);
        assert_eq!(record.event, Event::FallsAsleep);

        let record = "[1518-11-01 00:00] Guard #10 begins shift"
            .parse::<Record>()
            .unwrap();
        assert_eq!(record.minute, 0);
        assert_eq!(record.event, Event::BeginsShift(10));

        let record = "[1518-11-01 00:25] wakes up".parse::<Record>().unwrap();
        assert_eq!(record.event, Event::WakesUp);
    }

    #[test]
    fn record_from_str_err() {
        assert!(matches!(
            "1518-11-01 00:05 falls asleep".parse::<Record>(),
            Err(InvalidRecord::MissingTimestamp)
        ));
        assert!(matches!(
            "[1518-11-01] falls asleep".parse::<Record>(),
            Err(InvalidRecord::MissingMinute)
        ));
        assert!(matches!(
            "[1518-11-01 00:05] naps".parse::<Record>(),
            Err(InvalidRecord::UnknownEvent)
        ));
        assert!(matches!(
            "[1518-11-01 00:xx] wakes up".parse::<Record>(),
            Err(InvalidRecord::ParseInt(_))
        ));
    }

    /// Ordering the text orders the events, which is why chrono is not here.
    #[test]
    fn timestamps_sort_as_text() {
        let mut stamps = ["1518-11-01 00:30", "1518-02-03 23:59", "1518-11-01 00:05"];
        stamps.sort_unstable();
        assert_eq!(
            stamps,
            ["1518-02-03 23:59", "1518-11-01 00:05", "1518-11-01 00:30"]
        );
    }
}
