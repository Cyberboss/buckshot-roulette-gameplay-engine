use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoundNumber {
    One = 1,
    Two = 2,
    Three = 3,
}

impl Display for RoundNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            RoundNumber::One => "1",
            RoundNumber::Two => "2",
            RoundNumber::Three => "3",
        };
        write!(f, "{}", str)
    }
}
