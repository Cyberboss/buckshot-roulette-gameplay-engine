use std::cmp::{max, min};

use crate::{item::Item, round_player::RoundPlayer};

const MAX_ITEMS: usize = 8;

pub struct Seats {
    seats: Vec<Seat>,
}

#[derive(Debug, Clone)]
pub struct Seat {
    player: Option<RoundPlayer>,
    items: Vec<Item>,
}

#[derive(Debug)]
pub struct OccupiedSeat<'seat> {
    pub player: &'seat mut RoundPlayer,
    pub items: &'seat mut Vec<Item>,
}

impl Seat {
    pub fn new(player: RoundPlayer) -> Self {
        Seat {
            player: Some(player),
            items: Vec::with_capacity(MAX_ITEMS),
        }
    }

    pub fn player(&self) -> Option<&RoundPlayer> {
        match &self.player {
            Some(player) => Some(player),
            None => None,
        }
    }

    pub fn create_occupied_seat(&mut self) -> Option<OccupiedSeat> {
        match &mut self.player {
            Some(player) => {
                let occupied_seat = OccupiedSeat {
                    player,
                    items: &mut self.items,
                };
                Some(occupied_seat)
            }
            None => None,
        }
    }

    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }

    pub fn get_new_items(&mut self, items_to_get: usize, remaining_players: usize) {
        let allowed_items = min(items_to_get, MAX_ITEMS - self.items.len());

        todo!("get new items")
    }
}
