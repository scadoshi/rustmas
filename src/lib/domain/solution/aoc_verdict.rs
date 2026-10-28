//! What adventofcode.com said about a submitted answer.

use std::fmt::{self, Display};

/// What adventofcode.com said about a submission.
///
/// Each part grades once, so a second correct answer comes back as
/// [`AocVerdict::AlreadySolved`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AocVerdict {
    /// Right, and the star is new.
    Correct,
    /// Wrong, with no hint which way.
    Incorrect,
    /// Wrong, and too low.
    Low,
    /// Wrong, and too high.
    High,
    /// Submitted too recently. Holds the wait as AOC phrased it, like `1m 0s`,
    /// or `None` when the reply did not say.
    Cooldown(Option<String>),
    /// The part is already solved, so nothing was graded.
    AlreadySolved,
    /// The session cookie is missing or expired, so nothing was graded.
    NotLoggedIn,
    /// A reply that matches no known phrasing. Holds the start of its text,
    /// since the answer may or may not have been graded.
    Unrecognized(String),
}

impl Display for AocVerdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Correct => write!(f, "correct"),
            Self::Incorrect => write!(f, "incorrect"),
            Self::Low => write!(f, "low"),
            Self::High => write!(f, "high"),
            Self::Cooldown(Some(wait)) => write!(f, "rate limited, {wait} left to wait"),
            Self::Cooldown(None) => write!(f, "rate limited, unknown left to wait"),
            Self::AlreadySolved => write!(f, "already solved"),
            Self::NotLoggedIn => write!(f, "not logged in"),
            Self::Unrecognized(snippet) => write!(f, "unrecognized reply: {snippet}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cooldown_prints_the_wait_or_unknown() {
        assert_eq!(
            AocVerdict::Cooldown(Some("1m 0s".to_string())).to_string(),
            "rate limited, 1m 0s left to wait"
        );
        assert_eq!(
            AocVerdict::Cooldown(None).to_string(),
            "rate limited, unknown left to wait"
        );
    }

    #[test]
    fn unrecognized_says_so_and_shows_the_reply() {
        assert_eq!(
            AocVerdict::Unrecognized("Something new happened.".to_string()).to_string(),
            "unrecognized reply: Something new happened."
        );
    }
}
