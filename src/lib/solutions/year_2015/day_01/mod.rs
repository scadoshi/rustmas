use crate::solutions::{Answer, Solution};

pub struct Day01 {
    input: &'static str,
}

impl Solution for Day01 {
    fn new(input: &'static str) -> anyhow::Result<Self> {
        Ok(Self { input })
    }

    fn input(&self) -> &str {
        self.input
    }

    fn part_one(&self) -> Answer {
        Answer::solved(
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
        )
    }

    fn part_two(&self) -> Answer {
        let mut acc = 0;
        for (i, c) in self.input.chars().enumerate() {
            if c == '(' {
                acc += 1;
            } else if c == ')' {
                acc -= 1;
            }
            if acc == -1 {
                return Answer::solved((i + 1).to_string());
            }
        }
        Answer::None
    }
}
