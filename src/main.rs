use crate::discord::discord::Handler;
use dotenv::dotenv;
use reqwest::Client as ReqwestClient;
use serenity::prelude::*;
use serenity::Client;
use std::env;

// Import the API, models, discord and utility modules
pub mod api;
pub mod models;
pub mod util;
pub mod discord {
    pub mod commands;
    pub mod discord;
    pub mod messages;
}

#[tokio::main]
async fn main() {
    // Load environment variables from a .env file
    dotenv().ok();

    // Retrieve the Discord token from the environment variables
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Define the base URL for the API
    let base_url = "https://superna.ytechno.com.ng/api";
    // Create a new HTTP client
    let http_client = ReqwestClient::new();

    // Define the intents for the Discord bot (listening to guild messages and message content)
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    // Create a new Discord client with the specified token and intents
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler {
            http_client,
            base_url: base_url.to_string(),
        })
        .await
        .expect("Err creating client");

    // Start the client and handle any errors that occur
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
