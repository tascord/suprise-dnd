pub mod commands;
pub mod constants;
pub mod game;

use game::types::{CommandData, CommandResponse, GameState};
use serenity::async_trait;
use serenity::builder::{CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use std::env;
use std::sync::Arc;

struct Bot {
    state: Arc<std::sync::Mutex<GameState>>,
}

impl Bot {
    fn new() -> Self {
        Self {
            state: Arc::new(std::sync::Mutex::new(GameState::new())),
        }
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let user = command.clone().user;
            let options = command.data.options();

            let data = CommandData(user, &options, self.state.clone());
            let response = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(data)),
                "roll" => Some(commands::roll::run(data)),
                "stats" => Some(commands::stats::run(data)),
                _ => Some(CommandResponse(
                    CreateEmbed::new().title("Not supported!").color(0xFF006A),
                    true,
                )),
            };

            if let Some(response) = response {
                let data = CreateInteractionResponseMessage::new()
                    .embed(response.0)
                    .ephemeral(response.1);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, _ready: Ready) {
        println!("Ready!");

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let _commands = guild_id
            .set_commands(
                &ctx.http,
                vec![
                    commands::ping::register(),
                    commands::roll::register(),
                    commands::stats::register(),
                ],
            )
            .await;
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("TOKEN").expect("Expected a token in the environment");

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Bot::new())
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
