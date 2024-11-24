use buckshot_roulette_gameplay_engine::{
    game_session::GameSession,
    item::{Item, NotAdreneline, UnaryItem},
    multiplayer_count::MultiplayerCount,
    player_number::PlayerNumber,
    round::RoundContinuation,
    turn::{ItemUseResult, TakenAction, Turn},
};
use rand::{rngs::StdRng, SeedableRng};

/// Shoot each other in a two player scenario until an item is available
fn item_test_core<F>(target_item: Item, seed: u64, action: F)
where
    F: FnOnce(Turn<StdRng>, PlayerNumber) -> TakenAction<StdRng>,
{
    let rng: StdRng = StdRng::seed_from_u64(seed);

    let mut session: GameSession<StdRng> = GameSession::new(MultiplayerCount::Two, rng);
    if let Some(action) = play_round_shoot_each_other(&mut session, target_item, action) {
        if let Some(action) = play_round_shoot_each_other(&mut session, target_item, action) {
            if play_round_shoot_each_other(&mut session, target_item, action).is_some() {
                panic!("Current seed never spawned necessary item!");
            }
        }
    }
}

fn play_round_shoot_each_other<F>(
    session: &mut GameSession<StdRng>,
    target_item: Item,
    action: F,
) -> Option<F>
where
    F: FnOnce(Turn<StdRng>, PlayerNumber) -> TakenAction<StdRng>,
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
            let shell_count = turn.shell_count();
            let item_use = turn.use_unary_item(UnaryItem::Phone);
            match item_use {
                TakenAction::Continued(continued_turn) => {
                    match continued_turn.item_result() {
                        Ok(use_result) => match use_result {
                            ItemUseResult::Default => assert!(shell_count <= 2),
                            ItemUseResult::LearnedShell(learned_shell) => {
                                assert!(shell_count > 2);
                                assert!(learned_shell.relative_index > 1)
                            }
                            ItemUseResult::ShotgunRacked(_) | ItemUseResult::StunnedPlayer(_) => {
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

#[test]
fn test_beer() {
    item_test_core(
        Item::NotAdreneline(NotAdreneline::UnaryItem(UnaryItem::Beer)),
        42,
        |turn, player_to_shoot| {
            let shell_count = turn.shell_count();
            let item_use = turn.use_unary_item(UnaryItem::Beer);
            match item_use {
                TakenAction::Continued(continued_turn) => {
                    match continued_turn.item_result() {
                        Ok(use_result) => match use_result {
                            ItemUseResult::ShotgunRacked(rack_result) => {
                                assert!(rack_result.empty == (shell_count == 1));
                            }
                            ItemUseResult::Default
                            | ItemUseResult::LearnedShell(_)
                            | ItemUseResult::StunnedPlayer(_) => {
                                panic!("Shouldn't be possible with beer")
                            }
                        },
                        Err(_) => panic!("Beer should never have a bad use result"),
                    }

                    continued_turn.next_action().shoot(player_to_shoot)
                }
                TakenAction::Terminal(_) => {
                    panic!("Beer shouldn't be a terminal action")
                }
            }
        },
    );
}

#[test]
fn test_cigs() {
    let mut healed_once = false;
    item_test_core(
        Item::NotAdreneline(NotAdreneline::UnaryItem(UnaryItem::Cigarettes)),
        42,
        |turn, player_to_shoot| {
            let prior_health = turn.player().health();
            let item_use = turn.use_unary_item(UnaryItem::Cigarettes);
            match item_use {
                TakenAction::Continued(continued_turn) => {
                    match continued_turn.item_result() {
                        Ok(use_result) => match use_result {
                            ItemUseResult::Default => {}
                            ItemUseResult::LearnedShell(_)
                            | ItemUseResult::StunnedPlayer(_)
                            | ItemUseResult::ShotgunRacked(_) => {
                                panic!("Shouldn't be possible with beer")
                            }
                        },
                        Err(_) => panic!("Cigs should never have a bad use result"),
                    }

                    let next_action = continued_turn.next_action();

                    assert!(prior_health <= next_action.player().health());
                    if prior_health < next_action.player().health() {
                        healed_once = true;
                    }

                    next_action.shoot(player_to_shoot)
                }
                TakenAction::Terminal(_) => {
                    panic!("Cigs shouldn't be a terminal action")
                }
            }
        },
    );

    assert!(healed_once);
}
