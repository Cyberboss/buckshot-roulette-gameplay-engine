use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerNumber {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
}

impl Display for PlayerNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            PlayerNumber::One => "One",
            PlayerNumber::Two => "Two",
            PlayerNumber::Three => "Three",
            PlayerNumber::Four => "Four",
        };

        write!(f, "{}", str)
    }
}
