use buckshot_roulette_gameplay_engine::{
    game_session::GameSession, multiplayer_count::MultiplayerCount, round::RoundContinuation,
};
use rand::{rngs::StdRng, SeedableRng};

#[test]
fn all_four_shoot_self() {
    let rng: StdRng = StdRng::seed_from_u64(42);

    let mut session: GameSession<StdRng> = GameSession::new(MultiplayerCount::Four, rng);
    play_round_one_shoots_two_two_shoots_self(&mut session);
    play_round_one_shoots_two_two_shoots_self(&mut session);
    play_round_one_shoots_two_two_shoots_self(&mut session);

    assert!(session.round().is_none());
}

fn play_round_one_shoots_two_two_shoots_self(session: &mut GameSession<StdRng>) {
    let mut won_round = false;
    for _ in 0..100 {
        let current_player = session.round().unwrap().next_player();
        session
            .with_turn(
                |turn| turn.shoot(current_player),
                |summary| {
                    assert!(summary.shot_result.is_some());
                    match &summary.round_continuation {
                        RoundContinuation::RoundContinues(_) => {}
                        RoundContinuation::RoundEnds(finished_round) => {
                            assert!(finished_round.winner() != current_player);
                            won_round = true;
                        }
                    }
                },
            )
            .unwrap();

        if won_round {
            break;
        }
    }

    assert!(won_round)
}
