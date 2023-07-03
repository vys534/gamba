use std::collections::HashMap;

use crate::model::command;
use serenity;
use sqlx::Sqlite;

pub struct HelpCommand;

#[serenity::async_trait]
impl command::Command for HelpCommand {
    fn info(&self) -> command::CommandInfo {
        command::CommandInfo {
            module: crate::model::module::Class::Core,
            name: "help",
            shorthand: "h",
            cooldown_length: 10,
            description: "get info/help with any command, or view the command list",
            usage: "cmd_name",
            inner: Box::new(HelpCommand),
        }
    }

    async fn exec(
        &self,
        ctx: &serenity::prelude::Context,
        message: &serenity::model::prelude::Message,
        _db_conn: sqlx::pool::PoolConnection<Sqlite>,
        redis_conn: redis::aio::ConnectionManager,
        args: (Vec<&str>, HashMap<String, &str>),
    ) -> Result<serenity::model::prelude::Message, crate::model::error::Error> {
        self.check_cooldown(redis_conn, &message.author.id).await?;
        if args.0.is_empty() {
            return message
                .channel_id
                .say(
                    &ctx,
                    format!(
                        "List of all commands:\n{}",
                        crate::model::command::command_list()
                            .await
                            .iter()
                            .map(|c| format!("`{}`", c.name))
                            .collect::<Vec<String>>()
                            .join(" ")
                    ),
                )
                .await
                .map_err(crate::model::error::Error::Serenity);
        } else {
            for cmd in crate::model::command::command_list().await {
                if cmd.name == args.0[0].to_lowercase() {
                    return message.channel_id.say(&ctx, format!("Command help for `{}`\nAlias: `{}`\nPart of module: `{}`\nDescription: {}\nUsage: `{} {}`\n*This command can be used every {} seconds*", cmd.name, cmd.shorthand, cmd.module.info().name, cmd.description, cmd.name, cmd.usage, cmd.cooldown_length))
                        .await
                        .map_err(crate::model::error::Error::Serenity);
                }
            }
        }

        Err(crate::model::error::Error::CommandNotFound)
    }
}
