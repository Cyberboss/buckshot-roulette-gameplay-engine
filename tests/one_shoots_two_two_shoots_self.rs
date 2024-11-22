use buckshot_roulette_gameplay_engine::{
    game_session::GameSession, multiplayer_count::MultiplayerCount, player_number::PlayerNumber,
    round::RoundContinuation, round_number::RoundNumber, round_player::StunState,
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

#[test]
fn one_shoots_two_two_shoots_self() {
    let rng: ChaCha8Rng = ChaCha8Rng::seed_from_u64(42);

    let mut session: GameSession<ChaCha8Rng> = GameSession::new(MultiplayerCount::Two, rng);
    play_round_one_shoots_two_two_shoots_self(&mut session, RoundNumber::One);
    play_round_one_shoots_two_two_shoots_self(&mut session, RoundNumber::Two);
    play_round_one_shoots_two_two_shoots_self(&mut session, RoundNumber::Three);

    assert!(session.round().is_none());
    panic!("Force quit")
}

fn play_round_one_shoots_two_two_shoots_self(
    session: &mut GameSession<ChaCha8Rng>,
    round_number: RoundNumber,
) {
    let mut won_round = false;
    for _ in 0..100 {
        assert!(session.round().unwrap().number() == round_number);
        {
            let round = session.round().unwrap();

            let living_players = round.living_players().count();
            assert!(living_players == 2);
            if round_number == RoundNumber::One {
                assert!(round.next_player() == PlayerNumber::One);
            }

            round.living_players().for_each(|seat| {
                assert!(!seat.items().is_empty());
                let player = seat.player().unwrap();
                assert!(player.health() > 0);
                assert!(player.stun_state() == StunState::Unstunned);
            });
        }

        session
            .with_turn(
                |turn| turn.shoot(PlayerNumber::Two),
                |summary| {
                    assert!(summary.shot_result.is_some());
                    match &summary.round_continuation {
                        RoundContinuation::RoundContinues(_) => {}
                        RoundContinuation::RoundEnds(finished_round) => {
                            assert!(finished_round.winner() == PlayerNumber::One);
                            won_round = true;
                        }
                    }
                },
            )
            .unwrap();

        if won_round {
            check_round_win(session, round_number);
            break;
        }

        {
            let round = session.round().unwrap();
            assert!(round.living_players().count() == 2);
            if round_number == RoundNumber::One {
                assert!(round.next_player() == PlayerNumber::Two);
            }
        }

        session
            .with_turn(
                |turn| turn.shoot(PlayerNumber::Two),
                |summary| {
                    assert!(summary.shot_result.is_some());
                    match &summary.round_continuation {
                        RoundContinuation::RoundContinues(_) => {}
                        RoundContinuation::RoundEnds(finished_round) => {
                            assert!(finished_round.winner() == PlayerNumber::One);
                            won_round = true;
                        }
                    }
                },
            )
            .unwrap();

        if won_round {
            check_round_win(session, round_number);
            break;
        }
    }

    assert!(won_round)
}

fn check_round_win(session: &GameSession<ChaCha8Rng>, round_number: RoundNumber) {
    let player_one = session.players().as_vec()[0];
    match round_number {
        RoundNumber::One => {
            assert!(player_one.wins().len() == 1);
            assert!(player_one.wins().contains(&RoundNumber::One));
            assert!(session.round().unwrap().number() == RoundNumber::Two)
        }
        RoundNumber::Two => {
            assert!(player_one.wins().len() == 2);
            assert!(player_one.wins().contains(&RoundNumber::One));
            assert!(player_one.wins().contains(&RoundNumber::Two));
            assert!(session.round().unwrap().number() == RoundNumber::Three)
        }
        RoundNumber::Three => {
            assert!(player_one.wins().len() == 3);
            assert!(player_one.wins().contains(&RoundNumber::One));
            assert!(player_one.wins().contains(&RoundNumber::Two));
            assert!(player_one.wins().contains(&RoundNumber::Three));
            assert!(session.round().is_none())
        }
    }
}
