use crate::solutions::Solution;

pub struct Day01 {
    input: &'static str,
}

impl Solution for Day01 {
    fn new(input: &'static str) -> anyhow::Result<Self> {
        Ok(Self { input })
    }

    fn part_one(&self) -> Option<String> {
        Some(
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

    fn part_two(&self) -> Option<String> {
        let mut acc = 0;
        for (i, c) in self.input.chars().enumerate() {
            if c == '(' {
                acc += 1;
            } else if c == ')' {
                acc -= 1;
            }
            if acc == -1 {
                return Some((i + 1).to_string());
            }
        }
        None
    }
}
