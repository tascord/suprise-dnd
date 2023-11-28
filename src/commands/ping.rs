use serenity::builder::{CreateCommand, CreateEmbed};
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> CreateEmbed {
    CreateEmbed::new().title("Hai!").color(0x6AFF00)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A ping command")
}
