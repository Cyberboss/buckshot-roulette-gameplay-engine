use crate::{
    game_players::GamePlayers,
    multiplayer_count::MultiplayerCount,
    player_number::PlayerNumber,
    round::{FinishedRoundOrRng, Round, RoundContinuation, TurnSummary},
    round_number::RoundNumber,
    turn::{TakenTurn, Turn},
};
use anyhow::Result;
use rand::Rng;
use thiserror::Error;

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
            round_number: RoundNumber::One,
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

    pub fn with_turn<TurnF, SummaryF, TRet>(
        &mut self,
        turn_func: TurnF,
        summary_func: SummaryF,
    ) -> Result<TRet>
    where
        TurnF: FnOnce(Turn) -> TakenTurn,
        SummaryF: FnOnce(&TurnSummary<TRng>) -> TRet,
    {
        match self.round.take() {
            Some(round) => {
                let turn_summary = round.with_turn(turn_func);
                let result = summary_func(&turn_summary);

                match turn_summary.round_continuation {
                    RoundContinuation::RoundContinues(continued_round) => {
                        self.round = Some(continued_round.round)
                    }
                    RoundContinuation::RoundEnds(finished_round) => {
                        self.players
                            .register_win(finished_round.winner(), self.round_number)?;

                        if self.round_number == RoundNumber::Three {
                            self.round = None;
                        } else {
                            self.round = Some(Round::new(
                                &self.players,
                                self.multiplayer,
                                FinishedRoundOrRng::FinishedRound(finished_round),
                            ))
                        }
                    }
                }

                Ok(result)
            }
            None => Err(NoRoundError::NoRound)?,
        }
    }

    pub fn players(&self) -> &GamePlayers {
        &self.players
    }

    pub fn next_player(&self) -> PlayerNumber {
        todo!("Next player")
    }
}
