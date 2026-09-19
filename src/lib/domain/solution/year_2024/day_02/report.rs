use std::{num::ParseIntError, str::FromStr};

/// One reactor report: the levels it read, in order.
#[derive(Debug, Clone)]
pub struct Report(Vec<i32>);

impl FromStr for Report {
    type Err = ParseIntError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<_, _>>()
            .map(Self)
    }
}

/// Safe when every step is 1 to 3 in the same direction.
fn is_safe(levels: &[i32]) -> bool {
    let mut steps = levels.windows(2).map(|w| w[1] - w[0]);
    steps.clone().all(|s| (1..=3).contains(&s)) || steps.all(|s| (-3..=-1).contains(&s))
}

impl Report {
    pub fn is_safe(&self) -> bool {
        is_safe(&self.0)
    }

    /// Safe as is, or safe once any single level is removed.
    pub fn is_safe_dampened(&self) -> bool {
        self.is_safe()
            || (0..self.0.len()).any(|skip| {
                let without: Vec<i32> = self
                    .0
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| i != skip)
                    .map(|(_, &level)| level)
                    .collect();
                is_safe(&without)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn report(s: &str) -> Report {
        s.parse().unwrap()
    }

    /// The puzzle's six example reports.
    #[test]
    fn the_examples_are_safe_or_not() {
        assert!(report("7 6 4 2 1").is_safe());
        assert!(!report("1 2 7 8 9").is_safe(), "a jump of 5");
        assert!(!report("9 7 6 2 1").is_safe(), "a drop of 4");
        assert!(!report("1 3 2 4 5").is_safe(), "changes direction");
        assert!(!report("8 6 4 4 1").is_safe(), "a repeat");
        assert!(report("1 3 6 7 9").is_safe());
    }

    #[test]
    fn dampening_forgives_one_level() {
        assert!(report("1 3 2 4 5").is_safe_dampened());
        assert!(report("8 6 4 4 1").is_safe_dampened());
        assert!(!report("1 2 7 8 9").is_safe_dampened());
    }
}
