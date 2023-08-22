use std::collections::HashMap;

use crate::model::command;
use serenity;
use sqlx::Sqlite;

pub struct ClaimCommand;

#[serenity::async_trait]
impl command::Command for ClaimCommand {
    fn info(&self) -> command::CommandInfo {
        command::CommandInfo {
            module: crate::model::module::Class::GetItTwisted,
            name: "claim",
            shorthand: "c",
            cooldown_length: 30,
            description: "claim free credits every 24 hours",
            usage: "",
            inner: Box::new(ClaimCommand),
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

        let (streak_old, streak_new, streak_best) =
            crate::db::claim::handle(&message.author.id, &mut db_conn).await?;
        let now = crate::db::currency::current(&message.author.id, &mut db_conn).await?;
        let streak_clamp = if streak_new >= 7 { 6 } else { streak_new };
        message.channel_id.say(&ctx,
            format!("Free money sir, you got {} + {} (Streak bonus!). You now have {}\nCome back in 24 hours to claim again\nCurrent streak: {} -> **{}** (Record: {})\nNext bonus: {}", 
                crate::util::format_currency(crate::db::claim::DAILY_STREAK_AMOUNT),
                // TODO: clamp function
                crate::util::format_currency(crate::db::claim::DAILY_STREAK_BONUS_SCALE[streak_clamp as usize]),
                crate::util::format_currency(now),
                streak_old,
                streak_new,
                streak_best,
                crate::util::format_currency(crate::db::claim::DAILY_STREAK_BONUS_SCALE[streak_clamp as usize]),
            ))
            .await
            .map_err(crate::model::error::Error::Serenity)
    }
}
