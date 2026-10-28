mod passphrase;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::parse;
use crate::domain::solution::year_2017::day_04::passphrase::Passphrase;

#[derive(Debug)]
pub(crate) struct Puzzle {
    passphrases: Vec<Passphrase>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            passphrases: parse::lines(input.as_ref())?,
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The puzzle's examples for both parts, one passphrase per line.
    #[test]
    fn examples() {
        let first = Puzzle::new("aa bb cc dd ee\naa bb cc dd aa\naa bb cc dd aaa").unwrap();
        assert_eq!(first.part_one().unwrap(), Answer::solved("2"));
        let second = Puzzle::new(
            "abcde fghij\nabcde xyz ecdab\na ab abc abd abf abj\niiii oiii ooii oooi oooo\noiii ioii iioi iiio",
        )
        .unwrap();
        assert_eq!(second.part_two().unwrap(), Answer::solved("3"));
    }

    #[test]
    fn anything_but_lowercase_words_is_rejected() {
        assert!(Puzzle::new("aa bb\naa Bb").is_err());
        assert!(Puzzle::new("aa-bb cc").is_err());
    }
}
