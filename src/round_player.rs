use std::cmp::max;

use thiserror::Error;

use crate::{player::Player, player_number::PlayerNumber, round_start_info::RoundStartInfo};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StunState {
    Unstunned,
    Stunned,
    Recovering,
}

#[derive(Error, Debug, Clone, Copy)]
pub enum AlreadyStunnedError {
    #[error("Player is already stunned")]
    AlreadyStunned,
    #[error("Player can't be restunned while recovering")]
    CantStunWhileRecovering,
}

#[derive(Debug, Clone)]
pub struct RoundPlayer {
    player_number: PlayerNumber,
    health: i32,
    max_health: i32,
    stun_state: StunState,
}

impl RoundPlayer {
    pub fn new(player: &Player, round_start_info: &RoundStartInfo) -> Self {
        let max_health = round_start_info.max_health();
        RoundPlayer {
            player_number: player.number(),
            health: max_health,
            max_health,
            stun_state: StunState::Unstunned,
        }
    }

    pub fn player_number(&self) -> PlayerNumber {
        self.player_number
    }

    pub fn health(&self) -> i32 {
        self.health
    }

    pub fn stun_state(&self) -> StunState {
        self.stun_state
    }

    pub fn take_damage(&mut self, sawn: bool) -> bool {
        let damage = if sawn { 2 } else { 1 };

        self.health = max(0, self.health - damage);

        self.health == 0
    }

    pub fn gain_health(&mut self, amount: u8) {
        self.health = max(self.max_health, self.health + i32::from(amount));
    }

    pub fn stun(&mut self) -> Result<(), AlreadyStunnedError> {
        match self.stun_state {
            StunState::Unstunned => {
                self.stun_state = StunState::Stunned;
                Ok(())
            }
            StunState::Stunned => Err(AlreadyStunnedError::AlreadyStunned),
            StunState::Recovering => Err(AlreadyStunnedError::CantStunWhileRecovering),
        }
    }

    /// Updates the player's stun_state and returns true if the player can take their turn. Should only be called once prior to the player's turn
    pub fn update_stunned(&mut self) -> bool {
        match self.stun_state {
            StunState::Unstunned => true,
            StunState::Stunned => {
                self.stun_state = StunState::Recovering;
                false
            }
            StunState::Recovering => {
                self.stun_state = StunState::Unstunned;
                false
            }
        }
    }
}
