use std::{collections::VecDeque, ops::IndexMut};

use rand::Rng;

use crate::{
    game_players::GamePlayers,
    item::initialize_item_count_map,
    loadout::Loadout,
    player_number::PlayerNumber,
    round_number::RoundNumber,
    round_player::RoundPlayer,
    round_start_info::RoundStartInfo,
    seat::Seat,
    shell::{Shell, ShellType, ShotgunDamage},
    turn::{ItemUseResult, TakenTurn, TerminalAction, Turn},
};
#[derive(Debug, Clone)]
pub struct Round<TRng> {
    round_number: RoundNumber,
    seats: Vec<Seat>,
    turn_order_reversed: bool,
    active_seat_index: usize,
    first_dead_player: Option<PlayerNumber>,
    start_info: RoundStartInfo,
    shells: VecDeque<Shell>,
    rng: TRng,
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

    pub fn number(&self) -> RoundNumber {
        self.round.round_number
    }
}

impl<TRng> Round<TRng>
where
    TRng: Rng,
{
    pub fn new(game_players: &GamePlayers, round_or_rng: FinishedRoundOrRng<TRng>) -> Self {
        let turn_order_reversed;
        let starting_player;

        let players = game_players.as_vec();

        let player_count = players.len();
        assert!(player_count != 0);

        let round_number;
        let mut rng;
        match round_or_rng {
            FinishedRoundOrRng::FinishedRound(finished_round) => {
                round_number = match finished_round.number() {
                    RoundNumber::One => RoundNumber::Two,
                    RoundNumber::Two => RoundNumber::Three,
                    RoundNumber::Three => panic!("Attempted to create round from round 3"),
                };
                starting_player = finished_round.first_dead_player;
                rng = finished_round.round.rng;
                turn_order_reversed = finished_round.round.turn_order_reversed;
            }
            FinishedRoundOrRng::Rng(inital_rng) => {
                round_number = RoundNumber::One;
                starting_player = PlayerNumber::One;
                rng = inital_rng;
                turn_order_reversed = false;
            }
        }

        let start_info =
            RoundStartInfo::new(starting_player, game_players.multiplayer_count, &mut rng);

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

        let shells = VecDeque::with_capacity(8);

        let mut round = Round {
            round_number,
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

    pub fn number(&self) -> RoundNumber {
        self.round_number
    }

    pub fn max_health(&self) -> i32 {
        self.start_info.max_health()
    }

    fn check_round_can_continue(&self) {
        assert!(
            self.living_players()
                .inspect(|seat| assert!(seat.player().unwrap().health() > 0))
                .count()
                > 1
        );
    }

    fn new_loadout(&mut self) {
        let loadout = Loadout::new(self.start_info.player_count, &mut self.rng);

        let remaining_players = self
            .seats
            .iter()
            .filter(|seat| seat.player().is_some())
            .count();

        let mut global_item_counts = initialize_item_count_map();
        for seat in &self.seats {
            for item in seat.items() {
                let count = global_item_counts.get_mut(item).unwrap();
                *count += 1
            }
        }

        // round robin because of global item limits
        for _ in 0..loadout.new_items {
            for seat in &mut self.seats {
                if let Some(added_item) =
                    seat.get_new_item(remaining_players, &global_item_counts, &mut self.rng)
                {
                    let count = global_item_counts.get_mut(&added_item).unwrap();
                    *count += 1
                }
            }
        }

        let mut blanks_to_load = loadout.initial_blank_rounds;
        let mut lives_to_load = loadout.initial_live_rounds;

        assert!(self.shells.is_empty());
        while blanks_to_load > 0 && lives_to_load > 0 {
            if self.rng.gen_bool(0.5) {
                self.shells.push_back(Shell::new(ShellType::Blank));
                blanks_to_load -= 1;
            } else {
                self.shells.push_back(Shell::new(ShellType::Live));
                lives_to_load -= 1;
            }
        }

        for _ in 0..blanks_to_load {
            self.shells.push_back(Shell::new(ShellType::Blank));
        }
        for _ in 0..lives_to_load {
            self.shells.push_back(Shell::new(ShellType::Live));
        }
    }

    pub fn living_players(&self) -> impl Iterator<Item = &Seat> {
        self.seats.iter().filter(|&seat| seat.player().is_some())
    }

    pub fn next_player(&self) -> PlayerNumber {
        self.seats[self.active_seat_index].player_number()
    }

    pub fn seats(&self) -> &Vec<Seat> {
        &self.seats
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
            } else if self.active_seat_index == last_seat_index {
                self.active_seat_index = 0;
            } else {
                self.active_seat_index += 1;
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
        assert!(!self.shells.is_empty());
        self.check_round_can_continue();

        let other_seats = self
            .seats
            .iter()
            .enumerate()
            .filter_map(|(index, seat)| {
                if index == self.active_seat_index {
                    None
                } else {
                    Some(seat.create_view())
                }
            })
            .collect();

        let seat = self.seats.index_mut(self.active_seat_index);

        let occupied_seat = seat.create_occupied_seat().unwrap();
        let turn = Turn::new(occupied_seat, other_seats, &mut self.shells);

        let taken_turn = func(turn);

        if taken_turn.turn_order_inverted {
            self.turn_order_reversed = !self.turn_order_reversed;
        }

        match taken_turn.action {
            TerminalAction::Item(item_use_result) => {
                // need to handle other cases
                assert!(item_use_result == ItemUseResult::ShotgunRackedEmpty);

                self.new_loadout();

                TurnSummary {
                    shot_result: None,
                    round_continuation: self.continue_round(),
                }
            }
            TerminalAction::Shot(target_player_number) => {
                let shell = self.shells.pop_front().unwrap();

                let target_seat_index = match target_player_number {
                    PlayerNumber::One => 0,
                    PlayerNumber::Two => 1,
                    PlayerNumber::Three => 2,
                    PlayerNumber::Four => 3,
                };

                let target_seat = self.seats.index_mut(target_seat_index);

                let mut occupied_seat = target_seat.create_occupied_seat().unwrap();

                let shotgun_damage = occupied_seat.shoot(shell, taken_turn.sawn);
                let outer_killed;
                match shotgun_damage {
                    ShotgunDamage::RegularShot(killed) | ShotgunDamage::SawedShot(killed) => {
                        outer_killed = killed;
                    }
                    ShotgunDamage::Blank => {
                        outer_killed = false;
                    }
                }

                let shot_result = Some(ShotResult {
                    target_player: target_player_number,
                    damage: shotgun_damage,
                });

                if outer_killed {
                    let first_dead_player = self
                        .first_dead_player
                        .unwrap_or_else(|| occupied_seat.player.player_number());

                    self.first_dead_player = Some(first_dead_player);

                    target_seat.empty_dead_body();

                    if self.living_players().count() == 1 {
                        let winner = self.living_players().next().unwrap().player_number();
                        return TurnSummary {
                            shot_result,
                            round_continuation: RoundContinuation::RoundEnds(FinishedRound {
                                first_dead_player,
                                winner,
                                round: self,
                            }),
                        };
                    }
                }

                if self.shells.is_empty() {
                    self.new_loadout();
                }

                TurnSummary {
                    shot_result,
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
