mod record;

use std::collections::HashMap;

use anyhow::Context;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::parse;
use crate::domain::solution::year_2018::day_04::record::{Event, Record};

/// Minutes in the midnight hour, the only hour any guard sleeps through.
const MINUTES: usize = 60;

#[derive(Debug)]
pub(crate) struct Puzzle {
    records: Vec<Record>,
}

impl Puzzle {
    /// How often each guard was asleep during each minute of the hour.
    ///
    /// A sleep runs from the minute it starts until the minute a wake ends it,
    /// so the log has to be read in order for a sleep to find its guard.
    fn sleep_by_minute(&self) -> anyhow::Result<HashMap<u32, [u32; MINUTES]>> {
        let mut slept: HashMap<u32, [u32; MINUTES]> = HashMap::new();
        let mut guard = None;
        let mut asleep_since = 0;
        for record in &self.records {
            match record.event {
                Event::BeginsShift(id) => guard = Some(id),
                Event::FallsAsleep => asleep_since = record.minute,
                Event::WakesUp => {
                    if let Some(id) = guard {
                        let minutes = slept.entry(id).or_insert([0; MINUTES]);
                        for slept_through in
                            minutes.iter_mut().take(record.minute).skip(asleep_since)
                        {
                            *slept_through = slept_through
                                .checked_add(1)
                                .context("a guard sleeps through a minute more than u32 counts")?;
                        }
                    }
                }
            }
        }
        Ok(slept)
    }
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        let mut records: Vec<Record> = parse::lines(input.as_ref())?;
        records.sort_unstable_by(|a, b| a.timestamp.cmp(&b.timestamp));
        Ok(Self { records })
    }

    /// The guard who sleeps most, times the minute they sleep through most.
    fn part_one(&self) -> anyhow::Result<Answer> {
        let slept = self.sleep_by_minute()?;
        let Some((guard, minutes)) = slept
            .iter()
            .max_by_key(|(_, minutes)| minutes.iter().copied().map(u64::from).sum::<u64>())
        else {
            return Ok(Answer::None);
        };
        let Some((minute, _)) = minutes.iter().enumerate().max_by_key(|(_, count)| **count) else {
            return Ok(Answer::None);
        };
        Ok(Answer::solved(id_times_minute(*guard, minute)?.to_string()))
    }

    /// The guard most often asleep on the same minute, times that minute.
    fn part_two(&self) -> anyhow::Result<Answer> {
        let slept = self.sleep_by_minute()?;
        let Some((guard, minute, _)) = slept
            .iter()
            .flat_map(|(guard, minutes)| {
                minutes
                    .iter()
                    .enumerate()
                    .map(move |(minute, count)| (*guard, minute, *count))
            })
            .max_by_key(|(_, _, count)| *count)
        else {
            return Ok(Answer::None);
        };
        Ok(Answer::solved(id_times_minute(guard, minute)?.to_string()))
    }
}

/// The answer both parts give: a guard's id times a minute of the hour.
fn id_times_minute(guard: u32, minute: usize) -> anyhow::Result<u64> {
    u64::from(guard)
        .checked_mul(u64::try_from(minute)?)
        .context("the guard id times the minute passes u64")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    /// The puzzle's example, shuffled to show the log is sorted first.
    #[test]
    fn example() {
        let mut lines: Vec<&str> = EXAMPLE.lines().collect();
        lines.reverse();
        let puzzle = Puzzle::new(lines.join("\n")).unwrap();
        assert_eq!(puzzle.part_one().unwrap(), Answer::solved("240"));
        assert_eq!(puzzle.part_two().unwrap(), Answer::solved("4455"));
    }
}
