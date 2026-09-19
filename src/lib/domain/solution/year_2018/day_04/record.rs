use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidRecord {
    #[error("expected a timestamp in brackets")]
    MissingTimestamp,
    #[error("expected a minute in the timestamp")]
    MissingMinute,
    #[error("expected a shift, a sleep or a wake")]
    UnknownEvent,
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    BeginsShift(u32),
    FallsAsleep,
    WakesUp,
}

/// One line of the log: when it happened and what it was.
///
/// The timestamp stays as text because it is only ever sorted, and the format
/// puts the fields largest first, so ordering the strings orders the events.
#[derive(Debug, Clone)]
pub struct Record {
    pub timestamp: String,
    pub minute: usize,
    pub event: Event,
}

impl TryFrom<&str> for Record {
    type Error = InvalidRecord;

    /// Parses `[1518-11-01 00:05] falls asleep`.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
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
    fn record_try_from_str_ok() {
        let record = Record::try_from("[1518-11-01 00:05] falls asleep").unwrap();
        assert_eq!(record.timestamp, "1518-11-01 00:05");
        assert_eq!(record.minute, 5);
        assert_eq!(record.event, Event::FallsAsleep);

        let record = Record::try_from("[1518-11-01 00:00] Guard #10 begins shift").unwrap();
        assert_eq!(record.minute, 0);
        assert_eq!(record.event, Event::BeginsShift(10));

        let record = Record::try_from("[1518-11-01 00:25] wakes up").unwrap();
        assert_eq!(record.event, Event::WakesUp);
    }

    #[test]
    fn record_try_from_str_err() {
        assert!(matches!(
            Record::try_from("1518-11-01 00:05 falls asleep"),
            Err(InvalidRecord::MissingTimestamp)
        ));
        assert!(matches!(
            Record::try_from("[1518-11-01] falls asleep"),
            Err(InvalidRecord::MissingMinute)
        ));
        assert!(matches!(
            Record::try_from("[1518-11-01 00:05] naps"),
            Err(InvalidRecord::UnknownEvent)
        ));
        assert!(matches!(
            Record::try_from("[1518-11-01 00:xx] wakes up"),
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
