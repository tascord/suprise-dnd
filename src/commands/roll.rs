use serenity::all::{CommandOptionType, ResolvedValue};
use serenity::builder::{CreateCommand, CreateCommandOption, CreateEmbed};
use serenity::model::application::ResolvedOption;

use crate::game::dice::Dice;

pub fn run(options: &[ResolvedOption]) -> CreateEmbed {
    if let Some(ResolvedOption {
        value: ResolvedValue::String(roll),
        ..
    }) = options.first()
    {
        match Dice::from_string(roll) {
            Ok(result) => embed(roll, result),
            Err(_) => error_embed(),
        }
    } else {
        error_embed()
    }
}

fn embed(dice: &str, result: u32) -> CreateEmbed {
    CreateEmbed::new()
        .title(format!("Rolled {}, counted {} total", dice, result))
        .color(0x6AFF00)
}

fn error_embed() -> CreateEmbed {
    CreateEmbed::new()
        .title("Please provide a valid roll")
        .color(0xFF006A)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("roll")
        .description("Roll some dice! (e.g. 2d6+1)")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "dice",
                "Dice to roll, e.g. 2d6+1",
            )
            .required(true),
        )
}
