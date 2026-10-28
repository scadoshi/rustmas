//! An event year, and the clock rules for which days of it have unlocked.

use chrono::{DateTime, Datelike, Timelike, Utc};
use thiserror::Error;

use crate::domain::address::day::FINAL_DAY;

/// The first Advent of Code.
pub const FIRST_YEAR: i32 = 2015;

/// The month every event runs in, as [`Datelike::month`] counts it.
const DECEMBER: u32 = 12;

/// The UTC hour each puzzle unlocks at, which is midnight UTC-5.
const UNLOCK_HOUR_UTC: u32 = 5;

/// The first event to run twelve days. Every later one does too.
const FIRST_SHORT_EVENT: i32 = 2025;

/// The last day of an event from [`FIRST_SHORT_EVENT`] on, rather than
/// [`FINAL_DAY`].
const SHORT_EVENT_FINAL_DAY: i32 = 12;

/// Returned when a year falls outside the published events.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("year {given} is outside {FIRST_YEAR}..={latest}")]
pub struct YearOutOfRange {
    /// The year as asked for.
    pub given: i32,
    /// [`Year::latest`] at the time of asking.
    pub latest: i32,
}

/// A validated event year, from [`FIRST_YEAR`] through [`Year::latest`].
///
/// The upper bound moves with the clock: a year joins once its day 1 unlocks,
/// so a `Year` built in November cannot be the coming December's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Year(i32);

impl Year {
    /// The calendar year, like `2015`.
    pub fn value(&self) -> i32 {
        self.0
    }

    /// Bounded by the latest *published* event, not the calendar year.
    ///
    /// # Errors
    ///
    /// [`YearOutOfRange`] when `year` is before [`FIRST_YEAR`] or after
    /// [`Year::latest`].
    pub fn new(year: i32) -> Result<Self, YearOutOfRange> {
        Self::new_at(year, Utc::now())
    }

    /// [`Year::new`] as of `now`.
    pub(crate) fn new_at(year: i32, now: DateTime<Utc>) -> Result<Self, YearOutOfRange> {
        let latest = Self::latest_at(now);
        if year > latest || year < FIRST_YEAR {
            return Err(YearOutOfRange {
                given: year,
                latest,
            });
        }
        Ok(Self(year))
    }

    /// Every event with a puzzle unlocked as of `now`, oldest first.
    pub(crate) fn all_at(now: DateTime<Utc>) -> impl Iterator<Item = Self> {
        (FIRST_YEAR..=Self::latest_at(now)).map(Self)
    }

    /// The latest event with a puzzle unlocked. Until day 1 unlocks at 05:00
    /// UTC on December 1, that is last year.
    pub fn latest() -> i32 {
        Self::latest_at(Utc::now())
    }

    /// [`Year::latest`] as of `now`.
    pub(crate) fn latest_at(now: DateTime<Utc>) -> i32 {
        if unlocked_this_december(now).is_some() {
            now.year()
        } else {
            now.year().saturating_sub(1)
        }
    }

    /// Days in the whole event: 25 through 2024, 12 from 2025 on.
    pub fn days_in(&self) -> i32 {
        if self.0 >= FIRST_SHORT_EVENT {
            SHORT_EVENT_FINAL_DAY
        } else {
            FINAL_DAY
        }
    }

    /// Days of this event unlocked as of `now`: all of [`Year::days_in`],
    /// except for the event running at `now`.
    pub(crate) fn unlocked_days_at(&self, now: DateTime<Utc>) -> i32 {
        match unlocked_this_december(now) {
            Some(unlocked) if self.0 == now.year() => unlocked.min(self.days_in()),
            _ => self.days_in(),
        }
    }
}

/// How many days of December have unlocked at `now`, or `None` outside
/// December and before day 1 unlocks.
fn unlocked_this_december(now: DateTime<Utc>) -> Option<i32> {
    if now.month() != DECEMBER {
        return None;
    }
    let unlocked = if now.hour() < UNLOCK_HOUR_UTC {
        now.day0()
    } else {
        now.day()
    };
    i32::try_from(unlocked)
        .ok()
        .filter(|&unlocked| unlocked > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_years_outside_published_events() {
        assert!(Year::new(FIRST_YEAR - 1).is_err());
        assert!(Year::new(Year::latest() + 1).is_err());
        assert!(Year::new(FIRST_YEAR).is_ok());
        assert!(Year::new(Year::latest()).is_ok());
    }

    /// The message names the value and the live bounds, since it is the only
    /// clue the user gets about what to type instead.
    #[test]
    fn the_error_names_the_value_and_the_bounds() {
        let error = Year::new(2030).unwrap_err();
        assert_eq!(
            error.to_string(),
            format!("year 2030 is outside 2015..={}", Year::latest())
        );
    }

    /// 2025 ran twelve days rather than the twenty five before it.
    #[test]
    fn knows_how_long_each_event_ran() {
        assert_eq!(Year::new(2025).unwrap().days_in(), 12);
        assert_eq!(Year::new(2015).unwrap().days_in(), 25);
    }

    fn at(time: &str) -> DateTime<Utc> {
        time.parse().expect("test times are RFC 3339")
    }

    /// Puzzles unlock at midnight UTC-5, so an event starts at 05:00 UTC on
    /// December 1 and the year before stays the latest until then.
    #[test]
    fn the_latest_event_starts_when_day_one_unlocks() {
        assert_eq!(Year::latest_at(at("2026-11-30T23:00:00Z")), 2025);
        assert_eq!(Year::latest_at(at("2026-12-01T04:59:59Z")), 2025);
        assert_eq!(Year::latest_at(at("2026-12-01T05:00:00Z")), 2026);
        assert_eq!(Year::latest_at(at("2027-01-01T03:00:00Z")), 2026);
    }

    /// During an event only the days whose midnight UTC-5 has passed exist,
    /// capped at the event's length.
    #[test]
    fn unlocked_days_follow_the_clock_during_the_event() {
        let now = at("2026-12-03T04:59:59Z");
        let year = Year::new_at(2026, now).unwrap();
        assert_eq!(year.unlocked_days_at(now), 2);
        assert_eq!(year.unlocked_days_at(at("2026-12-03T05:00:00Z")), 3);
        assert_eq!(year.unlocked_days_at(at("2026-12-25T12:00:00Z")), 12);

        let past = Year::new_at(2024, now).unwrap();
        assert_eq!(past.unlocked_days_at(now), 25);
    }

    /// Every event from 2025 on runs twelve days.
    #[test]
    fn events_from_2025_on_run_twelve_days() {
        let later = Year::new_at(2026, at("2027-02-01T00:00:00Z")).unwrap();
        assert_eq!(later.days_in(), 12);
    }
}
