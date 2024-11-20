use crate::{
    game_players::GamePlayers, multiplayer_count::MultiplayerCount, player_number::PlayerNumber, round::Round, round_number::RoundNumber, round_start_info::RoundStartInfo
};

#[derive(Debug, Clone)]
pub struct GameSession {
    multiplayer: bool,
    round_number: RoundNumber,
    round: Round,
    players: GamePlayers,
}

impl GameSession {
    pub fn new(multiplayer_option: Option<MultiplayerCount>) -> Self {
        let multiplayer;

        match multiplayer_option {
            Some(_) => multiplayer = true,
            None => multiplayer = false,
        }

        let players = GamePlayers::new(multiplayer_option);
        let round = Round::new(&players, RoundStartInfo::new(multiplayer), None);

        GameSession {
            round_number: RoundNumber::RoundOne,
            players,
            multiplayer,
            round,
        }
    }

    pub fn round_number(&self) -> RoundNumber {
        self.round_number
    }

    pub fn round(&self) -> &Round {
        &self.round
    }

    pub fn players(&self) -> &GamePlayers {
        &self.players
    }

    pub fn next_player(&self) -> PlayerNumber {
        todo!("Next player")
    }

    pub fn
}
