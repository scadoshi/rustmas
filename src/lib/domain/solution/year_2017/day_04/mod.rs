pub mod passphrase;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;

#[derive(Debug)]
pub struct Puzzle {
    passphrases: Vec<String>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            passphrases: input.as_ref().lines().map(str::to_owned).collect(),
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            passphrase::count_valid(&self.passphrases, |word| word).to_string(),
        ))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            passphrase::count_valid(&self.passphrases, passphrase::sorted_letters).to_string(),
        ))
    }
}
