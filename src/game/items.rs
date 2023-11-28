use crate::game::types::Item;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SAND: Item = Item {
        id: String::from("sand"),
        name: String::from("Sand"),
        description: String::from("A handful of sand, seems useless..."),
        on_roll: None,
        on_turn: None,
        on_use: None,
    };
    pub static ref ITEM_MAP: Vec<(&'static str, &'static Item)> = vec![("Sand", &SAND),];
}
