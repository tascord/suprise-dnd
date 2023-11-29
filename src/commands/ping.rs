use serenity::builder::{CreateCommand, CreateEmbed};

use crate::game::types::{CommandData, CommandResponse};

pub fn run(_: CommandData) -> CommandResponse {
    let embed = CreateEmbed::new().title("Hai!").color(0x6AFF00);
    CommandResponse(embed, true)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A ping command")
}
