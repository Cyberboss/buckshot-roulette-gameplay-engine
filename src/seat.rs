use std::{cmp::min, ops::Range};

use rand::Rng;

use crate::{
    item::{Item, NotAdreneline, UnaryItem},
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

    pub fn get_new_items<TRng>(
        &mut self,
        items_to_get: usize,
        remaining_players: usize,
        rng: &mut TRng,
    ) where
        TRng: Rng,
    {
        let allowed_items = min(items_to_get, MAX_ITEMS - self.items.len());

        for _ in 0..allowed_items {
            self.get_item(remaining_players, rng);
        }
    }

    fn get_item<TRng>(&mut self, remaining_players: usize, rng: &mut TRng)
    where
        TRng: Rng,
    {
        let mut item_pool = Vec::with_capacity(8);
        if remaining_players > 2 {
            item_pool.push(Item::NotAdreneline(NotAdreneline::UnaryItem(
                UnaryItem::Remote,
            )));
        }

        let index = rng.gen_range(Range {
            start: 0,
            end: item_pool.len(),
        });

        self.items.push(item_pool[index]);

        todo!("Finish item pool");
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
