pub mod patrol;

use crate::domain::solution::{Solution, answer::Answer, year_2024::day_06::patrol::Patrol};

pub struct Puzzle {
    patrol: Patrol,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            patrol: input.as_ref().parse()?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.patrol.walk(None).visited.len().to_string(),
        ))
    }

    /// An obstacle only matters on the path she would otherwise take, so try
    /// each visited cell except where she starts.
    fn part_two(&self) -> anyhow::Result<Answer> {
        let path = self.patrol.walk(None).visited;
        Ok(Answer::solved(
            path.into_iter()
                .filter(|&cell| cell != self.patrol.start())
                .filter(|&cell| self.patrol.walk(Some(cell)).looped)
                .count()
                .to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::solution::common::cell::Cell;

    const EXAMPLE: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    /// The puzzle's example: 41 cells visited, 6 places to trap her.
    #[test]
    fn the_example() {
        let puzzle = Puzzle::new(EXAMPLE).unwrap();
        assert_eq!(puzzle.patrol.start(), Cell::new(4, 6));
        assert_eq!(puzzle.part_one().unwrap().to_string(), "41");
        assert_eq!(puzzle.part_two().unwrap().to_string(), "6");
    }

    #[test]
    fn a_map_without_a_guard_is_an_error() {
        assert!(Puzzle::new("....\n.#..").is_err());
    }
}
