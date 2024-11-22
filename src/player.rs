use std::collections::HashSet;

use crate::{player_number::PlayerNumber, round_number::RoundNumber};

#[derive(Debug, Clone)]
pub struct Player {
    player_number: PlayerNumber,
    wins: HashSet<RoundNumber>,
}

impl Player {
    pub fn new(player_number: PlayerNumber) -> Self {
        Player {
            player_number,
            wins: HashSet::with_capacity(3),
        }
    }

    pub fn number(&self) -> PlayerNumber {
        self.player_number
    }

    pub fn register_win(&mut self, round_number: RoundNumber) {
        assert!(self.wins.insert(round_number));
    }

    pub fn wins(&self) -> &HashSet<RoundNumber> {
        &self.wins
    }
}
