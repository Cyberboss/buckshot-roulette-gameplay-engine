use std::collections::VecDeque;

use crate::{
    game_players::GamePlayers,
    player,
    player_number::PlayerNumber,
    round_player::RoundPlayer,
    round_start_info::RoundStartInfo,
    turn::{TakenTurn, Turn},
};

#[derive(Debug, Clone, Copy)]
pub enum Shell {
    Live,
    Blank,
}

#[derive(Debug, Clone)]
pub struct Round {
    turn_order: VecDeque<RoundPlayer>,
    turn_order_reversed: bool,
    first_dead_player: Option<PlayerNumber>,
    start_info: RoundStartInfo,
    shells: Vec<Shell>,
}

impl Round {
    pub fn new(
        game_players: &GamePlayers,
        start_info: RoundStartInfo,
        previous_round: Option<Round>,
    ) -> Self {
        let turn_order_reversed;
        let mut turn_order;
        let first_dead_player;

        let players = game_players.as_vec();

        let player_count = players.len();
        assert!(player_count != 0);

        let round_players_iter = players
            .iter()
            .map(|player| RoundPlayer::new(player, &start_info));

        match previous_round {
            Some(previous_round) => {
                turn_order_reversed = previous_round.turn_order_reversed;
                first_dead_player = previous_round.first_dead_player;
            }
            None => {
                turn_order_reversed = false;
                first_dead_player = None;
            }
        }

        if turn_order_reversed {
            turn_order = VecDeque::from_iter(round_players_iter.rev());
        } else {
            turn_order = VecDeque::from_iter(round_players_iter);
        }

        if let Some(first_dead_player) = first_dead_player {
            while turn_order[0].player_number() != first_dead_player {
                let moved_player = turn_order.pop_front();
                turn_order.push_back(moved_player.unwrap());
            }
        }

        let shells = start_info.generate_shells();

        Round {
            first_dead_player: None,
            turn_order_reversed,
            turn_order,
            start_info,
            shells,
        }
    }

    pub fn with_turn<F>(&mut self, func: F)
    where
        F: FnOnce(Turn) -> TakenTurn,
    {
        let player = self.turn_order.pop_front().unwrap();
        let turn = Turn::new(&player);
        let taken_turn = func(turn);

        todo!("Handle taken turn");
        self.turn_order.push_back(player);

        while self.turn_order[0].unstun() {
            let stunned_player = self.turn_order.pop_front().unwrap();
            self.turn_order.push_back(stunned_player);
        }
    }
}
