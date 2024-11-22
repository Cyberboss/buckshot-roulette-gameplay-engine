use std::ops::Range;

use rand::Rng;

use crate::multiplayer_count::MultiplayerCount;

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
        let total_shells = rng.gen_range(Range { start: 2, end: 9 });

        let initial_live_rounds = rng.gen_range(Range {
            start: 1,
            end: total_shells - 1,
        });

        let initial_blank_rounds = total_shells - initial_live_rounds;

        // https://github.com/thecatontheceiling/buckshotroulette_multiplayer/blob/aed4aecb7fd7f6cec14a7bd17239e736039915c0/global%20scripts/MP_RoundManager.gd#L528
        let items_range = match multiplayer_count {
            MultiplayerCount::Two => Range { start: 2, end: 5 },
            MultiplayerCount::Three => Range { start: 3, end: 6 },
            MultiplayerCount::Four => Range { start: 3, end: 5 },
        };

        let new_items = rng.gen_range(items_range);

        Loadout {
            initial_blank_rounds,
            initial_live_rounds,
            new_items,
        }
    }
}
