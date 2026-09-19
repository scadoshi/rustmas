use crate::domain::solution::year_2024::day_05::update::Update;
use std::{cmp::Ordering, collections::HashSet, num::ParseIntError, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidRule {
    #[error("expected two pages separated by `|`, like `47|53`")]
    MissingBar,
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

/// `before` must be printed somewhere ahead of `after`, if both appear.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rule {
    before: u32,
    after: u32,
}

impl FromStr for Rule {
    type Err = InvalidRule;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (before, after) = value
            .trim()
            .split_once('|')
            .ok_or(InvalidRule::MissingBar)?;
        Ok(Self {
            before: before.parse()?,
            after: after.parse()?,
        })
    }
}

/// Every rule as a pair, so "is `a` before `b`" is one lookup.
#[derive(Debug, Clone)]
pub struct Rules(HashSet<(u32, u32)>);

impl FromIterator<Rule> for Rules {
    fn from_iter<I: IntoIterator<Item = Rule>>(rules: I) -> Self {
        Self(rules.into_iter().map(|r| (r.before, r.after)).collect())
    }
}

impl Rules {
    /// How two pages must be ordered, or `Equal` when no rule mentions both.
    fn order(&self, a: u32, b: u32) -> Ordering {
        if self.0.contains(&(a, b)) {
            Ordering::Less
        } else if self.0.contains(&(b, a)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    /// Whether no pair of pages in `update` is the wrong way round.
    pub fn is_ordered(&self, update: &Update) -> bool {
        update.pages().iter().enumerate().all(|(i, &a)| {
            update.pages()[i + 1..]
                .iter()
                .all(|&b| self.order(a, b) != Ordering::Greater)
        })
    }

    pub fn reordered(&self, update: &Update) -> Update {
        let mut pages = update.pages().to_vec();
        pages.sort_by(|&a, &b| self.order(a, b));
        Update::new(pages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_from_str() {
        assert_eq!(
            "47|53".parse::<Rule>().unwrap(),
            Rule {
                before: 47,
                after: 53
            }
        );
        assert!(matches!(
            "47,53".parse::<Rule>(),
            Err(InvalidRule::MissingBar)
        ));
    }
}
