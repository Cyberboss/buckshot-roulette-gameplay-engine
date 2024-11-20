use crate::{multiplayer_count::MultiplayerCount, player::Player, player_number::PlayerNumber};

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
}

impl GamePlayers {
    pub fn new(multiplayer_option: Option<MultiplayerCount>) -> Self {
        let player_1 = Player::new(PlayerNumber::One);
        let player_2 = Player::new(PlayerNumber::Two);
        let extra_players;

        match multiplayer_option {
            Some(player_count) => match player_count {
                MultiplayerCount::Two => extra_players = None,
                MultiplayerCount::Three | MultiplayerCount::Four => {
                    let player_4 = if player_count == MultiplayerCount::Four {
                        Some(Player::new(PlayerNumber::Four))
                    } else {
                        None
                    };

                    extra_players = Some(ExtraPlayers {
                        player_3: Player::new(PlayerNumber::Three),
                        player_4,
                    });
                }
            },
            None => extra_players = None,
        }

        GamePlayers {
            player_1,
            player_2,
            extra_players,
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
}
