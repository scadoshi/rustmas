//! Passport Processing: count passports with every required field, then those whose fields also hold valid values.

mod passport;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::parse;
use crate::domain::solution::year_2020::day_04::passport::Passport;

/// The batch of scanned passports.
#[derive(Debug)]
pub(crate) struct Puzzle {
    /// In input order.
    passports: Vec<Passport>,
}

impl Puzzle {
    /// How many passports pass `rule`, as the answer text.
    fn count(&self, rule: impl Fn(&Passport) -> bool) -> String {
        self.passports
            .iter()
            .filter(|p| rule(p))
            .count()
            .to_string()
    }
}

impl Solution for Puzzle {
    /// Passports are blocks separated by a blank line.
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            passports: parse::blocks(input.as_ref())?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(self.count(Passport::is_complete)))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(self.count(Passport::is_valid)))
    }
}
