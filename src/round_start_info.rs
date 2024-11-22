use std::ops::Range;

use rand::Rng;

use crate::{multiplayer_count::MultiplayerCount, player_number::PlayerNumber};

#[derive(Debug, Clone)]
pub struct RoundStartInfo {
    max_health: i32,
    starting_player: PlayerNumber,
}

impl RoundStartInfo {
    pub fn new<TRng>(
        starting_player: PlayerNumber,
        player_count: MultiplayerCount,
        rng: &mut TRng,
    ) -> Self
    where
        TRng: Rng,
    {
        // https://github.com/thecatontheceiling/buckshotroulette_multiplayer/blob/aed4aecb7fd7f6cec14a7bd17239e736039915c0/global%20scripts/MP_RoundManager.gd#L427
        let health_range = match player_count {
            MultiplayerCount::Two => Range { start: 3, end: 5 },
            MultiplayerCount::Three => Range { start: 4, end: 6 },
            MultiplayerCount::Four => Range { start: 3, end: 6 },
        };

        let max_health = rng.gen_range(health_range);

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
