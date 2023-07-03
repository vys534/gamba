use std::collections::HashMap;

use chrono::Utc;
use serenity;
use sqlx::Sqlite;

static COMMAND_LIST: tokio::sync::OnceCell<[CommandInfo; 8]> = tokio::sync::OnceCell::const_new();

pub async fn command_list() -> &'static [CommandInfo; 8] {
    COMMAND_LIST
        .get_or_init(|| async {
            [
                crate::commands::claim::ClaimCommand.info(),
                crate::commands::info::InfoCommand.info(),
                crate::commands::ping::PingCommand.info(),
                crate::commands::help::HelpCommand.info(),
                crate::commands::gamble::GambleCommand.info(),
                crate::commands::leaderboard::LeaderboardCommand.info(),
                crate::commands::balance::BalanceCommand.info(),
                crate::commands::setbalance::SetBalanceCommand.info(),
            ]
        })
        .await
}

pub type BoxCommand = Box<dyn Command + Send + Sync + 'static>;

pub struct CommandInfo {
    pub module: crate::model::module::Class,
    pub name: &'static str,
    pub shorthand: &'static str,
    pub cooldown_length: usize,
    pub description: &'static str,
    pub usage: &'static str,
    pub inner: BoxCommand,
}

#[serenity::async_trait]
pub trait Command {
    fn info(&self) -> CommandInfo;
    async fn exec(
        &self,
        ctx: &serenity::prelude::Context,
        message: &serenity::model::prelude::Message,
        mut db_conn: sqlx::pool::PoolConnection<Sqlite>,
        redis_conn: redis::aio::ConnectionManager,
        args: (Vec<&str>, HashMap<String, &str>),
    ) -> Result<serenity::model::prelude::Message, crate::model::error::Error>;
    async fn check_cooldown(
        &self,
        mut redis_conn: redis::aio::ConnectionManager,
        author: &serenity::model::prelude::UserId,
    ) -> Result<(), crate::model::error::Error> {
        let k = format!("{}:{}", author.0, self.info().name);
        let command_cooldown_active = redis::Cmd::get(&k)
            .query_async::<_, Option<usize>>(&mut redis_conn)
            .await
            .map_err(crate::model::error::Error::Redis)?;

        if let Some(timestamp) = command_cooldown_active {
            return Err(crate::model::error::Error::Cooldown(
                (timestamp + self.info().cooldown_length) as i64 - Utc::now().timestamp(),
            ));
        }
        redis::Cmd::set_ex(&k, Utc::now().timestamp(), self.info().cooldown_length)
            .query_async(&mut redis_conn)
            .await
            .map_err(crate::model::error::Error::Redis)?;
        Ok(())
    }
}
