mod dimensions;

use anyhow::anyhow;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::parse;
use crate::domain::solution::year_2015::day_02::dimensions::Dimensions;

#[derive(Debug)]
pub(crate) struct Puzzle {
    presents: Vec<Dimensions>,
}

/// What every present needs, added up.
///
/// # Errors
///
/// When a present's need or the total passes [`u32::MAX`].
fn total(presents: &[Dimensions], need: fn(&Dimensions) -> Option<u32>) -> anyhow::Result<u32> {
    presents
        .iter()
        .try_fold(0_u32, |total, present| {
            need(present).and_then(|need| total.checked_add(need))
        })
        .ok_or_else(|| anyhow!("the total passes {}", u32::MAX))
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            presents: parse::lines(input.as_ref())?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        let paper = total(&self.presents, Dimensions::wrapping_paper_required)?;
        Ok(Answer::solved(paper.to_string()))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        let ribbon = total(&self.presents, Dimensions::ribbon_required)?;
        Ok(Answer::solved(ribbon.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_examples() {
        let puzzle = Puzzle::new("2x3x4\n1x1x10").unwrap();
        assert_eq!(puzzle.part_one().unwrap(), Answer::solved("101"));
        assert_eq!(puzzle.part_two().unwrap(), Answer::solved("48"));
    }

    /// One side alone needs ten billion square feet, past a `u32`.
    #[test]
    fn a_present_too_big_to_count_is_an_error() {
        let puzzle = Puzzle::new("100000x100000x100000").unwrap();
        assert!(puzzle.part_one().is_err());
        assert!(puzzle.part_two().is_err());
    }

    /// Each present fits, but together they pass a `u32`.
    #[test]
    fn totals_past_u32_are_an_error() {
        let paper = Puzzle::new("1x1x1000000000\n1x1x1000000000\n1x1x1000000000").unwrap();
        assert!(paper.part_one().is_err());
        let ribbon = Puzzle::new("1x1x2000000000\n1x1x2000000000\n1x1x2000000000").unwrap();
        assert!(ribbon.part_two().is_err());
    }
}
