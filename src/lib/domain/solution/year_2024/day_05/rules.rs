use std::cmp::Ordering;
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

use crate::domain::solution::year_2024::day_05::update::Update;

#[derive(Debug, Error)]
pub enum InvalidRule {
    #[error("expected two pages separated by `|`, like `47|53`")]
    MissingBar,
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

/// Why an update has no single order the rules agree on.
#[derive(Debug, Error)]
pub enum UnorderableUpdate {
    /// Two different pages in the update have no rule between them.
    #[error("no rule orders pages {0} and {1}")]
    Unrelated(u32, u32),
    /// The rules for the update's pages contradict each other.
    #[error("the rules for pages {0:?} go round in a circle")]
    Cycle(Vec<u32>),
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
        let mut rest = update.pages();
        while let Some((&a, tail)) = rest.split_first() {
            if tail.iter().any(|&b| self.order(a, b) == Ordering::Greater) {
                return false;
            }
            rest = tail;
        }
        true
    }

    /// `update` in the one order every rule agrees with.
    ///
    /// Each page goes by how many of the update's pages must come before it,
    /// which is the rules' order when they rank every pair without a circle.
    ///
    /// # Errors
    ///
    /// [`UnorderableUpdate::Unrelated`] when no rule orders two different
    /// pages, and [`UnorderableUpdate::Cycle`] when the rules contradict
    /// each other.
    pub fn reordered(&self, update: &Update) -> Result<Update, UnorderableUpdate> {
        let pages = update.pages();
        let mut rest = pages;
        while let Some((&a, tail)) = rest.split_first() {
            if let Some(&b) = tail
                .iter()
                .find(|&&b| a != b && self.order(a, b) == Ordering::Equal)
            {
                return Err(UnorderableUpdate::Unrelated(a, b));
            }
            rest = tail;
        }
        let mut reordered = pages.to_vec();
        reordered.sort_by_cached_key(|&page| {
            pages
                .iter()
                .filter(|&&other| self.order(other, page) == Ordering::Less)
                .count()
        });
        let reordered = Update::new(reordered);
        if self.is_ordered(&reordered) {
            Ok(reordered)
        } else {
            Err(UnorderableUpdate::Cycle(pages.to_vec()))
        }
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

    /// `1|2` and `2|3` leave 1 and 3 unrelated, so comparing them says `Equal`
    /// and the sort can leave the update out of order.
    #[test]
    fn reordered_rejects_unrelated_pages() {
        let rules: Rules = ["1|2", "2|3"].iter().map(|r| r.parse().unwrap()).collect();
        let update: Update = "3,1,2".parse().unwrap();
        let reordered = rules.reordered(&update);
        assert!(reordered.is_err(), "got {reordered:?}");
    }

    /// Every pair has a rule but they go round in a circle, so no order exists.
    #[test]
    fn reordered_rejects_a_cycle() {
        let rules: Rules = ["1|2", "2|3", "3|1"]
            .iter()
            .map(|r| r.parse().unwrap())
            .collect();
        let update: Update = "1,2,3".parse().unwrap();
        assert!(rules.reordered(&update).is_err());
    }

    /// Random rule sets, some pairs unrelated and some cycles: reordering
    /// returns an error rather than letting the sort panic on an inconsistent order.
    #[test]
    fn reordered_never_panics_on_random_rules() {
        let mut seed: u64 = 7;
        let mut next = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        for n in [10_u32, 33, 100] {
            for _ in 0..20 {
                let mut pairs = HashSet::new();
                for a in 0..n {
                    for b in (a + 1)..n {
                        match next() % 3 {
                            0 => pairs.insert((a, b)),
                            1 => pairs.insert((b, a)),
                            _ => false,
                        };
                    }
                }
                let rules = Rules(pairs);
                let update = Update::new((0..n).collect());
                let reordered = std::panic::catch_unwind(|| rules.reordered(&update));
                assert!(reordered.is_ok_and(|r| r.is_err()));
            }
        }
    }
}
