use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    hash: String,
    data: String,
}

fn hash_cookie(cookie: impl AsRef<str>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(cookie.as_ref().as_bytes());
    hex::encode(hasher.finalize())
}

impl Input {
    pub fn new(cookie: impl AsRef<str>, data: impl Into<String>) -> Self {
        Self {
            hash: hash_cookie(cookie),
            data: data.into(),
        }
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }

    pub fn data(&self) -> &str {
        &self.data
    }

    pub fn is_from(&self, cookie: impl AsRef<str>) -> bool {
        self.hash == hash_cookie(cookie)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instructions {
    pub part_one: String,
    pub part_two: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub input: Input,
    pub instructions: Instructions,
}
