pub mod schematic;

use crate::domain::solution::{
    Solution,
    answer::Answer,
    year_2023::day_03::schematic::{Schematic, is_symbol},
};

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
        let symbols = self.schematic.symbols(is_symbol);
        Ok(Answer::solved(
            self.schematic
                .numbers()
                .iter()
                .filter(|number| symbols.iter().any(|&cell| number.touches(cell)))
                .map(|number| number.value)
                .sum::<u32>()
                .to_string(),
        ))
    }

    /// A `*` touching exactly two numbers is a gear; its ratio is their product.
    fn part_two(&self) -> anyhow::Result<Answer> {
        let numbers = self.schematic.numbers();
        Ok(Answer::solved(
            self.schematic
                .symbols(|c| c == '*')
                .into_iter()
                .filter_map(|star| {
                    let touching: Vec<u64> = numbers
                        .iter()
                        .filter(|number| number.touches(star))
                        .map(|number| u64::from(number.value))
                        .collect();
                    match touching.as_slice() {
                        [a, b] => Some(a * b),
                        _ => None,
                    }
                })
                .sum::<u64>()
                .to_string(),
        ))
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
