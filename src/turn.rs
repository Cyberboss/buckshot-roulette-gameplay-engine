use crate::{item::Item, round_player::RoundPlayer};

pub struct Turn<'round> {
    player: &'round RoundPlayer,
}

pub struct TakenTurn {}

impl<'round> Turn<'round> {
    pub fn new(player: &'round RoundPlayer) -> Self {
        Turn { player }
    }

    pub fn available_items(&self) -> &[Option<Item>; 8] {
        self.player.items()
    }
}
