use anyhow::bail;
use std::fmt::{self, Debug, Display};

/// How many hex characters adventofcode.com's session cookie carries.
///
/// Observed rather than promised. The site reads this many and ignores anything
/// after, so a longer value still authenticates while a shorter one does not,
/// which makes a length check the only way to catch a truncated paste.
const LENGTH: usize = 128;

/// A session cookie in the shape adventofcode.com accepts.
///
/// Parsed once at the edge so a mistyped value fails with its own message
/// rather than as a `400` partway through a run.
#[derive(Clone, PartialEq, Eq)]
pub struct SessionCookie(String);

impl SessionCookie {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for SessionCookie {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> anyhow::Result<Self> {
        let value = value.trim();
        if value.len() != LENGTH {
            bail!(
                "COOKIE must be {LENGTH} hex characters, got {}. Copy the whole \
                 `session` cookie value from your browser.",
                value.len()
            );
        }
        if !value.chars().all(|c| c.is_ascii_hexdigit()) {
            bail!("COOKIE must be hex characters only, and this one is not");
        }
        Ok(Self(value.to_string()))
    }
}

impl AsRef<str> for SessionCookie {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for SessionCookie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Redacted, so a cookie never reaches a log through a `{:?}` on its holder.
impl Debug for SessionCookie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SessionCookie(redacted)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid() -> String {
        "a1b2c3d4".repeat(LENGTH / 8)
    }

    #[test]
    fn accepts_a_full_length_hex_value() {
        assert!(SessionCookie::try_from(valid().as_str()).is_ok());
    }

    #[test]
    fn trims_surrounding_whitespace() {
        let padded = format!("  {}\n", valid());
        assert_eq!(
            SessionCookie::try_from(padded.as_str()).unwrap().as_str(),
            valid()
        );
    }

    #[test]
    fn rejects_the_wrong_length() {
        assert!(SessionCookie::try_from(&valid()[1..]).is_err());
        assert!(SessionCookie::try_from(format!("{}a", valid()).as_str()).is_err());
        assert!(SessionCookie::try_from("").is_err());
    }

    /// The site ignores anything past the first 128 characters, so a longer
    /// value works while reading as a different cookie to the cache.
    #[test]
    fn rejects_a_value_the_site_would_have_accepted() {
        let long = format!("{}deadbeef", valid());
        assert!(SessionCookie::try_from(long.as_str()).is_err());
    }

    #[test]
    fn rejects_non_hex() {
        let mut wrong = valid();
        wrong.replace_range(0..1, "z");
        assert!(SessionCookie::try_from(wrong.as_str()).is_err());
    }

    /// A cookie is a secret, so its own `Debug` must not print it.
    #[test]
    fn debug_redacts() {
        let cookie = SessionCookie::try_from(valid().as_str()).unwrap();
        assert!(!format!("{cookie:?}").contains(&valid()));
    }
}
