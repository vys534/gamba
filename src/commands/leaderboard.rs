use std::collections::HashMap;

use crate::db::leaderboard::get_leaderboard;
use crate::db::leaderboard::LeaderboardType;
use crate::model::command;
use serenity;
use sqlx::Sqlite;

pub struct LeaderboardCommand;

#[serenity::async_trait]
impl command::Command for LeaderboardCommand {
    fn info(&self) -> command::CommandInfo {
        command::CommandInfo {
            module: crate::model::module::Class::GetItTwisted,
            name: "leaderboard",
            shorthand: "lb",
            cooldown_length: 5,
            description: "get the richest users",
            usage: "[alltime|current(default)|streak] -p [page_number]",
            inner: Box::new(LeaderboardCommand),
        }
    }

    async fn exec(
        &self,
        ctx: &serenity::prelude::Context,
        message: &serenity::model::prelude::Message,
        mut db_conn: sqlx::pool::PoolConnection<Sqlite>,
        redis_conn: redis::aio::ConnectionManager,
        args: (Vec<&str>, HashMap<String, &str>),
    ) -> Result<serenity::model::prelude::Message, crate::model::error::Error> {
        self.check_cooldown(redis_conn, &message.author.id).await?;

        let page = match args.1.get("p") {
            Some(v) => str::parse::<i64>(v).map_err(|_e| crate::model::error::Error::ParseInt)?,
            None => 0,
        };
        if !(0..=99).contains(&page) {
            return Err(crate::model::error::Error::OutOfBounds);
        }

        let t = if !args.0.is_empty() {
            match args.0[0].to_ascii_lowercase().as_str() {
                "alltime" => LeaderboardType::CurrencyBest,
                "current" => LeaderboardType::CurrencyHeld,
                "streak" => LeaderboardType::StreakBest,
                _ => LeaderboardType::CurrencyHeld,
            }
        } else {
            LeaderboardType::CurrencyHeld
        };

        let ((self_position, self_lb), all_lb) =
            get_leaderboard(&mut db_conn, page, &t, &message.author.id).await?;

        message
            .channel_id
            .say(
                &ctx,
                format!(
                    "{} __**{}**__ {}\n{}\n-----\n{}",
                    crate::model::emoji::Emoji::WhatsappGold.to_string(),
                    match t {
                        LeaderboardType::CurrencyBest => "All Time Richest Whatsappians",
                        LeaderboardType::CurrencyHeld => "Current Richest Whatsappians",
                        LeaderboardType::StreakBest => "All Time Highest Daily Streak",
                    },
                    crate::model::emoji::Emoji::WhatsappGold.to_string(),
                    all_lb
                        .iter()
                        .enumerate()
                        .map(|(i, l)| crate::util::format_lb_entry(l, (i + 1) as i64, &t, false))
                        .collect::<Vec<String>>()
                        .join("\n"),
                    crate::util::format_lb_entry(&self_lb, self_position, &t, true)
                ),
            )
            .await
            .map_err(crate::model::error::Error::Serenity)
    }
}
