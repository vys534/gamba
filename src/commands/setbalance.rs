use std::collections::HashMap;

use sqlx::Sqlite;

use crate::{db::currency::adjust_currency_amount, model::command, OWNER_ID};

pub struct SetBalanceCommand;

#[serenity::async_trait]
impl command::Command for SetBalanceCommand {
    fn info(&self) -> command::CommandInfo {
        command::CommandInfo {
            module: crate::model::module::Class::Core,
            name: "setbalance",
            shorthand: "setbal",
            cooldown_length: 0,
            description: "manually increase/decrease the balance of any user by X",
            usage: "[user_id] [amount]",
            inner: Box::new(SetBalanceCommand),
        }
    }

    async fn exec(
        &self,
        ctx: &serenity::prelude::Context,
        message: &serenity::model::prelude::Message,
        mut db_conn: sqlx::pool::PoolConnection<Sqlite>,
        _redis_conn: redis::aio::ConnectionManager,
        args: (Vec<&str>, HashMap<String, &str>),
    ) -> Result<serenity::model::prelude::Message, crate::model::error::Error> {
        if message.author.id.0 != OWNER_ID {
            return Err(crate::model::error::Error::NoPermission);
        }
        if args.0.len() != 2 {
            return Err(crate::model::error::Error::InvalidArguments);
        }
        let user_id = str::parse::<u64>(args.0[0])
            .map_err(|_e: std::num::ParseIntError| crate::model::error::Error::ParseInt)?;
        let by = str::parse::<i64>(args.0[1])
            .map_err(|_e: std::num::ParseIntError| crate::model::error::Error::ParseInt)?;
        adjust_currency_amount(user_id, by, &mut db_conn).await?;
        message
            .channel_id
            .say(&ctx, ":ok_hand:")
            .await
            .map_err(crate::model::error::Error::Serenity)
    }
}
