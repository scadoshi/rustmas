#[derive(Debug, Clone, Copy)]
pub enum Part {
    One,
    Two,
}

impl Part {
    pub fn to_wire_value(&self) -> u32 {
        match self {
            Self::One => 1,
            Self::Two => 2,
        }
    }
}
