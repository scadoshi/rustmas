//! Day 5: Print Queue. The ordering rules are kept as a set of page pairs. Part one keeps the updates no rule contradicts; part two sorts the others by how many of their own pages must come before each page.

mod rules;
mod update;

use anyhow::anyhow;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::parse;
use crate::domain::solution::year_2024::day_05::rules::{Rule, Rules};
use crate::domain::solution::year_2024::day_05::update::Update;

/// The page ordering rules and the updates to check against them.
#[derive(Debug)]
pub(crate) struct Puzzle {
    /// Every `before|after` rule from the top section.
    rules: Rules,
    /// The updates from the bottom section, in input order.
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
        let ordered = self.updates.iter().filter(|u| self.rules.is_ordered(u));
        Ok(Answer::solved(
            sum_of_middles(ordered.cloned())?.to_string(),
        ))
    }

    /// The middle pages of the rest, once put in order.
    fn part_two(&self) -> anyhow::Result<Answer> {
        let reordered = self
            .updates
            .iter()
            .filter(|u| !self.rules.is_ordered(u))
            .map(|u| self.rules.reordered(u))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Answer::solved(sum_of_middles(reordered)?.to_string()))
    }
}

/// The middle page of every update, added up.
fn sum_of_middles(updates: impl IntoIterator<Item = Update>) -> anyhow::Result<u32> {
    updates.into_iter().try_fold(0_u32, |total, update| {
        let middle = update
            .middle()
            .ok_or_else(|| anyhow!("an update has no pages"))?;
        total
            .checked_add(middle)
            .ok_or_else(|| anyhow!("the middle pages add up past {}", u32::MAX))
    })
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
            puzzle
                .rules
                .reordered(&"75,97,47,61,53".parse().unwrap())
                .unwrap(),
            "97,75,47,61,53".parse().unwrap()
        );
    }
}
