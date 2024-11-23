use rand::Rng;

use crate::{multiplayer_count::MultiplayerCount, LOG_RNG};

pub const MAX_SHELLS: u32 = 8;

#[derive(Debug, Clone, Copy)]
struct Sequence {
    num_live: usize,
    num_blank: usize,
}

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
        let sequence = get_sequence(multiplayer_count, rng);

        // https://github.com/thecatontheceiling/buckshotroulette_multiplayer/blob/aed4aecb7fd7f6cec14a7bd17239e736039915c0/global%20scripts/MP_RoundManager.gd#L528
        // #4 overridden in mp_main.tscn
        let new_items = match multiplayer_count {
            MultiplayerCount::Two => rng.gen_range(2, 5),
            MultiplayerCount::Three => rng.gen_range(3, 6),
            MultiplayerCount::Four => rng.gen_range(2, 5),
        };

        if LOG_RNG {
            println!("{} new items", new_items);
        }

        Loadout {
            initial_blank_rounds: sequence.num_blank,
            initial_live_rounds: sequence.num_live,
            new_items,
        }
    }
}

fn get_sequence<TRng>(multiplayer_count: MultiplayerCount, rng: &mut TRng) -> Sequence
where
    TRng: Rng,
{
    // found in mp_main.tscn
    let sequences = match multiplayer_count {
        MultiplayerCount::Two => vec![
            s(1, 1),
            s(1, 2),
            s(2, 1),
            s(2, 2),
            s(2, 3),
            s(3, 1),
            s(3, 2),
            s(3, 3),
            s(4, 2),
        ],
        MultiplayerCount::Three => vec![
            s(1, 1),
            s(2, 2),
            s(2, 3),
            s(3, 1),
            s(3, 2),
            s(3, 3),
            s(3, 4),
            s(4, 2),
            s(4, 3),
            s(4, 4),
        ],
        MultiplayerCount::Four => vec![
            s(2, 1),
            s(2, 2),
            s(3, 1),
            s(3, 2),
            s(3, 3),
            s(3, 4),
            s(3, 4),
            s(4, 2),
            s(4, 3),
            s(4, 4),
        ],
    };

    let sequence_index = rng.gen_range(0, sequences.len());
    if LOG_RNG {
        println!(
            "Selecting sequence index {}/{}",
            sequence_index,
            sequences.len()
        );
    }
    sequences[sequence_index]
}

fn s(num_live: usize, num_blank: usize) -> Sequence {
    Sequence {
        num_live,
        num_blank,
    }
}
