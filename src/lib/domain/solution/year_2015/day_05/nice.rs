/// Pairs that disqualify a line under the first rules.
const NAUGHTY: [&[u8]; 4] = [b"ab", b"cd", b"pq", b"xy"];

/// Three vowels, a doubled letter, and none of [`NAUGHTY`].
pub(super) fn is_nice(line: &[u8]) -> bool {
    let vowels = line.iter().filter(|b| b"aeiou".contains(b)).count();
    let doubled = line.windows(2).any(|w| matches!(w, [a, b] if a == b));
    let clean = !line.windows(2).any(|w| NAUGHTY.contains(&w));
    vowels >= 3 && doubled && clean
}

/// A pair repeated later in the line, and a letter repeated one apart.
///
/// Searching only past the pair is what keeps the two pairs from overlapping,
/// so `aaa` holds one pair rather than two.
pub(super) fn is_nicer(line: &[u8]) -> bool {
    let sandwiched = line.windows(3).any(|w| matches!(w, [a, _, c] if a == c));
    has_repeated_pair(line) && sandwiched
}

/// Whether some pair of letters appears again later without overlapping.
fn has_repeated_pair(line: &[u8]) -> bool {
    let mut rest = line;
    while let [first, tail @ ..] = rest {
        if let [second, later @ ..] = tail
            && later.windows(2).any(|other| other == [*first, *second])
        {
            return true;
        }
        rest = tail;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The puzzle's examples for the first rules.
    #[test]
    fn is_nice_examples() {
        assert!(is_nice(b"ugknbfddgicrmopn"));
        assert!(is_nice(b"aaa"));
        assert!(!is_nice(b"jchzalrnumimnmhp"));
        assert!(!is_nice(b"haegwjzuvuyypxyu"));
        assert!(!is_nice(b"dvszwmarrgswjxmb"));
    }

    /// The puzzle's examples for the second rules, plus an overlapping pair.
    #[test]
    fn is_nicer_examples() {
        assert!(is_nicer(b"qjhvhtzxzqqjkmpb"));
        assert!(is_nicer(b"xxyxx"));
        assert!(!is_nicer(b"uurcxstgmygtbstg"));
        assert!(!is_nicer(b"ieodomkazucvgmuy"));
        assert!(!is_nicer(b"aaa"));
        assert!(is_nicer(b"aaaa"));
    }
}
