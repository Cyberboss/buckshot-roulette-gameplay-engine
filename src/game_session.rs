use crate::{
    game_players::GamePlayers,
    multiplayer_count::MultiplayerCount,
    round::{FinishedRoundOrRng, Round, RoundContinuation, TurnSummary},
    round_number::RoundNumber,
    turn::{TakenTurn, Turn},
};
use anyhow::Result;
use rand::Rng;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct GameSession<TRng> {
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
    pub fn new(multiplayer_count: MultiplayerCount, rng: TRng) -> Self {
        let players = GamePlayers::new(multiplayer_count);
        let round = Some(Round::new(&players, FinishedRoundOrRng::Rng(rng)));

        GameSession {
            round_number: RoundNumber::One,
            players,
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
}
