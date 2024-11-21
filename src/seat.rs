use crate::{item::Item, round_player::RoundPlayer};

#[derive(Debug, Clone)]
pub struct Seat {
    player: Option<RoundPlayer>,
    items: [Option<Item>; 8],
}

#[derive(Debug)]
pub struct OccupiedSeat<'seat> {
    pub player: &'seat mut RoundPlayer,
    pub items: &'seat mut [Option<Item>; 8],
}

impl Seat {
    pub fn new(player: RoundPlayer) -> Self {
        Seat {
            player: Some(player),
            items: [None, None, None, None, None, None, None, None],
        }
    }

    pub fn player(&self) -> Option<&RoundPlayer> {
        match &self.player {
            Some(player) => Some(player),
            None => None,
        }
    }

    pub fn with_occupied<F, TRet>(&mut self, func: F) -> Option<TRet>
    where
        F: FnOnce(OccupiedSeat) -> TRet,
    {
        match &mut self.player {
            Some(player) => {
                let occupied_seat = OccupiedSeat {
                    player,
                    items: &mut self.items,
                };
                Some(func(occupied_seat))
            }
            None => None,
        }
    }

    pub fn items(&self) -> &[Option<Item>; 8] {
        &self.items
    }
}
