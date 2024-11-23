use rand::Rng;

use crate::{multiplayer_count::MultiplayerCount, LOG_RNG};

pub struct Loadout {
    pub initial_blank_rounds: usize,
    pub initial_live_rounds: usize,
    pub new_items: usize,
}

impl Loadout {
    pub fn new<TRng>(multiplayer_count: MultiplayerCount, rng: &mut TRng) -> Self
    where
        TRng: Rng,
    {
        // TODO: Validate shell possiblities
        let total_shells = rng.gen_range(2, 9);

        if LOG_RNG {
            println!("Generating {} shells", total_shells);
        }

        let initial_live_rounds = rng.gen_range(1, total_shells);

        if LOG_RNG {
            println!("{}/{} live rounds", initial_live_rounds, total_shells);
        }

        let initial_blank_rounds = total_shells - initial_live_rounds;

        // https://github.com/thecatontheceiling/buckshotroulette_multiplayer/blob/aed4aecb7fd7f6cec14a7bd17239e736039915c0/global%20scripts/MP_RoundManager.gd#L528
        let new_items = match multiplayer_count {
            MultiplayerCount::Two => rng.gen_range(2, 5),
            MultiplayerCount::Three => rng.gen_range(3, 6),
            MultiplayerCount::Four => rng.gen_range(3, 5),
        };

        if LOG_RNG {
            println!("{} new items", new_items);
        }

        Loadout {
            initial_blank_rounds,
            initial_live_rounds,
            new_items,
        }
    }
}
