use crate::{
    item::Item, player_number::PlayerNumber, round_player::RoundPlayer, seat::OccupiedSeat,
    shell::Shell,
};

pub struct Turn<'turn> {
    occupied_seat: OccupiedSeat<'turn>,
    shells: &'turn mut Vec<Shell>,
}

pub struct TakenTurn {
    pub target: ShotgunTarget,
    pub sawn: bool,
}

pub enum ShotgunTarget {
    RackedEmpty,
    Shot(PlayerNumber),
}

impl<'turn> Turn<'turn> {
    pub fn new(occupied_seat: OccupiedSeat<'turn>, shells: &'turn mut Vec<Shell>) -> Turn<'turn> {
        Turn {
            occupied_seat,
            shells,
        }
    }

    pub fn items(&self) -> &Vec<Item> {
        &self.occupied_seat.items
    }

    pub fn player(&self) -> &RoundPlayer {
        &self.occupied_seat.player
    }
}
