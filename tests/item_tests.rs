use buckshot_roulette_gameplay_engine::{
    game_session::GameSession,
    item::{Item, NotAdreneline, UnaryItem},
    multiplayer_count::MultiplayerCount,
    player_number::PlayerNumber,
    round::RoundContinuation,
    turn::{ItemUseResult, TakenAction, TakenTurn, Turn},
};
use rand::{rngs::StdRng, SeedableRng};

/// Shoot each other in a two player scenario until an item is available
fn item_test_core<F>(target_item: Item, seed: u64, action: F)
where
    F: FnOnce(Turn<StdRng>, PlayerNumber) -> TakenTurn,
{
    let rng: StdRng = StdRng::seed_from_u64(seed);

    let mut session: GameSession<StdRng> = GameSession::new(MultiplayerCount::Two, rng);
    match play_round_shoot_each_other(&mut session, target_item, action) {
        Some(action) => match play_round_shoot_each_other(&mut session, target_item, action) {
            Some(action) => match play_round_shoot_each_other(&mut session, target_item, action) {
                Some(_) => panic!("Current seed never spawned necessary item!"),
                None => {}
            },
            None => {}
        },
        None => {}
    }
}

fn play_round_shoot_each_other<F>(
    session: &mut GameSession<StdRng>,
    target_item: Item,
    action: F,
) -> Option<F>
where
    F: FnOnce(Turn<StdRng>, PlayerNumber) -> TakenTurn,
{
    let mut action_option = Some(action);
    for _ in 0..100 {
        let mut won_round = false;
        session
            .with_turn(
                |turn| {
                    let has_item = turn.items().iter().any(|item| *item == target_item);

                    let current_player = turn.player().player_number();
                    let player_to_shoot = match current_player {
                        PlayerNumber::One => PlayerNumber::Two,
                        PlayerNumber::Two => PlayerNumber::One,
                        PlayerNumber::Three | PlayerNumber::Four => panic!("Unexpected player"),
                    };

                    if has_item {
                        if let Some(action) = action_option.take() {
                            return action(turn, player_to_shoot);
                        }
                    }

                    turn.shoot(player_to_shoot)
                },
                |summary| {
                    assert!(summary.shot_result.is_some());
                    match &summary.round_continuation {
                        RoundContinuation::RoundContinues(_) => {}
                        RoundContinuation::RoundEnds(_) => {
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

    action_option
}

#[test]
fn test_phone() {
    item_test_core(
        Item::NotAdreneline(NotAdreneline::UnaryItem(UnaryItem::Phone)),
        42,
        |turn, player_to_shoot| {
            let item_use = turn.use_unary_item(UnaryItem::Phone);
            match item_use {
                TakenAction::Continued(continued_turn) => {
                    match continued_turn.item_result() {
                        Ok(use_result) => match use_result {
                            ItemUseResult::Default => {}
                            ItemUseResult::LearnedShell(learned_shell) => {
                                assert!(learned_shell.relative_index > 1)
                            }
                            ItemUseResult::ShotgunRackedEmpty | ItemUseResult::StunnedPlayer(_) => {
                                panic!("Should be impossible with phone")
                            }
                        },
                        Err(_) => panic!("Phone should never have a bad use result"),
                    }

                    continued_turn.next_action().shoot(player_to_shoot)
                }
                TakenAction::Terminal(_) => {
                    panic!("Phone shouldn't be a terminal action")
                }
            }
        },
    );
}
