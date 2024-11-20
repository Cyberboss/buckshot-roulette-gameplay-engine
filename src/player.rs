use crate::{player_number::PlayerNumber, round_number::RoundNumber};

#[derive(Debug, Clone)]
pub struct Player {
    player_number: PlayerNumber,
    wins: Vec<RoundNumber>,
}

impl Player {
    pub fn new(player_number: PlayerNumber) -> Self {
        Player {
            player_number,
            wins: Vec::with_capacity(3),
        }
    }

    pub fn number(&self) -> PlayerNumber {
        self.player_number
    }
}
