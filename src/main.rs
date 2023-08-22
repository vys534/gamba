use serenity::{prelude::GatewayIntents, Client};

mod commands;
mod db;
mod handler;
mod model;
mod util;

const PREFIX: &str = "&";
// replace with your discord user ID--not required, but without it you cannot manually change users' balance
const OWNER_ID: u64 = 0;

#[tokio::main]
async fn main() {
    println!("Starting Gamba bot...");

    let token = std::env::var("DISCORD_TOKEN").expect("no token found");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("./data.sqlite")
                .create_if_missing(true),
        )
        .await
        .expect("Error connecting to sqlite database");

    println!("Connected to sqlite database");

    crate::db::setup(&mut database)
        .await
        .expect("Error setting up sqlite database");

    let redis_client = redis::Client::open(std::env::var("REDIS_URI").expect("no redis uri found"))
        .expect("Error opening Redis connection");
    let conn = redis_client
        .get_tokio_connection_manager()
        .await
        .expect("Error getting Redis connection manager");

    println!("Connected to Redis");

    let handler = crate::handler::Handler {
        db: database,
        redis: conn,
    };

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Err creating client");

    println!("Creating client, handing off to serenity");
    if let Err(e) = client.start().await {
        println!("Client error: {}", e)
    }
}
