use rand::Rng;

use crate::{multiplayer_count::MultiplayerCount, player_number::PlayerNumber, LOG_RNG};

#[derive(Debug, Clone)]
pub struct RoundStartInfo {
    max_health: i32,
    starting_player: PlayerNumber,
    pub player_count: MultiplayerCount,
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
        let max_health = match player_count {
            MultiplayerCount::Two => rng.gen_range(3, 5),
            MultiplayerCount::Three => rng.gen_range(4, 6),
            MultiplayerCount::Four => rng.gen_range(3, 6),
        };

        if LOG_RNG {
            println!("{} health this round", max_health);
        }

        RoundStartInfo {
            max_health,
            starting_player,
            player_count,
        }
    }

    pub fn max_health(&self) -> i32 {
        self.max_health
    }

    pub fn starting_player(&self) -> PlayerNumber {
        self.starting_player
    }
}
