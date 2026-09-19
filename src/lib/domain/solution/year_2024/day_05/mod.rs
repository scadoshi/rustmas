pub mod rules;
pub mod update;

use crate::domain::solution::{
    Solution,
    answer::Answer,
    common::parse,
    year_2024::day_05::{
        rules::{Rule, Rules},
        update::Update,
    },
};
use anyhow::anyhow;

pub struct Puzzle {
    rules: Rules,
    updates: Vec<Update>,
}

impl Solution for Puzzle {
    /// Rules come first, then a blank line, then the updates.
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        let (rules, updates) = input
            .as_ref()
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("expected the rules, a blank line, then the updates"))?;
        let rules: Vec<Rule> = parse::lines(rules)?;
        Ok(Self {
            rules: rules.into_iter().collect(),
            updates: parse::lines(updates)?,
        })
    }

    /// The middle pages of the updates already in order.
    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.updates
                .iter()
                .filter(|u| self.rules.is_ordered(u))
                .map(Update::middle)
                .sum::<u32>()
                .to_string(),
        ))
    }

    /// The middle pages of the rest, once put in order.
    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.updates
                .iter()
                .filter(|u| !self.rules.is_ordered(u))
                .map(|u| self.rules.reordered(u).middle())
                .sum::<u32>()
                .to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    /// The puzzle's example: 143 already ordered, 123 after reordering.
    #[test]
    fn the_example() {
        let puzzle = Puzzle::new(EXAMPLE).unwrap();
        assert_eq!(puzzle.part_one().unwrap().to_string(), "143");
        assert_eq!(puzzle.part_two().unwrap().to_string(), "123");
        assert_eq!(
            puzzle.rules.reordered(&"75,97,47,61,53".parse().unwrap()),
            "97,75,47,61,53".parse().unwrap()
        );
    }
}
