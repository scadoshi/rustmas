use crate::session::verdict::Verdict;

#[derive(Debug)]
pub struct Answer {
    value: String,
    verdict: Option<Verdict>,
}

impl Answer {
    pub fn new(value: impl Into<String>, verdict: Option<Verdict>) -> Self {
        Self {
            value: value.into(),
            verdict,
        }
    }
}
