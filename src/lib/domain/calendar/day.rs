use crate::domain::calendar::{OutOfRange, year::Year};

/// Days published for `year`. Usually 25; 2025 was a 12-day event.
pub fn days_in_year(year: i32) -> i32 {
    match year {
        2025 => 12,
        _ => 25,
    }
}

/// A validated day within a validated [`Year`], which it always carries.
#[derive(Debug)]
pub struct Day {
    value: i32,
    year: Year,
}

impl Day {
    /// Returns the inner day value.
    pub fn value(&self) -> i32 {
        self.value
    }

    /// Returns the value of the year this day belongs to.
    pub fn year(&self) -> i32 {
        self.year.value()
    }

    /// Validates `year`, then rejects any day outside [`days_in_year`].
    pub fn new(day: i32, year: i32) -> Result<Self, OutOfRange> {
        let year = Year::new(year)?;
        if !(1..=days_in_year(year.value())).contains(&day) {
            return Err(OutOfRange);
        }
        Ok(Self { value: day, year })
    }
}
