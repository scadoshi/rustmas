use crate::domain::solution::{Solution, answer::Answer};

pub struct Puzzle {
    input: String,
}

impl Solution for Puzzle {
    fn new(input: impl Into<String>) -> anyhow::Result<Self> {
        Ok(Self {
            input: input.into(),
        })
    }

    fn input(&self) -> &str {
        &self.input
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.input
                .chars()
                .fold(0, |acc, c| {
                    if c == '(' {
                        acc + 1
                    } else if c == ')' {
                        acc - 1
                    } else {
                        acc
                    }
                })
                .to_string(),
        ))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        let mut acc = 0;
        for (i, c) in self.input.chars().enumerate() {
            if c == '(' {
                acc += 1;
            } else if c == ')' {
                acc -= 1;
            }
            if acc == -1 {
                return Ok(Answer::solved((i + 1).to_string()));
            }
        }
        Ok(Answer::None)
    }
}
