use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("expected fields as `key:value` separated by whitespace")]
pub struct InvalidPassport;

/// Every field a passport must carry. `cid` is optional, which is the loophole.
const REQUIRED: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

const EYE_COLOURS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

/// One passport's fields, exactly as written.
#[derive(Debug)]
pub struct Passport(HashMap<String, String>);

impl TryFrom<&str> for Passport {
    type Error = InvalidPassport;

    /// Parses a block of `key:value` pairs, across any number of lines.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value
            .split_whitespace()
            .map(|field| {
                field
                    .split_once(':')
                    .map(|(key, value)| (key.to_owned(), value.to_owned()))
                    .ok_or(InvalidPassport)
            })
            .collect::<Result<_, _>>()
            .map(Self)
    }
}

impl Passport {
    /// Whether every required field is present, whatever it holds.
    pub fn is_complete(&self) -> bool {
        REQUIRED.iter().all(|key| self.0.contains_key(*key))
    }

    /// Whether every required field is present and holds a sensible value.
    pub fn is_valid(&self) -> bool {
        REQUIRED.iter().all(|key| {
            self.0
                .get(*key)
                .is_some_and(|value| field_is_valid(key, value))
        })
    }
}

/// A four digit year inside `range`.
fn year_in(value: &str, range: std::ops::RangeInclusive<u32>) -> bool {
    value.len() == 4 && value.parse().is_ok_and(|year| range.contains(&year))
}

/// The rules for each field, from the puzzle text.
fn field_is_valid(key: &str, value: &str) -> bool {
    match key {
        "byr" => year_in(value, 1920..=2002),
        "iyr" => year_in(value, 2010..=2020),
        "eyr" => year_in(value, 2020..=2030),
        "hgt" => match value.split_at_checked(value.len().saturating_sub(2)) {
            Some((number, "cm")) => number.parse().is_ok_and(|n| (150..=193).contains(&n)),
            Some((number, "in")) => number.parse().is_ok_and(|n| (59..=76).contains(&n)),
            _ => false,
        },
        "hcl" => value
            .strip_prefix('#')
            .is_some_and(|hex| hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit())),
        "ecl" => EYE_COLOURS.contains(&value),
        "pid" => value.len() == 9 && value.chars().all(|c| c.is_ascii_digit()),
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passport_try_from_str_reads_fields_across_lines() {
        let passport = Passport::try_from("ecl:gry pid:860033327\nbyr:1937 iyr:2017").unwrap();
        assert_eq!(passport.0["pid"], "860033327");
        assert_eq!(passport.0["byr"], "1937");
    }

    #[test]
    fn passport_try_from_str_err() {
        assert!(Passport::try_from("ecl:gry pid").is_err());
    }

    #[test]
    fn is_complete_forgives_only_a_missing_cid() {
        let all = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm";
        let no_cid = "hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm";
        let no_hgt = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929";
        assert!(Passport::try_from(all).unwrap().is_complete());
        assert!(Passport::try_from(no_cid).unwrap().is_complete());
        assert!(!Passport::try_from(no_hgt).unwrap().is_complete());
    }

    /// The field examples the puzzle gives, valid and invalid side by side.
    #[test]
    fn field_rules_follow_the_puzzle_examples() {
        let valid = [
            ("byr", "2002"),
            ("hgt", "60in"),
            ("hgt", "190cm"),
            ("hcl", "#123abc"),
            ("ecl", "brn"),
            ("pid", "000000001"),
        ];
        let invalid = [
            ("byr", "2003"),
            ("hgt", "190in"),
            ("hgt", "190"),
            ("hcl", "#123abz"),
            ("hcl", "123abc"),
            ("ecl", "wat"),
            ("pid", "0123456789"),
        ];
        for (key, value) in valid {
            assert!(field_is_valid(key, value), "{key}:{value}");
        }
        for (key, value) in invalid {
            assert!(!field_is_valid(key, value), "{key}:{value}");
        }
    }

    #[test]
    fn is_valid_needs_every_field_to_pass() {
        let good = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f";
        let bad = "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926";
        assert!(Passport::try_from(good).unwrap().is_valid());
        assert!(!Passport::try_from(bad).unwrap().is_valid());
    }
}
