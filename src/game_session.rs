use crate::{
    game_players::GamePlayers,
    multiplayer_count::MultiplayerCount,
    round::{FinishedRoundOrRng, Round, RoundContinuation, TurnSummary},
    round_number::RoundNumber,
    turn::{TakenAction, Turn},
};
use anyhow::Result;
use rand::Rng;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct GameSession<TRng> {
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

        GameSession { players, round }
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
    ) -> Result<Option<TRet>>
    where
        TurnF: FnOnce(Turn<TRng>) -> TakenAction<TRng>,
        SummaryF: FnOnce(&TurnSummary<TRng>) -> TRet,
    {
        match self.round.take() {
            Some(round) => {
                let turn_summary_option = round.with_turn(turn_func);
                let turn_summary = match turn_summary_option {
                    Some(turn_summary) => turn_summary,
                    None => return Ok(None),
                };

                let result = summary_func(&turn_summary);

                match turn_summary.round_continuation {
                    RoundContinuation::RoundContinues(continued_round) => {
                        self.round = Some(continued_round.round)
                    }
                    RoundContinuation::RoundEnds(finished_round) => {
                        let finished_round_number = finished_round.number();
                        self.players
                            .register_win(finished_round.winner(), finished_round_number)?;

                        match finished_round_number {
                            RoundNumber::One | RoundNumber::Two => {
                                self.round = Some(Round::new(
                                    &self.players,
                                    FinishedRoundOrRng::FinishedRound(finished_round),
                                ))
                            }
                            RoundNumber::Three => self.round = None,
                        }
                    }
                }

                Ok(Some(result))
            }
            None => Err(NoRoundError::NoRound)?,
        }
    }

    pub fn players(&self) -> &GamePlayers {
        &self.players
    }
}
