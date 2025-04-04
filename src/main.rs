use bot::Handler;
use serenity::prelude::*;
use serenity::model::id::ChannelId;
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;
use log::{error, info};

mod bot;
mod api;
mod models;
mod web;

pub struct ChannelIdKey;
impl TypeMapKey for ChannelIdKey {
    type Value = Arc<RwLock<Option<ChannelId>>>;
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");

    // Create shared channel_id
    let channel_id = Arc::new(RwLock::new(None::<ChannelId>));
    info!("Created channel_id Arc: {:?}", Arc::as_ptr(&channel_id)); // Log the pointer for debugging

    // Set up Discord client
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::MESSAGE_CONTENT;

    let handler = Handler::new(channel_id.clone());
    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Error creating client");

    // Insert channel_id into TypeMap
    {
        let mut data = client.data.write().await;
        data.insert::<ChannelIdKey>(channel_id.clone());
        info!("Inserted channel_id into TypeMap: {:?}", Arc::as_ptr(&channel_id));
    }

    // Start Warp server
    tokio::spawn(web::start_warp_server(client.data.clone()));

    info!("Starting bot...");
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}