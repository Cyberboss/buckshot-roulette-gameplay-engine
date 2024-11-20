use crate::{
    item::Item, player::Player, player_number::PlayerNumber, round_start_info::RoundStartInfo,
};

#[derive(Debug, Clone)]
pub struct RoundPlayer {
    player_number: PlayerNumber,
    health: u8,
    items: [Option<Item>; 8],
    stunned: bool,
}

impl RoundPlayer {
    pub fn new(player: &Player, round_start_info: &RoundStartInfo) -> Self {
        todo!("Impl RoundPlayer::new()")
    }

    pub fn player_number(&self) -> PlayerNumber {
        self.player_number
    }

    pub fn items(&self) -> &[Option<Item>; 8] {
        &self.items
    }

    pub fn health(&self) -> u8 {
        self.health
    }

    pub fn unstun(&mut self) -> bool {
        let stunned = self.stunned;
        self.stunned = false;
        stunned
    }
}
