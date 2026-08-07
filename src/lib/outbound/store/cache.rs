use sha2::{Digest, Sha256};

/// A day's cached input, and the session that fetched it.
#[derive(Debug)]
pub struct Input {
    hash: String,
    data: String,
}

impl Input {
    pub fn new(cookie: impl AsRef<str>, data: impl Into<String>) -> Self {
        Self {
            hash: hash_cookie(cookie.as_ref()),
            data: data.into(),
        }
    }

    /// Rebuilt from disk, where the hash was already generated.
    pub fn from_parts(hash: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            hash: hash.into(),
            data: data.into(),
        }
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }

    pub fn data(&self) -> &str {
        &self.data
    }

    /// Whether this input was fetched with `cookie`.
    pub fn is_from(&self, cookie: impl AsRef<str>) -> bool {
        self.hash == hash_cookie(cookie.as_ref())
    }
}

/// A day's puzzle text. Part two stays `None` until part one is solved.
#[derive(Debug)]
pub struct Instructions {
    pub part_one: String,
    pub part_two: Option<String>,
}

/// Everything cached for one day.
#[derive(Debug)]
pub struct Entry {
    pub input: Input,
    pub instructions: Instructions,
}

fn hash_cookie(cookie: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(cookie.as_bytes());
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognises_its_own_session() {
        let input = Input::new("cookie-a", "()()");
        assert!(input.is_from("cookie-a"));
        assert!(!input.is_from("cookie-b"));
    }

    /// The cookie itself never reaches disk, only a digest of it.
    #[test]
    fn stores_a_digest_rather_than_the_cookie() {
        let input = Input::new("secret", "()()");
        assert!(!input.hash().contains("secret"));
        assert_eq!(input.hash().len(), 64);
    }

    /// Rebuilt from disk, where the hash was generated on the way in.
    #[test]
    fn from_parts_keeps_the_stored_hash() {
        let written = Input::new("cookie", "()()");
        let read = Input::from_parts(written.hash(), written.data());
        assert!(read.is_from("cookie"));
    }
}
