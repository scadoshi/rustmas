pub mod schematic;

use anyhow::Context;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::year_2023::day_03::schematic::Schematic;

#[derive(Debug)]
pub struct Puzzle {
    schematic: Schematic,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            schematic: input.as_ref().parse()?,
        })
    }

    /// A number is a part number if any symbol touches it.
    fn part_one(&self) -> anyhow::Result<Answer> {
        let symbols = self.schematic.symbols(schematic::is_symbol);
        let total = self
            .schematic
            .numbers()?
            .iter()
            .filter(|number| symbols.iter().any(|&cell| number.touches(cell)))
            .try_fold(0_u32, |total, number| total.checked_add(number.value))
            .context("the part numbers add up past u32")?;
        Ok(Answer::solved(total.to_string()))
    }

    /// A `*` touching exactly two numbers is a gear; its ratio is their product.
    fn part_two(&self) -> anyhow::Result<Answer> {
        let numbers = self.schematic.numbers()?;
        let mut total: u64 = 0;
        for star in self.schematic.symbols(|c| c == '*') {
            let touching: Vec<u64> = numbers
                .iter()
                .filter(|number| number.touches(star))
                .map(|number| u64::from(number.value))
                .collect();
            if let [a, b] = touching.as_slice() {
                total = a
                    .checked_mul(*b)
                    .and_then(|ratio| total.checked_add(ratio))
                    .context("the sum of the gear ratios passes u64")?;
            }
        }
        Ok(Answer::solved(total.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The puzzle's example schematic.
    const EXAMPLE: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn the_example_gives_4361_and_467835() {
        let puzzle = Puzzle::new(EXAMPLE).unwrap();
        assert_eq!(puzzle.part_one().unwrap().to_string(), "4361");
        assert_eq!(puzzle.part_two().unwrap().to_string(), "467835");
    }
}
