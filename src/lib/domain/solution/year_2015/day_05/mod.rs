mod nice;

use anyhow::anyhow;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;

#[derive(Debug)]
pub(crate) struct Puzzle {
    /// The non-blank lines, each checked when parsed to hold only `a-z`.
    lines: Vec<String>,
}

impl Puzzle {
    /// How many lines `rule` calls nice.
    fn count(&self, rule: impl Fn(&[u8]) -> bool) -> String {
        self.lines
            .iter()
            .filter(|line| rule(line.as_bytes()))
            .count()
            .to_string()
    }
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        let lines: Vec<String> = input
            .as_ref()
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::to_owned)
            .collect();
        for (number, line) in (1_usize..).zip(&lines) {
            if let Some(other) = line.chars().find(|c| !c.is_ascii_lowercase()) {
                return Err(anyhow!(
                    "expected string {number} to hold only lowercase letters, read {other:?}"
                ));
            }
        }
        Ok(Self { lines })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(self.count(nice::is_nice)))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(self.count(nice::is_nicer)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The examples from both parts, one string per line.
    #[test]
    fn examples() {
        let first = "ugknbfddgicrmopn\naaa\njchzalrnumimnmhp\nhaegwjzuvuyypxyu\ndvszwmarrgswjxmb\n";
        assert_eq!(
            Puzzle::new(first).unwrap().part_one().unwrap(),
            Answer::solved("2")
        );
        let second = "qjhvhtzxzqqjkmpb\nxxyxx\nuurcxstgmygtbstg\nieodomkazucvgmuy";
        assert_eq!(
            Puzzle::new(second).unwrap().part_two().unwrap(),
            Answer::solved("2")
        );
    }

    #[test]
    fn anything_but_lowercase_letters_is_rejected() {
        assert!(Puzzle::new("abc\naBc").is_err());
        assert!(Puzzle::new("ab cd").is_err());
    }
}
