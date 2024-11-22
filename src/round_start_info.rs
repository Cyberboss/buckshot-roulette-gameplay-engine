use std::ops::Range;

use rand::Rng;

use crate::player_number::PlayerNumber;

#[derive(Debug, Clone)]
pub struct RoundStartInfo {
    max_health: i32,
    starting_player: PlayerNumber,
}

impl RoundStartInfo {
    pub fn new<TRng>(starting_player: PlayerNumber, rng: &mut TRng) -> Self
    where
        TRng: Rng,
    {
        // TODO: Verify starting health range
        let max_health = rng.gen_range(Range { start: 2, end: 6 });

        RoundStartInfo {
            max_health,
            starting_player,
        }
    }

    pub fn max_health(&self) -> i32 {
        self.max_health
    }

    pub fn starting_player(&self) -> PlayerNumber {
        self.starting_player
    }
}
