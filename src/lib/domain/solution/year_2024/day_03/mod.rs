pub mod instruction;

use crate::domain::solution::{
    Solution,
    answer::Answer,
    year_2024::day_03::instruction::{Instruction, instructions},
};

pub struct Puzzle {
    memory: String,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            memory: input.as_ref().to_owned(),
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            instructions(&self.memory)
                .filter_map(|i| match i {
                    Instruction::Mul(a, b) => Some(a * b),
                    _ => None,
                })
                .sum::<u64>()
                .to_string(),
        ))
    }

    /// `don't()` switches the multiplications off until the next `do()`.
    fn part_two(&self) -> anyhow::Result<Answer> {
        let (total, _) =
            instructions(&self.memory).fold((0, true), |(total, enabled), i| match i {
                Instruction::Mul(a, b) if enabled => (total + a * b, enabled),
                Instruction::Mul(..) => (total, enabled),
                Instruction::Do => (total, true),
                Instruction::Dont => (total, false),
            });
        Ok(Answer::solved(total.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The puzzle's two examples: 161 with everything on, 48 with do/don't.
    #[test]
    fn the_examples() {
        let one =
            Puzzle::new("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
                .unwrap();
        assert_eq!(one.part_one().unwrap().to_string(), "161");
        let two = Puzzle::new(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        )
        .unwrap();
        assert_eq!(two.part_two().unwrap().to_string(), "48");
    }
}
