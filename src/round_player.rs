use std::cmp::max;

use thiserror::Error;

use crate::{
    item::Item, player::Player, player_number::PlayerNumber, round::ShotgunDamage,
    round_start_info::RoundStartInfo, shell::Shell,
};

#[derive(Debug, Clone, Copy)]
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
    stun_state: StunState,
}

impl RoundPlayer {
    pub fn new(player: &Player, round_start_info: &RoundStartInfo) -> Self {
        RoundPlayer {
            player_number: player.number(),
            health: round_start_info.max_health(),
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

    pub fn shoot(&mut self, shell: Shell, sawn: bool) -> ShotgunDamage {
        let killed;
        let damage;
        let shotgun_damage = if shell.fire() {
            if sawn {
                damage = 2;
                killed = damage > self.health;
                ShotgunDamage::SawedShot(killed)
            } else {
                damage = 1;
                killed = damage > self.health;
                ShotgunDamage::RegularShot(killed)
            }
        } else {
            damage = 0;
            ShotgunDamage::Blank
        };

        self.health = max(0, self.health - damage);

        shotgun_damage
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
