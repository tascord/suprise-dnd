use std::borrow::BorrowMut;

use serenity::all::{Command, CommandOptionType, ResolvedValue, User};
use serenity::builder::{CreateCommand, CreateCommandOption, CreateEmbed};
use serenity::model::application::ResolvedOption;

use crate::constants::is_dm;
use crate::game::types::{CommandData, CommandResponse, Player};

pub fn run(data: CommandData) -> CommandResponse {
    let dm = is_dm(&data.0);

    let user = data
        .1
        .first()
        .and_then(|option| match option {
            ResolvedOption {
                value: ResolvedValue::User(user, _),
                ..
            } => Some(user),
            _ => None,
        })
        .cloned();

    let mut game = data.2.lock().unwrap();
    let user = user.unwrap_or(&data.0);
    let player = game.get_player(&user.id.to_string()).borrow_mut();

    CommandResponse(embed(user.clone(), &player, dm), dm)
}

fn embed(user: User, player: &Player, dm: bool) -> CreateEmbed {
    let status: Vec<String> = player
        .effects
        .iter()
        .map(|e| format!("{}, level {} ({} rounds)", e.name, e.modifier, e.duration))
        .collect();
    let status = if status.is_empty() {
        "None".to_string()
    } else {
        status.join("\n")
    };

    let items = player
        .inventory
        .iter()
        .map(|(_, item)| item.name.clone())
        .collect::<Vec<String>>()
        .join("\n");
    let items = if items.is_empty() {
        "None".to_string()
    } else {
        items
    };

    let or_hidden = |v: u32| -> String {
        if dm {
            v.to_string()
        } else {
            "??".to_string()
        }
    };

    let stats = format!("**Strength**: `{}`, **Dexterity**: `{}`, **Constitution**: `{}`\n**Intelligence**: `{}`, **Wisdom**: `{}`, **Charisma**: `{}`",
            or_hidden(player.stats.strength),
            or_hidden(player.stats.dexterity),
            or_hidden(player.stats.constitution),
            or_hidden(player.stats.intelligence),
            or_hidden(player.stats.wisdom),
            or_hidden(player.stats.charisma)
        );

    CreateEmbed::new()
        .title(format!("Stats for {}", user.name))
        .color(0x6AFF00)
        .field("Stats", stats, false)
        .field("Status Effects", status, true)
        .field("Items", items, true)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("stats")
        .description("Get stats for a user")
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "user", "User to check stats of")
                .required(false),
        )
}
