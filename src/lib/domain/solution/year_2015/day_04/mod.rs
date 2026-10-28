pub mod hashing;

use anyhow::anyhow;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;

/// [`hashing::lowest_suffix`], with running out of numbers as an error.
fn lowest(secret: &str, zeros: usize) -> anyhow::Result<usize> {
    hashing::lowest_suffix(secret, zeros)
        .ok_or_else(|| anyhow!("no suffix up to {} gives {zeros} zeros", usize::MAX))
}

#[derive(Debug)]
pub struct Puzzle {
    /// The secret key, checked when parsed to be one non-empty run of ASCII
    /// letters and digits.
    secret: String,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        let secret = input.as_ref().trim();
        if secret.is_empty() {
            return Err(anyhow!("expected a secret key, read nothing"));
        }
        if let Some(other) = secret.chars().find(|c| !c.is_ascii_alphanumeric()) {
            return Err(anyhow!(
                "expected the secret key to be letters and digits, read {other:?}"
            ));
        }
        Ok(Self {
            secret: secret.to_owned(),
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(lowest(&self.secret, 5)?.to_string()))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(lowest(&self.secret, 6)?.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_secret_key_is_one_run_of_letters_and_digits() {
        assert!(Puzzle::new("abcdef\n").is_ok());
        assert!(Puzzle::new("").is_err());
        assert!(Puzzle::new("abc def").is_err());
        assert!(Puzzle::new("abc-def").is_err());
    }
}
