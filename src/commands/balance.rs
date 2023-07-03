use std::collections::HashMap;

use crate::db::currency::get_current;
use crate::model::command;
use serenity;
use sqlx::Sqlite;

pub struct BalanceCommand;

#[serenity::async_trait]
impl command::Command for BalanceCommand {
    fn info(&self) -> command::CommandInfo {
        command::CommandInfo {
            module: crate::model::module::Class::GetItTwisted,
            name: "balance",
            shorthand: "bal",
            cooldown_length: 5,
            description: "check how rich (or poor) you are",
            usage: "",
            inner: Box::new(BalanceCommand),
        }
    }

    async fn exec(
        &self,
        ctx: &serenity::prelude::Context,
        message: &serenity::model::prelude::Message,
        mut db_conn: sqlx::pool::PoolConnection<Sqlite>,
        redis_conn: redis::aio::ConnectionManager,
        _args: (Vec<&str>, HashMap<String, &str>),
    ) -> Result<serenity::model::prelude::Message, crate::model::error::Error> {
        self.check_cooldown(redis_conn, &message.author.id).await?;

        let bal = get_current(&message.author.id, &mut db_conn).await?;
        message
            .channel_id
            .say(
                &ctx,
                format!("you currently have {}", crate::util::format_currency(bal),),
            )
            .await
            .map_err(crate::model::error::Error::Serenity)
    }
}
