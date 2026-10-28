use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;

#[derive(Debug)]
pub struct Puzzle {
    digits: Vec<u32>,
}

impl Puzzle {
    fn matching(&self, step: usize) -> u32 {
        let ahead = self.digits.iter().cycle().skip(step);
        self.digits
            .iter()
            .zip(ahead)
            .filter(|(digit, other)| digit == other)
            .map(|(digit, _)| digit)
            .sum()
    }
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        let digits = input
            .as_ref()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
        Ok(Self { digits })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(self.matching(1).to_string()))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.matching(self.digits.len() / 2).to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn answers(input: &str) -> (Answer, Answer) {
        let puzzle = Puzzle::new(input).unwrap();
        (puzzle.part_one().unwrap(), puzzle.part_two().unwrap())
    }

    /// The puzzle's examples for both parts.
    #[test]
    fn examples() {
        assert_eq!(answers("1122").0, Answer::solved("3"));
        assert_eq!(answers("1111").0, Answer::solved("4"));
        assert_eq!(answers("1234").0, Answer::solved("0"));
        assert_eq!(answers("91212129").0, Answer::solved("9"));
        assert_eq!(answers("1212").1, Answer::solved("6"));
        assert_eq!(answers("1221").1, Answer::solved("0"));
        assert_eq!(answers("123425").1, Answer::solved("4"));
        assert_eq!(answers("123123").1, Answer::solved("12"));
        assert_eq!(answers("12131415").1, Answer::solved("4"));
    }
}
