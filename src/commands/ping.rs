use crate::model::command;
use serenity;
use sqlx::Sqlite;
use std::collections::HashMap;

pub struct PingCommand;

#[serenity::async_trait]
impl command::Command for PingCommand {
    fn info(&self) -> command::CommandInfo {
        command::CommandInfo {
            module: crate::model::module::Class::Core,
            name: "ping",
            shorthand: "p",
            cooldown_length: 15,
            description: "responds with a message if alive, used for debugging",
            usage: "<none>",
            inner: Box::new(PingCommand),
        }
    }

    async fn exec(
        &self,
        ctx: &serenity::prelude::Context,
        message: &serenity::model::prelude::Message,
        _db_conn: sqlx::pool::PoolConnection<Sqlite>,
        _redis_conn: redis::aio::ConnectionManager,
        _args: (Vec<&str>, HashMap<String, &str>),
    ) -> Result<serenity::model::prelude::Message, crate::model::error::Error> {
        message
            .channel_id
            .say(&ctx, "pong")
            .await
            .map_err(crate::model::error::Error::Serenity)
    }
}
