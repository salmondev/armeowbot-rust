use dotenv::dotenv;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;
use tracing::{info, warn};
use tracing_subscriber;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("Bot is connected as {}", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        info!(
            "Received message: '{}' from {}",
            msg.content, msg.author.name
        );

        if msg.content == "!ping" && !msg.author.bot {
            if let Err(why) = msg.channel_id.say(&ctx.http, "pong").await {
                warn!("Error sending message: {:?}", why);
            } else {
                info!("Responded with 'pong' to {}", msg.author.name);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Load the .env file
    dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    // Get the token from the .env file
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        warn!("Client error: {:?}", why);
    }
}
