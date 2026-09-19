use std::{num::ParseIntError, str::FromStr};

/// The pages of one update, in the order given.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Update(Vec<u32>);

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
    pub fn new(pages: Vec<u32>) -> Self {
        Self(pages)
    }

    pub fn pages(&self) -> &[u32] {
        &self.0
    }

    pub fn middle(&self) -> u32 {
        self.0[self.0.len() / 2]
    }
}
