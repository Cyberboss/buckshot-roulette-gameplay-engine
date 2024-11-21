use std::ops::Range;

use rand::Rng;

pub struct Loadout {
    initial_blank_rounds: u8,
    initial_live_rounds: u8,
    new_items: u8,
}

impl Loadout {
    fn new<TRng>(rng: &mut TRng) -> Self
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

        // TODO: Validate item count possibilities
        let new_items = rng.gen_range(Range { start: 2, end: 5 });

        Loadout {
            initial_blank_rounds,
            initial_live_rounds,
            new_items,
        }
    }
}
