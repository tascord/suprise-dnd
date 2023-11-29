use crate::game::types::{Item, ItemMeta};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SAND: Item = Item {
        id: String::from("item_sand"),
        meta: ItemMeta::default(),
        name: String::from("Sand"),
        description: String::from("A handful of sand, seems useless..."),
        on_roll: None,
        on_turn: None,
        on_use: None,
    };
    pub static ref ITEM_MAP: Vec<(&'static str, &'static Item)> = vec![("item_sand", &SAND),];
}
