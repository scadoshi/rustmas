use std::num::ParseIntError;
use std::str::FromStr;

/// The pages of one update, in the order given.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Update(Vec<u32>);

impl FromStr for Update {
    type Err = ParseIntError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value
            .split(',')
            .map(|p| p.trim().parse())
            .collect::<Result<_, _>>()
            .map(Self)
    }
}

impl Update {
    pub(super) fn new(pages: Vec<u32>) -> Self {
        Self(pages)
    }

    pub(super) fn pages(&self) -> &[u32] {
        &self.0
    }

    /// The page halfway along, or `None` for an update with no pages.
    pub(super) fn middle(&self) -> Option<u32> {
        self.0.get(self.0.len() / 2).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn middle_of_nothing_is_none() {
        assert_eq!(Update::new(Vec::new()).middle(), None);
    }
}
