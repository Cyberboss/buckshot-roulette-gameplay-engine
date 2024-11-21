use rand::Rng;
use thiserror::Error;

use crate::{
    game_players::GamePlayers,
    multiplayer_count::MultiplayerCount,
    player_number::PlayerNumber,
    round::{FinishedRoundOrRng, Round},
    round_number::RoundNumber,
    turn::{TakenTurn, Turn},
};

#[derive(Debug, Clone)]
pub struct GameSession<TRng> {
    multiplayer: bool,
    round_number: RoundNumber,
    round: Option<Round<TRng>>,
    players: GamePlayers,
}

#[derive(Error, Debug, Clone, Copy)]
pub enum NoRoundError {
    #[error("No round is active")]
    NoRound,
}

impl<TRng> GameSession<TRng>
where
    TRng: Rng,
{
    pub fn new(multiplayer_option: Option<MultiplayerCount>, rng: TRng) -> Self {
        let multiplayer;

        match multiplayer_option {
            Some(_) => multiplayer = true,
            None => multiplayer = false,
        }

        let players = GamePlayers::new(multiplayer_option);
        let round = Some(Round::new(
            &players,
            multiplayer,
            FinishedRoundOrRng::Rng(rng),
        ));

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

    pub fn round(&self) -> Option<&Round<TRng>> {
        match &self.round {
            Some(round) => Some(round),
            None => None,
        }
    }

    pub fn with_turn<F>(&mut self, func: F) -> Result<Option<PlayerNumber>, NoRoundError>
    where
        F: FnOnce(Turn) -> TakenTurn,
    {
        match &mut self.round {
            Some(round) => match round.with_turn(func) {
                Some(_finished_round) => {
                    todo!("Handle finished round")
                }
                None => Ok(None),
            },
            None => Err(NoRoundError::NoRound),
        }
    }

    pub fn players(&self) -> &GamePlayers {
        &self.players
    }

    pub fn next_player(&self) -> PlayerNumber {
        todo!("Next player")
    }
}
