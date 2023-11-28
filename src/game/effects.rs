use crate::game::items::SAND;

use super::types::{Effect, RollType};
use lazy_static::lazy_static;
use rand::prelude::IteratorRandom;
use rand::thread_rng;
use std::{borrow::BorrowMut, sync::Arc};

lazy_static! {
    pub static ref POISON: Effect = Effect {
        id: String::from("poison"),
        name: String::from("Poison"),
        duration: 2,
        modifier: 1,
        on_turn: Arc::new(Box::new(|_| {})),
        on_roll: Arc::new(Box::new(|_, mut r| {
            match r.roll_type {
                RollType::Ability(_) | RollType::Attack(_) => {
                    r.modifier -= 1;
                }
            }

            r
        })),
    };
    pub static ref CURSE_OF_RA: Effect = Effect {
        id: String::from("curse_of_ra"),
        name: String::from("Curse of Ra"),
        duration: 6,
        modifier: 1,
        on_turn: Arc::new(Box::new(|mut p| {
            let (i, _) = p
                .inventory
                .clone()
                .iter()
                .enumerate()
                .choose(&mut thread_rng())
                .unwrap();

            p.inventory.get_mut(i).replace(SAND.clone().borrow_mut());
        })),
        on_roll: Arc::new(Box::new(|_, r| r)),
    };
    pub static ref EFFECT_MAP: Vec<(&'static str, &'static Effect)> =
        vec![("Poison", &POISON), ("Curse of Ra", &CURSE_OF_RA)];
}
