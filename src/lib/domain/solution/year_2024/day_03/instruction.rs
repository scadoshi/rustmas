/// The instructions that survive the corruption.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Mul(u64, u64),
    Do,
    Dont,
}

/// Up to three digits at the front of `text`, and what follows them.
///
/// No digits parses as an error, so an empty run needs no special case.
fn number(text: &str) -> Option<(u64, &str)> {
    let (digits, rest) = text.split_at(text.bytes().take(3).take_while(u8::is_ascii_digit).count());
    digits.parse().ok().map(|n| (n, rest))
}

/// A `mul(a,b)` exactly at the front of `text`, and what follows it.
fn mul(text: &str) -> Option<(Instruction, &str)> {
    let rest = text.strip_prefix("mul(")?;
    let (a, rest) = number(rest)?;
    let rest = rest.strip_prefix(',')?;
    let (b, rest) = number(rest)?;
    let rest = rest.strip_prefix(')')?;
    Some((Instruction::Mul(a, b), rest))
}

/// Every instruction in `memory`, in order, skipping the garbage between.
pub fn instructions(memory: &str) -> impl Iterator<Item = Instruction> + '_ {
    let mut rest = memory;
    std::iter::from_fn(move || {
        while !rest.is_empty() {
            if let Some((found, after)) = mul(rest) {
                rest = after;
                return Some(found);
            }
            if let Some(after) = rest.strip_prefix("do()") {
                rest = after;
                return Some(Instruction::Do);
            }
            if let Some(after) = rest.strip_prefix("don't()") {
                rest = after;
                return Some(Instruction::Dont);
            }
            rest = &rest[rest.chars().next().map_or(0, char::len_utf8)..];
        }
        None
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul_needs_the_exact_shape() {
        assert_eq!(mul("mul(2,4)x"), Some((Instruction::Mul(2, 4), "x")));
        assert_eq!(mul("mul(123,456)"), Some((Instruction::Mul(123, 456), "")));
        assert!(mul("mul(1234,5)").is_none(), "four digits");
        assert!(mul("mul(4*").is_none());
        assert!(mul("mul(6,9!").is_none());
        assert!(mul("mul ( 2 , 4 )").is_none());
    }
}
