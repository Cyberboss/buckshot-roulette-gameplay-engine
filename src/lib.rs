pub mod game_players;
pub mod game_session;
pub mod item;
pub mod loadout;
pub mod multiplayer_count;
pub mod player;
pub mod player_number;
pub mod round;
pub mod round_number;
pub mod round_player;
pub mod round_start_info;
pub mod seat;
pub mod shell;
pub mod turn;

pub(crate) const LOG_RNG: bool = cfg!(feature = "print_rng_to_stdout");
