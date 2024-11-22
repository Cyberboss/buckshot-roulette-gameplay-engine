use thiserror::Error;

use crate::{
    multiplayer_count::MultiplayerCount, player::Player, player_number::PlayerNumber,
    round_number::RoundNumber,
};

#[derive(Debug, Clone)]
struct ExtraPlayers {
    player_3: Player,
    player_4: Option<Player>,
}

#[derive(Debug, Clone)]
pub struct GamePlayers {
    player_1: Player,
    player_2: Player,
    extra_players: Option<ExtraPlayers>,
    pub multiplayer_count: MultiplayerCount,
}

#[derive(Error, Debug, Clone, Copy)]
pub enum MissingPlayerError {
    #[error("Requested player is not registered!")]
    MissingPlayer,
}

impl GamePlayers {
    pub fn new(multiplayer_count: MultiplayerCount) -> Self {
        let player_1 = Player::new(PlayerNumber::One);
        let player_2 = Player::new(PlayerNumber::Two);
        let extra_players = match multiplayer_count {
            MultiplayerCount::Two => None,
            MultiplayerCount::Three | MultiplayerCount::Four => {
                let player_4 = if multiplayer_count == MultiplayerCount::Four {
                    Some(Player::new(PlayerNumber::Four))
                } else {
                    None
                };

                Some(ExtraPlayers {
                    player_3: Player::new(PlayerNumber::Three),
                    player_4,
                })
            }
        };

        GamePlayers {
            player_1,
            player_2,
            extra_players,
            multiplayer_count,
        }
    }

    pub fn as_vec(&self) -> Vec<&Player> {
        let mut vec = Vec::with_capacity(4);
        vec.push(&self.player_1);
        vec.push(&self.player_2);

        if let Some(extra_players) = &self.extra_players {
            vec.push(&extra_players.player_3);
            if let Some(player_4) = &extra_players.player_4 {
                vec.push(player_4);
            }
        }

        vec
    }

    pub fn register_win(
        &mut self,
        player_number: PlayerNumber,
        round_number: RoundNumber,
    ) -> Result<(), MissingPlayerError> {
        match player_number {
            PlayerNumber::One => self.player_1.register_win(round_number),
            PlayerNumber::Two => self.player_2.register_win(round_number),
            PlayerNumber::Three | PlayerNumber::Four => match &mut self.extra_players {
                Some(extra_players) => {
                    if player_number == PlayerNumber::Three {
                        extra_players.player_3.register_win(round_number);
                    } else if let Some(player_4) = &mut extra_players.player_4 {
                        player_4.register_win(round_number);
                    } else {
                        return Err(MissingPlayerError::MissingPlayer);
                    }
                }
                None => return Err(MissingPlayerError::MissingPlayer),
            },
        };

        Ok(())
    }
}
