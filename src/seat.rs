use std::{collections::HashMap, ops::Range};

use rand::Rng;

use crate::{
    item::{initialize_item_count_map, Item, NotAdreneline, UnaryItem},
    player_number::PlayerNumber,
    round_player::{RoundPlayer, StunState},
    shell::{Shell, ShotgunDamage},
};

const MAX_ITEMS: usize = 8;

#[derive(Debug, Clone)]
pub struct Seat {
    player_number: PlayerNumber,
    player: Option<RoundPlayer>,
    items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub struct SeatView {
    pub stun_state: Option<StunState>,
    pub player_number: PlayerNumber,
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub struct OccupiedSeat<'seat> {
    pub player: &'seat mut RoundPlayer,
    pub items: &'seat mut Vec<Item>,
}

impl Seat {
    pub fn new(player: RoundPlayer) -> Self {
        Seat {
            player_number: player.player_number(),
            player: Some(player),
            items: Vec::with_capacity(MAX_ITEMS),
        }
    }

    pub fn player_number(&self) -> PlayerNumber {
        self.player_number
    }

    pub fn player(&self) -> Option<&RoundPlayer> {
        match &self.player {
            Some(player) => Some(player),
            None => None,
        }
    }

    pub fn create_occupied_seat(&mut self) -> Option<OccupiedSeat> {
        match &mut self.player {
            Some(player) => {
                let occupied_seat = OccupiedSeat {
                    player,
                    items: &mut self.items,
                };
                Some(occupied_seat)
            }
            None => None,
        }
    }

    pub fn empty_dead_body(&mut self) {
        let player = self.player.as_ref().unwrap();
        assert!(player.health() == 0);
        self.player = None;
    }

    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }

    pub fn create_view(&self) -> SeatView {
        SeatView {
            stun_state: match &self.player {
                Some(player) => Some(player.stun_state()),
                None => None,
            },
            player_number: self.player_number,
            items: self.items.clone(),
        }
    }

    pub fn get_new_item<TRng>(
        &mut self,
        remaining_players: usize,
        current_table_item_counts: &HashMap<Item, usize>,
        rng: &mut TRng,
    ) -> Option<Item>
    where
        TRng: Rng,
    {
        if self.player.is_none() {
            return None;
        }

        let mut item_pool = Vec::with_capacity(current_table_item_counts.len());

        let mut player_item_counts = initialize_item_count_map();
        self.items.iter().for_each(|item| {
            let count = player_item_counts.get_mut(item).unwrap();
            *count += 1;
        });

        current_table_item_counts.keys().for_each(|item| {
            add_item_to_pool_checked(
                &mut item_pool,
                *item,
                current_table_item_counts,
                &mut player_item_counts,
                || {
                    *item != Item::NotAdreneline(NotAdreneline::UnaryItem(UnaryItem::Remote))
                        || remaining_players > 2
                },
            )
        });

        let index = rng.gen_range(Range {
            start: 0,
            end: item_pool.len(),
        });

        let item = item_pool[index];
        self.items.push(item);
        Some(item)
    }
}

impl<'seat> OccupiedSeat<'seat> {
    pub fn shoot(&mut self, shell: Shell, sawn: bool) -> ShotgunDamage {
        if shell.fire() {
            let killed = self.player.take_damage(sawn);
            if sawn {
                ShotgunDamage::SawedShot(killed)
            } else {
                ShotgunDamage::RegularShot(killed)
            }
        } else {
            ShotgunDamage::Blank
        }
    }
}

fn global_item_limit(item: Item) -> usize {
    match item {
        Item::NotAdreneline(not_adreneline) => match not_adreneline {
            NotAdreneline::UnaryItem(unary_item) => match unary_item {
                UnaryItem::Remote => 2,
                UnaryItem::Phone
                | UnaryItem::Inverter
                | UnaryItem::MagnifyingGlass
                | UnaryItem::Cigarettes
                | UnaryItem::Handsaw
                | UnaryItem::Beer => 32,
            },
            NotAdreneline::Jammer => 1,
        },
        Item::Adreneline => 32,
    }
}

fn add_item_to_pool_checked<F>(
    pool: &mut Vec<Item>,
    item: Item,
    current_table_item_counts: &HashMap<Item, usize>,
    player_item_counts: &mut HashMap<Item, usize>,
    additional_check: F,
) where
    F: FnOnce() -> bool,
{
    let player_item_limit = player_item_limit(item);
    let current_count = player_item_counts.get(&item);
    if player_item_limit >= *current_count.unwrap() {
        return;
    }

    let global_item_limit = global_item_limit(item);
    let global_count = current_table_item_counts.get(&item);
    if global_item_limit >= *global_count.unwrap() {
        return;
    }

    if !additional_check() {
        return;
    }

    pool.push(item);
}

fn player_item_limit(item: Item) -> usize {
    match item {
        Item::NotAdreneline(not_adreneline) => match not_adreneline {
            NotAdreneline::UnaryItem(unary_item) => match unary_item {
                UnaryItem::Remote | UnaryItem::Cigarettes => 1,
                UnaryItem::MagnifyingGlass | UnaryItem::Handsaw => 2,
                UnaryItem::Inverter => 4,
                UnaryItem::Phone | UnaryItem::Beer => 8,
            },
            NotAdreneline::Jammer => 1,
        },
        Item::Adreneline => 4,
    }
}
