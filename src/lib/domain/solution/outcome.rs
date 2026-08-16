use crate::domain::solution::{
    answer::Answer, aoc_verdict::AocVerdict, solver_verdict::SolverVerdict,
};
use std::{fmt::Display, time::Duration};

/// One part's answer and everything learned about it afterwards.
///
/// A failure is held rather than propagated, so one broken part does not hide
/// the other's answer. Only [`Answer::Value`] can carry a verdict, which the
/// attaching methods enforce and which rules errors out for free.
#[derive(Debug)]
pub struct Outcome {
    answer: anyhow::Result<Answer>,
    /// Never includes a network round trip.
    elapsed: Duration,
    /// From the third-party solver. Repeatable, so it gates submission.
    verdict: Option<SolverVerdict>,
    /// From adventofcode.com. Says whether the star exists.
    submission: Option<AocVerdict>,
}

impl Outcome {
    pub fn new(answer: anyhow::Result<Answer>, elapsed: Duration) -> Self {
        Self {
            answer,
            elapsed,
            verdict: None,
            submission: None,
        }
    }

    /// Attaches a solver verdict, ignored unless there is something to check.
    pub fn with_verdict(mut self, verdict: SolverVerdict) -> Self {
        if self.value().is_some() {
            self.verdict = Some(verdict);
        }
        self
    }

    /// Attaches what AOC said, ignored unless there is something to submit.
    pub fn with_submission(mut self, submission: AocVerdict) -> Self {
        if self.value().is_some() {
            self.submission = Some(submission);
        }
        self
    }

    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    /// The submittable text, if there is any. A failed part has none.
    pub fn value(&self) -> Option<&str> {
        self.answer.as_ref().ok()?.value()
    }

    pub fn verdict(&self) -> Option<&SolverVerdict> {
        self.verdict.as_ref()
    }

    pub fn submission(&self) -> Option<&AocVerdict> {
        self.submission.as_ref()
    }
}

impl Display for Outcome {
    /// Answer, then what is known about it, then timing. One line, unless the
    /// answer is art, which brings its own.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut line = match &self.answer {
            Ok(answer) => answer.to_string(),
            // `{:#}` puts the whole chain on the line rather than just the
            // outermost message.
            Err(e) => format!("error: {e:#}"),
        };

        // AOC's word supersedes the solver's, so a starred part reads as starred
        // rather than repeating that the solver agreed.
        let notes: String = match (&self.verdict, &self.submission) {
            (_, Some(AocVerdict::Correct)) => "new star".to_string(),
            (_, Some(AocVerdict::AlreadySolved)) => "starred".to_string(),
            (Some(v), Some(s)) => format!("{}, {}", v, s),
            (Some(v), None) => v.to_string(),
            (None, Some(s)) => s.to_string(),
            (None, None) => String::new(),
        };
        if !notes.is_empty() {
            line.push_str(&format!(" ({notes})"));
        }

        // Art ends its own line, so the timing needs no space in front of it.
        if !line.ends_with('\n') {
            line.push(' ');
        }

        write!(f, "{line}[{:?}]", self.elapsed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Zero elapsed, so a test can assert the whole line without the timing
    /// varying under it.
    fn value() -> Outcome {
        Outcome::new(Ok(Answer::solved("foo")), Duration::ZERO)
    }

    #[test]
    fn bare_answer_has_no_notes() {
        assert_eq!(value().to_string(), "foo [0ns]");
    }

    #[test]
    fn solver_verdict_alone_shows() {
        assert_eq!(
            value().with_verdict(SolverVerdict::High).to_string(),
            "foo (high) [0ns]"
        );
    }

    #[test]
    fn aoc_verdict_alone_shows() {
        assert_eq!(
            value().with_submission(AocVerdict::Low).to_string(),
            "foo (low) [0ns]"
        );
    }

    #[test]
    fn aoc_supersedes_the_solver() {
        let starred = value()
            .with_verdict(SolverVerdict::Correct)
            .with_submission(AocVerdict::AlreadySolved);
        assert_eq!(starred.to_string(), "foo (starred) [0ns]");

        let fresh = value()
            .with_verdict(SolverVerdict::Correct)
            .with_submission(AocVerdict::Correct);
        assert_eq!(fresh.to_string(), "foo (new star) [0ns]");
    }

    /// Any other AOC reply is worth seeing next to what the solver thought.
    #[test]
    fn both_verdicts_show_when_aoc_did_not_grade() {
        let cooled = value()
            .with_verdict(SolverVerdict::Correct)
            .with_submission(AocVerdict::Cooldown("1m 0s".to_string()));
        assert_eq!(
            cooled.to_string(),
            "foo (correct, rate limited, 1m 0s left to wait) [0ns]"
        );
    }

    /// The one test that uses a real duration, since zero cannot show a unit.
    #[test]
    fn timing_renders_in_its_own_unit() {
        let outcome = Outcome::new(Ok(Answer::solved("foo")), Duration::from_micros(7));
        assert_eq!(outcome.to_string(), "foo [7µs]");
    }

    /// The invariant that survived splitting `Answer` from `Outcome`.
    #[test]
    fn unsubmittable_answers_never_take_a_verdict() {
        for answer in [Answer::Visual("art".to_string()), Answer::None] {
            let outcome = Outcome::new(Ok(answer), Duration::ZERO)
                .with_verdict(SolverVerdict::Correct)
                .with_submission(AocVerdict::Correct);
            assert!(outcome.verdict().is_none());
            assert!(outcome.submission().is_none());
        }
    }

    #[test]
    fn visual_answers_render_their_art() {
        let outcome = Outcome::new(Ok(Answer::Visual("###".to_string())), Duration::ZERO);
        // Art is the one answer with no space before the timing, since it ends
        // its own line.
        assert_eq!(outcome.to_string(), "\n###\n[0ns]");
    }

    #[test]
    fn absent_answers_say_so() {
        let outcome = Outcome::new(Ok(Answer::None), Duration::ZERO);
        assert_eq!(outcome.to_string(), "(none) [0ns]");
    }

    /// A stub must not read as a part that is finished and has nothing to say.
    #[test]
    fn unwritten_parts_are_not_absent_ones() {
        let outcome = Outcome::new(Ok(Answer::Unwritten), Duration::ZERO);
        assert_eq!(outcome.to_string(), "(unwritten) [0ns]");
        assert!(outcome.value().is_none());
    }

    fn failed() -> Outcome {
        let cause = anyhow::anyhow!("given string was too short");
        Outcome::new(
            Err(cause.context("could not parse the input")),
            Duration::ZERO,
        )
    }

    /// The whole chain, since the outermost message rarely names the day.
    #[test]
    fn failed_parts_render_their_error() {
        assert_eq!(
            failed().to_string(),
            "error: could not parse the input: given string was too short [0ns]"
        );
    }

    /// Nothing to submit, so nothing to check.
    #[test]
    fn failed_parts_never_take_a_verdict() {
        let outcome = failed()
            .with_verdict(SolverVerdict::Correct)
            .with_submission(AocVerdict::Correct);
        assert!(outcome.value().is_none());
        assert!(outcome.verdict().is_none());
        assert!(outcome.submission().is_none());
    }
}
