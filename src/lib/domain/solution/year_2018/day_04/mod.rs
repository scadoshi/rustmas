pub mod record;

use crate::domain::solution::{
    Solution,
    answer::Answer,
    common::parse,
    year_2018::day_04::record::{Event, Record},
};
use std::collections::HashMap;

/// Minutes in the midnight hour, the only hour any guard sleeps through.
const MINUTES: usize = 60;

pub struct Puzzle {
    records: Vec<Record>,
}

impl Puzzle {
    /// How often each guard was asleep during each minute of the hour.
    ///
    /// A sleep runs from the minute it starts until the minute a wake ends it,
    /// so the log has to be read in order for a sleep to find its guard.
    fn sleep_by_minute(&self) -> HashMap<u32, [u32; MINUTES]> {
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
                            *slept_through += 1;
                        }
                    }
                }
            }
        }
        slept
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
        let slept = self.sleep_by_minute();
        let Some((guard, minutes)) = slept
            .iter()
            .max_by_key(|(_, minutes)| minutes.iter().sum::<u32>())
        else {
            return Ok(Answer::None);
        };
        let Some((minute, _)) = minutes.iter().enumerate().max_by_key(|(_, count)| **count) else {
            return Ok(Answer::None);
        };
        Ok(Answer::solved((*guard as usize * minute).to_string()))
    }

    /// The guard most often asleep on the same minute, times that minute.
    fn part_two(&self) -> anyhow::Result<Answer> {
        let slept = self.sleep_by_minute();
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
        Ok(Answer::solved((guard as usize * minute).to_string()))
    }
}
