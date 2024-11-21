use crate::{item::Item, seat::OccupiedSeat};

pub struct Turn<'turn> {
    occupied_seat: OccupiedSeat<'turn>,
}

pub struct TakenTurn {}

impl<'turn> Turn<'turn> {
    pub fn new(occupied_seat: OccupiedSeat<'turn>) -> Turn {
        Turn { occupied_seat }
    }

    pub fn available_items(&self) -> impl Iterator<Item = &Item> {
        self.occupied_seat
            .items
            .iter()
            .filter_map(|item| match item {
                Some(item) => Some(item),
                None => None,
            })
    }
}
