use std::collections::HashMap;

use serenity::model::prelude::Message;

use crate::{
    db::leaderboard::{LeaderboardRow, LeaderboardType},
    model::error::Error,
};

pub async fn handle_error(
    ctx: serenity::prelude::Context,
    m: Message,
    e: Error,
    from_command: bool,
) {
    // drop sending a response if we had no permission to send to that channel
    if let Error::Serenity(serenity::Error::Model(
        serenity::model::error::Error::InvalidPermissions(_perm),
    )) = &e
    {
        return;
    }
    let error_msg = match e.is_unexpected() {
        true => format!(
            "{} something went wrong internally: {}",
            ":warning:",
            e.message()
        ),
        false => format!(
            "{} {}",
            crate::model::emoji::Emoji::EpicFail.to_string(),
            e.message()
        ),
    };
    if !from_command {
        // if the error wasn't from issuing a command,
        // do not send anything to either a) prevent flooding chat or b) not confuse the user
        println!(
            "handled error from message event without running a command: {}",
            e.message()
        );
        return;
    }
    let s = m.channel_id.say(&ctx, error_msg).await;
    if let Err(err) = s {
        println!("Error sending error message: {}", err);
    }
}

pub fn format_currency(amount: i64) -> String {
    format!(
        "**{}** {}",
        amount,
        crate::model::emoji::Emoji::Cp.to_string()
    )
}

pub fn format_streak(amount: i64) -> String {
    format!(
        "**{}** {}",
        amount,
        crate::model::emoji::Emoji::Fire.to_string()
    )
}

pub fn parse_args(args: Vec<&str>) -> (Vec<&str>, HashMap<String, &str>) {
    let mut m: HashMap<String, &str> = HashMap::new();
    let mut args_copy = args.clone();

    for (i, arg) in args.iter().enumerate() {
        // we skip over the part of the message with the command name
        if i == 0 {
            args_copy.remove(i);
            continue;
        }
        if arg.starts_with('-') && arg.len() >= 2 && args.len() > i + 1 {
            m.insert(args[i][1..].to_ascii_lowercase(), args[i + 1]);
            args_copy.remove(i);
            args_copy.remove(i + 1);
        }
    }

    (args_copy, m)
}

pub fn format_lb_entry(
    row: &LeaderboardRow,
    position: i64,
    t: &LeaderboardType,
    is_self: bool,
) -> String {
    format!(
        "{} | {} - {}",
        if is_self {
            format!("You (#{})", position)
        } else {
            format!("`#{}`", position)
        },
        format_username(&row.user_name, row.discriminator),
        match t {
            LeaderboardType::CurrencyBest | LeaderboardType::CurrencyHeld =>
                format_currency(row.value),
            LeaderboardType::StreakBest => format_streak(row.value),
        }
    )
}

pub fn format_username(name: &str, discriminator: u16) -> String {
    if discriminator == 0 {
        return name.to_string();
    }
    format!("{}#{}", name, discriminator)
}
