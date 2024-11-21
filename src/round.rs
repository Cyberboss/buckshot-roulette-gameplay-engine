use std::{borrow::BorrowMut, collections::VecDeque, iter::Filter, ops::IndexMut};

use rand::Rng;

use crate::{
    game_players::GamePlayers,
    player_number::PlayerNumber,
    round_player::RoundPlayer,
    round_start_info::RoundStartInfo,
    seat::Seat,
    shell::Shell,
    turn::{TakenTurn, Turn},
};
#[derive(Debug, Clone)]
pub struct Round<TRng> {
    seats: Vec<Seat>,
    turn_order_reversed: bool,
    active_seat_index: usize,
    first_dead_player: Option<PlayerNumber>,
    start_info: RoundStartInfo,
    shells: Vec<Shell>,
    rng: TRng,
}

#[derive(Debug, Clone)]
pub struct FinishedRound<TRng> {
    round: Round<TRng>,
    first_dead_player: PlayerNumber,
    winner: PlayerNumber,
}

#[derive(Debug, Clone)]
pub enum FinishedRoundOrRng<TRng> {
    FinishedRound(FinishedRound<TRng>),
    Rng(TRng),
}

impl<TRng> FinishedRound<TRng> {
    pub fn winner(&self) -> PlayerNumber {
        self.winner
    }
}

impl<'rng, TRng> Round<TRng>
where
    TRng: Rng,
{
    pub fn new(
        game_players: &GamePlayers,
        multiplayer: bool,
        round_or_rng: FinishedRoundOrRng<TRng>,
    ) -> Self {
        let turn_order_reversed;
        let starting_player;

        let players = game_players.as_vec();

        let player_count = players.len();
        assert!(player_count != 0);

        let mut rng;
        match round_or_rng {
            FinishedRoundOrRng::FinishedRound(finished_round) => {
                starting_player = finished_round.first_dead_player;
                rng = finished_round.round.rng;
                turn_order_reversed = finished_round.round.turn_order_reversed;
            }
            FinishedRoundOrRng::Rng(inital_rng) => {
                starting_player = PlayerNumber::One;
                rng = inital_rng;
                turn_order_reversed = false;
            }
        }

        let start_info = RoundStartInfo::new(multiplayer, starting_player, &mut rng);

        let mut turn_index = 0;

        let seats: Vec<Seat> = players
            .iter()
            .enumerate()
            .map(|(index, player)| {
                if player.number() == starting_player {
                    turn_index = index;
                }
                Seat::new(RoundPlayer::new(player, &start_info))
            })
            .collect();

        let shells = Vec::with_capacity(8);

        let mut round = Round {
            first_dead_player: None,
            turn_order_reversed,
            seats,
            start_info,
            shells,
            rng,
            active_seat_index: turn_index,
        };

        round.new_loadout();

        round
    }

    fn check_round_can_continue(&self) {
        assert!(self.shells.len() == 0);
        assert!(self.living_players().count() > 1);
    }

    fn new_loadout(&mut self) {
        self.check_round_can_continue();
    }

    pub fn living_players(&self) -> impl Iterator<Item = &Seat> {
        self.seats.iter().filter(|&seat| match seat.player() {
            Some(_) => true,
            None => false,
        })
    }

    fn advance_seat_index(&mut self) {
        let last_seat_index = self.seats.len() - 1;
        if self.turn_order_reversed {
            if self.active_seat_index == 0 {
                self.active_seat_index = last_seat_index;
            } else {
                self.active_seat_index -= 1;
            }
        } else {
            if self.active_seat_index == last_seat_index {
                self.active_seat_index = 0;
            } else {
                self.active_seat_index += 1;
            }
        }
    }

    pub fn with_turn<F>(&mut self, func: F) -> Option<FinishedRound<TRng>>
    where
        F: FnOnce(Turn) -> TakenTurn,
    {
        self.check_round_can_continue();

        loop {
            let seat = self.seats.index_mut(self.active_seat_index);

            if let Some(took_turn) = seat.with_occupied(|occupied_seat| {
                if !occupied_seat.player.update_stunned() {
                    return false;
                }

                let turn = Turn::new(occupied_seat);

                todo!("Turn taking");
            }) {
                if !took_turn {
                    self.advance_seat_index();
                    continue;
                }

                todo!("Handle finished round")
            }
        }
    }
}
