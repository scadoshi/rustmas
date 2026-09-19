pub mod passport;

use crate::domain::solution::{Solution, answer::Answer, year_2020::day_04::passport::Passport};

pub struct Puzzle {
    passports: Vec<Passport>,
}

impl Puzzle {
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
            passports: input
                .as_ref()
                .split("\n\n")
                .filter(|block| !block.trim().is_empty())
                .map(Passport::try_from)
                .collect::<Result<_, _>>()?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(self.count(Passport::is_complete)))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(self.count(Passport::is_valid)))
    }
}
