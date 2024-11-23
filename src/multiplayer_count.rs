use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MultiplayerCount {
    Two = 2,
    Three = 3,
    Four = 4,
}

impl Display for MultiplayerCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            MultiplayerCount::Four => "4",
            MultiplayerCount::Two => "2",
            MultiplayerCount::Three => "3",
        };
        write!(f, "{}", str)
    }
}
