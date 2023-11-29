use serde::{Deserialize, Deserializer, Serialize};
use serenity::{
    all::{ResolvedOption, User},
    builder::CreateEmbed,
};
use std::{io::Write, rc::Rc, sync::Mutex};
use std::{collections::HashMap, sync::Arc};

use super::{dice::Dice, effects::EFFECT_MAP, items::ITEM_MAP};

type RollFn = Arc<Box<dyn FnMut(Player, RollContext) -> RollContext + Send + Sync + 'static>>;
type PlayerFn = Arc<Box<dyn FnMut(Player) + Send + Sync + 'static>>;

#[derive(Deserialize, Serialize, Clone)]
pub struct Player {
    pub id: String,
    pub stats: Stats,
    pub effects: Vec<Effect>,
    pub inventory: HashMap<String, Item>,
}

pub enum StatType {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Stats {
    pub strength: u32,
    pub dexterity: u32,
    pub constitution: u32,
    pub intelligence: u32,
    pub wisdom: u32,
    pub charisma: u32,
}

#[derive(Clone)]
pub struct Effect {
    pub id: String,
    pub name: String,
    pub duration: i32,
    pub modifier: i32,
    pub on_turn: PlayerFn,
    pub on_roll: RollFn,
}

#[derive(Clone)]
pub struct Item {
    pub id: String,
    pub meta: ItemMeta,
    pub name: String,
    pub description: String,
    pub on_turn: Option<PlayerFn>,
    pub on_roll: Option<RollFn>,
    pub on_use: Option<PlayerFn>,
}

#[derive(Clone, Serialize, Default)]
pub struct ItemMeta {
    attachment: String,
    last_used: u32,
}

pub enum RollType {
    Attack(Option<Item>),
    Ability(StatType),
}

pub struct RollContext {
    pub roll_type: RollType,
    pub modifier: i32,
    pub on_success: Option<RollFn>,
    pub on_failure: Option<RollFn>,
    pub on_c_success: Option<RollFn>,
    pub on_c_failure: Option<RollFn>,
}

impl<'de> Deserialize<'de> for Effect {
    fn deserialize<D>(deserializer: D) -> Result<Effect, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        EFFECT_MAP
            .iter()
            .find(|(name, _)| *name == s)
            .map(|(_, effect)| effect.clone())
            .ok_or_else(|| serde::de::Error::custom("invalid effect"))
            .cloned()
    }
}

impl<'de> Deserialize<'de> for Item {
    fn deserialize<D>(deserializer: D) -> Result<Item, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ITEM_MAP
            .iter()
            .find(|(name, _)| *name == s)
            .map(|(_, item)| item.clone())
            .ok_or_else(|| serde::de::Error::custom("invalid item"))
            .cloned()
    }
}

impl Serialize for Effect {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        String::serialize(&self.name, serializer)
    }
}

impl Serialize for Item {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        String::serialize(&self.name, serializer)
    }
}

impl Player {
    pub fn new(id: String) -> Player {
        let stats = Stats {
            strength: Dice::stat_roll(),
            dexterity: Dice::stat_roll(),
            constitution: Dice::stat_roll(),
            intelligence: Dice::stat_roll(),
            wisdom: Dice::stat_roll(),
            charisma: Dice::stat_roll(),
        };

        Player {
            id,
            stats,
            effects: Vec::new(),
            inventory: HashMap::new(),
        }
    }

    pub fn collect(&mut self, mut item: Item) {
        let id = String::new(); // TODO: uuid
        item.meta.attachment = id.clone();
        self.inventory.insert(id, item);
    }
}

/// User, args
#[derive(Clone)]
pub struct CommandData<'a>(pub User, pub &'a [ResolvedOption<'a>], pub Arc<Mutex<GameState>>);

/// Embed, ephemeral
#[derive(Clone)]
pub struct CommandResponse(pub CreateEmbed, pub bool);

#[derive(Deserialize, Serialize, Clone)]
pub struct GameState {
    pub players: HashMap<String, Player>,
}

static GAME_FILE: &str = "game.json";
impl GameState {
    pub fn new() -> Self {
        if !std::path::Path::new(GAME_FILE).exists() {
            // Create new game file
            let mut file = std::fs::File::create(GAME_FILE).unwrap();
            let game_state = GameState {
                players: HashMap::new(),
            };

            let json = serde_json::to_string(&game_state).unwrap();
            file.write_all(json.as_bytes()).unwrap();
        }

        let file = std::fs::File::open(GAME_FILE).unwrap();
        let game_state: GameState = serde_json::from_reader(file).unwrap();

        game_state
    }

    pub fn save(&self) {
        let mut file = std::fs::File::create(GAME_FILE).unwrap();
        let json = serde_json::to_string(&self).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    pub fn get_player(&mut self, id: &str) -> &mut Player {
        self.players
            .entry(id.to_string())
            .or_insert_with(|| Player::new(id.to_string()))
    }
}
