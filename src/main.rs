use serenity::{prelude::GatewayIntents, Client};

mod commands;
mod db;
mod handler;
mod model;
mod util;

const PREFIX: &str = "&";
// replace with your discord user ID
const OWNER_ID: u64 = 0;

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("no token found");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("data.sqlite")
                .create_if_missing(true),
        )
        .await
        .expect("Error connecting to sqlite database");

    let redis_client =
        redis::Client::open("redis://127.0.0.1:6379/").expect("Error opening Redis connection");
    let conn = redis_client
        .get_tokio_connection_manager()
        .await
        .expect("Error getting Redis connection manager");

    let handler = crate::handler::Handler {
        db: database,
        redis: conn,
    };

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Err creating client");

    if let Err(e) = client.start().await {
        println!("Client error: {}", e)
    }
}
