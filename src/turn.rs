use std::{
    collections::VecDeque,
    ops::{IndexMut, Range},
};

use rand::Rng;
use thiserror::Error;

use crate::{
    item::{Item, NotAdreneline, UnaryItem},
    player_number::PlayerNumber,
    round_player::{RoundPlayer, StunState},
    seat::{OccupiedSeat, SeatView},
    shell::{Shell, ShellType},
};

#[derive(Debug)]
struct TurnOwnedData<'turn> {
    shells: &'turn mut VecDeque<Shell>,
    sawn: bool,
    turn_order_inverted: bool,
    occupied_seat: OccupiedSeat<'turn>,
}

#[derive(Debug)]
struct InnerTurn<'turn> {
    other_seats: Vec<SeatView>,
    owned_data: TurnOwnedData<'turn>,
}

#[derive(Debug)]
pub struct Turn<'turn> {
    inner_turn: InnerTurn<'turn>,
}

#[derive(Debug, Clone)]
pub enum TerminalAction {
    Item(ItemUseResult),
    Shot(PlayerNumber),
}

#[derive(Debug, Clone)]
pub struct TakenTurn {
    pub action: TerminalAction,
    pub sawn: bool,
    pub turn_order_inverted: bool,
}

#[derive(Debug)]
pub struct ContinuedTurn<'turn> {
    inner_turn: InnerTurn<'turn>,
    item_result: Result<ItemUseResult, InvalidItemUseError>,
}

#[derive(Debug)]
pub enum TakenAction<'turn> {
    Continued(ContinuedTurn<'turn>),
    Terminal(TakenTurn),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LearnedShell {
    pub relative_index: usize,
    pub shell_type: ShellType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemUseResult {
    Default,
    ShotgunRackedEmpty,
    LearnedShell(LearnedShell),
    StunnedPlayer(PlayerNumber),
}

#[derive(Debug, Clone, Copy, Error)]
pub enum InvalidItemUseError {
    #[error("The item is not present in the player's inventory")]
    NoItem,
    #[error("Cannot use adreneline due to being provided a target that doesn't exist, is the current player, or does not have the NotAdreneline item")]
    BadAdrenelineTarget,
    #[error("Shotgun is already sawn")]
    DoubleSaw,
    #[error("Player is recovering from previous stun and cannot be restunned")]
    DoubleStun,
    #[error("Player targeted by stun is dead, non-existent, or self")]
    InvalidStunTarget,
}

impl<'turn> ContinuedTurn<'turn> {
    pub fn item_result(&self) -> &Result<ItemUseResult, InvalidItemUseError> {
        &self.item_result
    }

    pub fn next_action(self) -> Turn<'turn> {
        Turn {
            inner_turn: self.inner_turn,
        }
    }
}

impl ItemUseResult {
    fn is_terminal(&self) -> bool {
        match self {
            ItemUseResult::ShotgunRackedEmpty => true,
            ItemUseResult::Default
            | ItemUseResult::LearnedShell(_)
            | ItemUseResult::StunnedPlayer(_) => false,
        }
    }
}

impl<'turn> Turn<'turn> {
    pub fn new(
        occupied_seat: OccupiedSeat<'turn>,
        other_seats: Vec<SeatView>,
        shells: &'turn mut VecDeque<Shell>,
    ) -> Turn<'turn> {
        Turn {
            inner_turn: InnerTurn {
                owned_data: TurnOwnedData {
                    occupied_seat,
                    shells,
                    sawn: false,
                    turn_order_inverted: false,
                },
                other_seats,
            },
        }
    }
    pub fn items(&self) -> &Vec<Item> {
        &self.inner_turn.owned_data.occupied_seat.items
    }

    pub fn other_seats(&self) -> &Vec<SeatView> {
        &self.inner_turn.other_seats
    }

    pub fn player(&self) -> &RoundPlayer {
        &self.inner_turn.owned_data.occupied_seat.player
    }

    pub fn turn_order_inverted(&self) -> bool {
        self.inner_turn.owned_data.turn_order_inverted
    }

    pub fn sawn(&self) -> bool {
        self.inner_turn.owned_data.sawn
    }

    pub fn use_unary_item<TRng>(
        mut self,
        unary_item: UnaryItem,
        rng: &mut TRng,
    ) -> TakenAction<'turn>
    where
        TRng: Rng,
    {
        let result = self.inner_turn.use_unary_item(unary_item, rng);
        self.convert_to_taken_action(result)
    }

    pub fn use_adreneline<TRng>(
        mut self,
        target_player: PlayerNumber,
        target_item: UnaryItem,
        rng: &mut TRng,
    ) -> TakenAction<'turn>
    where
        TRng: Rng,
    {
        let result = self
            .inner_turn
            .use_adreneline(target_player, target_item, rng);
        self.convert_to_taken_action(result)
    }

    pub fn use_jammer(mut self, target_player: PlayerNumber) -> TakenAction<'turn> {
        let result = self.inner_turn.use_jammer(target_player);
        self.convert_to_taken_action(result)
    }

    pub fn use_adreneline_then_jammer(
        mut self,
        theive_from: PlayerNumber,
        jam_target: PlayerNumber,
    ) -> TakenAction<'turn> {
        let result = self
            .inner_turn
            .use_adreneline_then_jammer(theive_from, jam_target);
        self.convert_to_taken_action(result)
    }

    fn convert_to_taken_action(
        self,
        mut item_result: Result<ItemUseResult, InvalidItemUseError>,
    ) -> TakenAction<'turn> {
        if let Ok(item_use_result) = item_result {
            if item_use_result.is_terminal() {
                return TakenAction::Terminal(TakenTurn {
                    action: TerminalAction::Item(item_use_result),
                    sawn: self.inner_turn.owned_data.sawn,
                    turn_order_inverted: self.inner_turn.owned_data.turn_order_inverted,
                });
            }

            item_result = Ok(item_use_result);
        }

        TakenAction::Continued(ContinuedTurn {
            item_result,
            inner_turn: self.inner_turn,
        })
    }
}

impl<'turn> InnerTurn<'turn> {
    fn use_unary_item<TRng>(
        &mut self,
        unary_item: UnaryItem,
        rng: &mut TRng,
    ) -> Result<ItemUseResult, InvalidItemUseError>
    where
        TRng: Rng,
    {
        self.owned_data.use_unary_item(None, unary_item, rng)
    }

    fn use_adreneline<TRng>(
        &mut self,
        target_player: PlayerNumber,
        target_item: UnaryItem,
        rng: &mut TRng,
    ) -> Result<ItemUseResult, InvalidItemUseError>
    where
        TRng: Rng,
    {
        self.with_adreneline(
            target_player,
            NotAdreneline::UnaryItem(target_item),
            |seat, owned_data| owned_data.use_unary_item(Some(&mut seat.items), target_item, rng),
        )
    }

    fn use_jammer(
        &mut self,
        target_player: PlayerNumber,
    ) -> Result<ItemUseResult, InvalidItemUseError> {
        self.check_can_jam(target_player)?;
        self.with_item(Item::NotAdreneline(NotAdreneline::Jammer), |_| {
            Ok(ItemUseResult::StunnedPlayer(target_player))
        })
    }

    fn use_adreneline_then_jammer(
        &mut self,
        theive_from: PlayerNumber,
        jam_target: PlayerNumber,
    ) -> Result<ItemUseResult, InvalidItemUseError> {
        self.check_can_jam(jam_target)?;
        self.with_adreneline(theive_from, NotAdreneline::Jammer, |_, _| {
            Ok(ItemUseResult::StunnedPlayer(jam_target))
        })
    }

    fn with_item<F>(&mut self, item: Item, func: F) -> Result<ItemUseResult, InvalidItemUseError>
    where
        F: FnOnce(&mut Self) -> Result<ItemUseResult, InvalidItemUseError>,
    {
        let index_to_remove = check_item_in_inventory(
            &self.owned_data.occupied_seat.items,
            item,
            InvalidItemUseError::NoItem,
        )?;

        let result = func(self);

        if result.is_ok() {
            self.owned_data.occupied_seat.items.remove(index_to_remove);
        }

        result
    }

    fn with_adreneline<F>(
        &mut self,
        target_player: PlayerNumber,
        target_item: NotAdreneline,
        func: F,
    ) -> Result<ItemUseResult, InvalidItemUseError>
    where
        F: FnOnce(&mut SeatView, &mut TurnOwnedData) -> Result<ItemUseResult, InvalidItemUseError>,
    {
        self.with_item(Item::Adreneline, |inner_self| {
            match get_opposing_seat(&mut inner_self.other_seats, target_player) {
                Some(seat) => {
                    check_item_in_inventory(
                        &seat.items,
                        Item::NotAdreneline(target_item),
                        InvalidItemUseError::BadAdrenelineTarget,
                    )?;
                    func(seat, &mut inner_self.owned_data)
                }
                None => Err(InvalidItemUseError::BadAdrenelineTarget),
            }
        })
    }

    fn check_can_jam(&mut self, target_player: PlayerNumber) -> Result<(), InvalidItemUseError> {
        match get_opposing_seat(&mut self.other_seats, target_player) {
            Some(view) => match view.stun_state {
                Some(stun_state) => match stun_state {
                    StunState::Unstunned => Ok(()),
                    StunState::Stunned | StunState::Recovering => {
                        Err(InvalidItemUseError::DoubleStun)
                    }
                },
                None => Err(InvalidItemUseError::InvalidStunTarget),
            },
            None => Err(InvalidItemUseError::InvalidStunTarget),
        }
    }
}

impl<'turn> TurnOwnedData<'turn> {
    fn use_unary_item<TRng>(
        &mut self,
        other_items: Option<&mut Vec<Item>>,
        unary_item: UnaryItem,
        rng: &mut TRng,
    ) -> Result<ItemUseResult, InvalidItemUseError>
    where
        TRng: Rng,
    {
        let items = match other_items {
            Some(other_items) => other_items,
            None => &mut self.occupied_seat.items,
        };

        let index_to_remove = check_item_in_inventory(
            items,
            Item::NotAdreneline(NotAdreneline::UnaryItem(unary_item)),
            InvalidItemUseError::NoItem,
        )?;

        let mut use_result = None;
        match unary_item {
            UnaryItem::Remote => self.turn_order_inverted = !self.turn_order_inverted,
            UnaryItem::Phone => {
                if self.shells.len() > 2 {
                    let relative_index = rng.gen_range(Range {
                        start: 2,
                        end: self.shells.len(),
                    });

                    use_result = learn_shell(&self.shells, relative_index)
                }
            }
            UnaryItem::Inverter => self.shells[0].invert(),
            UnaryItem::MagnifyingGlass => use_result = learn_shell(self.shells, 0),
            UnaryItem::Cigarettes => self.occupied_seat.player.gain_health(1),
            UnaryItem::Handsaw => {
                if self.sawn {
                    return Err(InvalidItemUseError::DoubleSaw);
                }
                self.sawn = true;
            }
            UnaryItem::Beer => {
                self.shells.pop_front();

                if self.shells.len() == 0 {
                    use_result = Some(ItemUseResult::ShotgunRackedEmpty);
                }
            }
        }

        items.remove(index_to_remove);

        Ok(use_result.unwrap_or(ItemUseResult::Default))
    }
}

fn get_opposing_seat(
    views: &mut Vec<SeatView>,
    player_number: PlayerNumber,
) -> Option<&mut SeatView> {
    match views
        .iter()
        .position(|seat| seat.player_number == player_number)
    {
        Some(seat_index) => Some(views.index_mut(seat_index)),
        None => None,
    }
}

fn learn_shell(shells: &VecDeque<Shell>, relative_index: usize) -> Option<ItemUseResult> {
    Some(ItemUseResult::LearnedShell(LearnedShell {
        relative_index,
        shell_type: shells[relative_index].shell_type(),
    }))
}

fn check_item_in_inventory(
    items: &Vec<Item>,
    target_item: Item,
    error: InvalidItemUseError,
) -> Result<usize, InvalidItemUseError> {
    let index_option = items.iter().position(|item| *item == target_item);

    match index_option {
        Some(index) => Ok(index),
        None => Err(error),
    }
}
