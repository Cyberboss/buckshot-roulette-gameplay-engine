use std::{borrow::BorrowMut, collections::VecDeque, iter::Filter, ops::IndexMut};

use rand::Rng;

use crate::{
    game_players::GamePlayers,
    loadout::{self, Loadout},
    player_number::PlayerNumber,
    round_player::RoundPlayer,
    round_start_info::RoundStartInfo,
    seat::{OccupiedSeat, Seat},
    shell::{Shell, ShellType},
    turn::{ShotgunTarget, TakenTurn, Turn},
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
pub enum ShotgunDamage {
    Blank,
    RegularShot(bool),
    SawedShot(bool),
}

#[derive(Debug, Clone)]
pub struct ShotResult {
    pub target_player: PlayerNumber,
    pub damage: ShotgunDamage,
}

#[derive(Debug, Clone)]
pub enum TurnContinuation {
    LoadoutContinues,
    /// PlayerNumber is the first player to go in the next loadout
    LoadoutEnds(PlayerNumber),
}

#[derive(Debug, Clone)]
pub struct ContinuedRound<TRng> {
    pub turn_continuation: TurnContinuation,
    pub round: Round<TRng>,
}

#[derive(Debug, Clone)]
pub enum RoundContinuation<TRng> {
    RoundContinues(ContinuedRound<TRng>),
    RoundEnds(FinishedRound<TRng>),
}

#[derive(Debug, Clone)]
pub struct TurnSummary<TRng> {
    pub shot_result: Option<ShotResult>,
    pub round_continuation: RoundContinuation<TRng>,
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

        let loadout = Loadout::new(&mut self.rng);

        let remaining_players = self
            .seats
            .iter()
            .filter(|seat| seat.player().is_some())
            .count();

        for seat in &mut self.seats {
            seat.get_new_items(loadout.new_items, remaining_players);
        }

        let mut blanks_to_load = loadout.initial_blank_rounds;
        let mut lives_to_load = loadout.initial_live_rounds;

        assert!(self.shells.len() == 0);
        while blanks_to_load > 0 && lives_to_load > 0 {
            if self.rng.gen_bool(0.5) {
                self.shells.push(Shell::new(ShellType::Blank));
                blanks_to_load -= 1;
            } else {
                self.shells.push(Shell::new(ShellType::Live));
                lives_to_load -= 1;
            }
        }

        for _ in 0..blanks_to_load {
            self.shells.push(Shell::new(ShellType::Blank));
        }
        for _ in 0..lives_to_load {
            self.shells.push(Shell::new(ShellType::Live));
        }
    }

    pub fn living_players(&self) -> impl Iterator<Item = &Seat> {
        self.seats.iter().filter(|&seat| match seat.player() {
            Some(_) => true,
            None => false,
        })
    }

    fn advance_turn(&mut self) {
        loop {
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

            let seat = self.seats.index_mut(self.active_seat_index);

            if let Some(occupied_seat) = seat.create_occupied_seat() {
                if occupied_seat.player.update_stunned() {
                    break;
                }
            }
        }
    }

    pub fn with_turn<F>(mut self, func: F) -> TurnSummary<TRng>
    where
        F: FnOnce(Turn) -> TakenTurn,
    {
        self.check_round_can_continue();

        let seat = self.seats.index_mut(self.active_seat_index);

        let occupied_seat = seat.create_occupied_seat().unwrap();
        let turn = Turn::new(occupied_seat, &mut self.shells);

        let taken_turn = func(turn);

        self.handle_taken_turn(taken_turn)
    }

    fn handle_taken_turn(mut self, taken_turn: TakenTurn) -> TurnSummary<TRng> {
        match taken_turn.target {
            ShotgunTarget::RackedEmpty => {
                self.new_loadout();

                TurnSummary {
                    shot_result: None,
                    round_continuation: self.continue_round(),
                }
            }
            ShotgunTarget::Shot(target_player_number) => {
                let shell = self.shells.pop().unwrap();

                let target_seat_index = match target_player_number {
                    PlayerNumber::One => 0,
                    PlayerNumber::Two => 1,
                    PlayerNumber::Three => 2,
                    PlayerNumber::Four => 3,
                };

                let target_seat = self
                    .seats
                    .index_mut(target_seat_index)
                    .create_occupied_seat()
                    .unwrap();

                let shotgun_damage = target_seat.player.shoot(shell, taken_turn.sawn);

                TurnSummary {
                    shot_result: Some(ShotResult {
                        target_player: target_player_number,
                        damage: shotgun_damage,
                    }),
                    round_continuation: self.continue_round(),
                }
            }
        }
    }

    fn continue_round(mut self) -> RoundContinuation<TRng> {
        self.advance_turn();
        RoundContinuation::RoundContinues(ContinuedRound {
            turn_continuation: TurnContinuation::LoadoutEnds(
                self.seats[self.active_seat_index]
                    .player()
                    .unwrap()
                    .player_number(),
            ),
            round: self,
        })
    }
}
