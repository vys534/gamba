use std::collections::HashMap;

use crate::model::command;
use serenity;
use sqlx::Sqlite;

pub struct InfoCommand;

#[serenity::async_trait]
impl command::Command for InfoCommand {
    fn info(&self) -> command::CommandInfo {
        command::CommandInfo {
            module: crate::model::module::Class::Core,
            name: "info",
            shorthand: "i",
            cooldown_length: 10,
            description: "info about the bot",
            usage: "",
            inner: Box::new(InfoCommand),
        }
    }

    async fn exec(
        &self,
        ctx: &serenity::prelude::Context,
        message: &serenity::model::prelude::Message,
        _db_conn: sqlx::pool::PoolConnection<Sqlite>,
        redis_conn: redis::aio::ConnectionManager,
        _args: (Vec<&str>, HashMap<String, &str>),
    ) -> Result<serenity::model::prelude::Message, crate::model::error::Error> {
        self.check_cooldown(redis_conn, &message.author.id).await?;
        message
            .channel_id
            .say(&ctx, "Whatsapp Casino discord bot")
            .await
            .map_err(crate::model::error::Error::Serenity)
    }
}
