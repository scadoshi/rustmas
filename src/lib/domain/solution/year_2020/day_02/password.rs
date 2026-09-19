use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct Policy {
    pub lo: usize,
    pub hi: usize,
    pub letter: char,
}

#[derive(Debug, Error)]
pub enum InvalidPolicy {
    #[error("expected a range and a letter separated by whitespace")]
    TooFewParts,
    #[error("expected a range like `1-3`")]
    MalformedRange,
    #[error("expected a single letter")]
    MalformedLetter,
}

impl TryFrom<&str> for Policy {
    type Error = InvalidPolicy;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split_whitespace();
        let (Some(rng), Some(ltr)) = (parts.next(), parts.next()) else {
            return Err(InvalidPolicy::TooFewParts);
        };
        let Some((a, b)) = rng.split_once('-') else {
            return Err(InvalidPolicy::MalformedRange);
        };
        let (Ok(a), Ok(b)) = (a.parse::<usize>(), b.parse::<usize>()) else {
            return Err(InvalidPolicy::MalformedRange);
        };
        let lo = a.min(b);
        let hi = a.max(b);
        if ltr.len() > 1 {
            return Err(InvalidPolicy::MalformedLetter);
        }
        let Some(letter) = ltr.chars().nth(0) else {
            return Err(InvalidPolicy::MalformedLetter);
        };
        Ok(Self { lo, hi, letter })
    }
}

#[derive(Debug, Error)]
pub enum InvalidPassword {
    #[error("expected a colon between the policy and the password")]
    MissingColon,
    #[error(transparent)]
    Policy(#[from] InvalidPolicy),
}

#[derive(Debug, Clone)]
pub struct Password {
    policy: Policy,
    value: String,
}

impl TryFrom<&str> for Password {
    type Error = InvalidPassword;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (policy, password) = value.split_once(':').ok_or(InvalidPassword::MissingColon)?;
        let value = password.trim().to_owned();
        let policy = Policy::try_from(policy)?;
        Ok(Self { policy, value })
    }
}

impl Password {
    pub fn valid_count(&self) -> bool {
        let count = self
            .value
            .chars()
            .filter(|c| *c == self.policy.letter)
            .count();
        count >= self.policy.lo && count <= self.policy.hi
    }

    pub fn valid_position(&self) -> bool {
        let lo = self.value.chars().nth(self.policy.lo - 1);
        let hi = self.value.chars().nth(self.policy.hi - 1);
        (lo.is_some_and(|lo| lo == self.policy.letter)
            || hi.is_some_and(|hi| hi == self.policy.letter))
            && lo != hi
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_try_from_str_ok() {
        let policy = Policy::try_from("1-3 a").unwrap();
        assert_eq!(policy.lo, 1);
        assert_eq!(policy.hi, 3);
        assert_eq!(policy.letter, 'a');
    }

    #[test]
    fn policy_try_from_str_err() {
        assert!(matches!(
            Policy::try_from("1-3"),
            Err(InvalidPolicy::TooFewParts)
        ));
        assert!(matches!(
            Policy::try_from("13 a"),
            Err(InvalidPolicy::MalformedRange)
        ));
        assert!(matches!(
            Policy::try_from("1-3 abc"),
            Err(InvalidPolicy::MalformedLetter)
        ));
    }

    #[test]
    fn password_try_from_str_ok() {
        let password = Password::try_from("1-3 a: abcd").unwrap();
        assert_eq!(password.value, "abcd");
    }

    #[test]
    fn password_try_from_str_err() {
        assert!(matches!(
            Password::try_from("1-3 a abcd"),
            Err(InvalidPassword::MissingColon)
        ));
    }

    #[test]
    fn password_valid_count() {
        assert!(Password::try_from("1-3 a: abcd").unwrap().valid_count());
        assert!(!Password::try_from("1-3 a: aaaabcd").unwrap().valid_count());
    }

    #[test]
    fn password_valid_position() {
        assert!(Password::try_from("1-3 a: abcd").unwrap().valid_position());
        assert!(!Password::try_from("1-3 a: abad").unwrap().valid_position());
    }
}
