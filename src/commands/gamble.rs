use std::collections::HashMap;

use crate::db::currency::adjust_currency_amount;
use crate::db::currency::get_current;
use crate::model::command;
use rand::Rng;
use serenity;
use sqlx::Sqlite;

pub struct GambleCommand;

pub const DEFAULT_AMOUNT: i64 = 10;
pub const MAX_AMOUNT: i64 = 10_000_000_000;

pub const SPENCER_ID: u64 = 439862794511450133;

static GAMBLE_WHEELS: [[crate::model::icons::Icon; 28]; 3] = [
    // [
    //     crate::model::icons::Icon::Moosh,
    //     crate::model::icons::Icon::Moosh,
    //     crate::model::icons::Icon::Fabio,
    //     crate::model::icons::Icon::Fabio,
    //     crate::model::icons::Icon::Reaper,
    //     crate::model::icons::Icon::Reaper,
    //     crate::model::icons::Icon::TheDollar,
    //     crate::model::icons::Icon::TheDollar,
    //     crate::model::icons::Icon::TheDollar,
    //     crate::model::icons::Icon::Troll,
    //     crate::model::icons::Icon::Troll,
    //     crate::model::icons::Icon::Troll,
    //     crate::model::icons::Icon::Troll,
    //     crate::model::icons::Icon::WhatsappSpencer,
    //     crate::model::icons::Icon::WhatsappSpencer,
    //     crate::model::icons::Icon::WhatsappSpencer,
    //     crate::model::icons::Icon::Whatsapp,
    //     crate::model::icons::Icon::Whatsapp,
    //     crate::model::icons::Icon::WhatsappGold,
    //     crate::model::icons::Icon::JointOLantern,
    //     crate::model::icons::Icon::JointOLantern,

    // ],
    [
        crate::model::icons::Icon::Whatsapp,
        crate::model::icons::Icon::Troll,
        crate::model::icons::Icon::WhatsappGold,
        crate::model::icons::Icon::Moosh,
        crate::model::icons::Icon::Moosh,
        crate::model::icons::Icon::JointOLantern,
        crate::model::icons::Icon::Reaper,
        crate::model::icons::Icon::WhatsappSpencer,
        crate::model::icons::Icon::Fabio,
        crate::model::icons::Icon::WhatsappSpencer,
        crate::model::icons::Icon::Reaper,
        crate::model::icons::Icon::Whatsapp,
        crate::model::icons::Icon::Fabio,
        crate::model::icons::Icon::Troll,
        crate::model::icons::Icon::Troll,
        crate::model::icons::Icon::Reaper,
        crate::model::icons::Icon::Troll,
        crate::model::icons::Icon::Fabio,
        crate::model::icons::Icon::WhatsappSpencer,
        crate::model::icons::Icon::TheDollar,
        crate::model::icons::Icon::TheDollar,
        crate::model::icons::Icon::Fabio,
        crate::model::icons::Icon::Fabio,
        crate::model::icons::Icon::Moosh,
        crate::model::icons::Icon::TheDollar,
        crate::model::icons::Icon::Reaper,
        crate::model::icons::Icon::JointOLantern,
        crate::model::icons::Icon::Reaper,
    ],
    [
        crate::model::icons::Icon::Reaper,
        crate::model::icons::Icon::Whatsapp,
        crate::model::icons::Icon::Moosh,
        crate::model::icons::Icon::Fabio,
        crate::model::icons::Icon::Troll,
        crate::model::icons::Icon::Moosh,
        crate::model::icons::Icon::Reaper,
        crate::model::icons::Icon::Whatsapp,
        crate::model::icons::Icon::Fabio,
        crate::model::icons::Icon::WhatsappSpencer,
        crate::model::icons::Icon::Troll,
        crate::model::icons::Icon::JointOLantern,
        crate::model::icons::Icon::JointOLantern,
        crate::model::icons::Icon::Reaper,
        crate::model::icons::Icon::Reaper,
        crate::model::icons::Icon::Fabio,
        crate::model::icons::Icon::WhatsappGold,
        crate::model::icons::Icon::TheDollar,
        crate::model::icons::Icon::Moosh,
        crate::model::icons::Icon::WhatsappSpencer,
        crate::model::icons::Icon::Troll,
        crate::model::icons::Icon::Reaper,
        crate::model::icons::Icon::Troll,
        crate::model::icons::Icon::Fabio,
        crate::model::icons::Icon::TheDollar,
        crate::model::icons::Icon::WhatsappSpencer,
        crate::model::icons::Icon::TheDollar,
        crate::model::icons::Icon::Fabio,
    ],
    [
        crate::model::icons::Icon::WhatsappSpencer,
        crate::model::icons::Icon::TheDollar,
        crate::model::icons::Icon::Reaper,
        crate::model::icons::Icon::WhatsappSpencer,
        crate::model::icons::Icon::Troll,
        crate::model::icons::Icon::Moosh,
        crate::model::icons::Icon::JointOLantern,
        crate::model::icons::Icon::Fabio,
        crate::model::icons::Icon::Fabio,
        crate::model::icons::Icon::Reaper,
        crate::model::icons::Icon::Troll,
        crate::model::icons::Icon::Fabio,
        crate::model::icons::Icon::Troll,
        crate::model::icons::Icon::Whatsapp,
        crate::model::icons::Icon::Reaper,
        crate::model::icons::Icon::Fabio,
        crate::model::icons::Icon::Reaper,
        crate::model::icons::Icon::Moosh,
        crate::model::icons::Icon::Moosh,
        crate::model::icons::Icon::Troll,
        crate::model::icons::Icon::WhatsappGold,
        crate::model::icons::Icon::Fabio,
        crate::model::icons::Icon::TheDollar,
        crate::model::icons::Icon::JointOLantern,
        crate::model::icons::Icon::Whatsapp,
        crate::model::icons::Icon::Reaper,
        crate::model::icons::Icon::WhatsappSpencer,
        crate::model::icons::Icon::TheDollar,
    ],
];

#[serenity::async_trait]
impl command::Command for GambleCommand {
    fn info(&self) -> command::CommandInfo {
        command::CommandInfo {
            module: crate::model::module::Class::GetItTwisted,
            name: "gamble",
            shorthand: "g",
            cooldown_length: 2,
            description: "get it twisted sir",
            usage: "[amount]",
            inner: Box::new(GambleCommand),
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
        let user_credits = get_current(&message.author.id, &mut db_conn).await?;

        let mut amount_to_gamble: i64 = if !args.0.is_empty() {
            if args.0[0] == "all" {
                user_credits
            } else {
                let suffix: char = args.0[0].chars().last().unwrap();
                let (multiplier, has_suffix) = match suffix {
                    'k' => (1000, true),
                    'm' => (1_000_000, true),
                    _ => (1, false),
                };
                let value = args.0[0]
                    .chars()
                    .take(args.0[0].len() - if has_suffix { 1 } else { 0 })
                    .collect::<String>();
                str::parse::<i64>(&value)
                    .map_err(|_e: std::num::ParseIntError| crate::model::error::Error::ParseInt)?
                    * multiplier
            }
        } else {
            DEFAULT_AMOUNT
        };
        if amount_to_gamble < DEFAULT_AMOUNT {
            amount_to_gamble = DEFAULT_AMOUNT;
        }
        if amount_to_gamble > MAX_AMOUNT {
            amount_to_gamble = MAX_AMOUNT;
        }
        if user_credits < amount_to_gamble {
            return Err(crate::model::error::Error::Broke(
                amount_to_gamble - user_credits,
            ));
        }
        let mut wheels: Vec<Vec<&crate::model::icons::Icon>> = Vec::new();
        for item in &GAMBLE_WHEELS {
            let mut wheel: Vec<&crate::model::icons::Icon> = Vec::new();
            let pos = rand::thread_rng().gen_range(0..item.len());
            for j in 0..5 {
                let pos_offset = pos + j;
                wheel.push(
                    &item[if pos_offset > item.len() - 1 {
                        pos_offset % (item.len() - 1)
                    } else {
                        pos_offset
                    }],
                )
            }
            wheels.push(wheel);
        }

        let payout_row = [wheels[0][2], wheels[1][2], wheels[2][2]];
        let mut payout_multiplier = 0.0;
        let mut payout_information: Vec<String> = Vec::new();
        let mut dollar_event = false;
        let mut spencer_event = false;
        if payout_row
            .iter()
            .all(|&item| item.name() == payout_row[0].name())
        {
            payout_multiplier += payout_row[0].match_data().1;
            payout_information.push(match payout_row[0] {
                crate::model::icons::Icon::TheDollar => {
                    dollar_event = true;
                    "the dollar".to_string()
                }
                crate::model::icons::Icon::WhatsappSpencer => {
                    spencer_event = true;
                    payout_multiplier += 1.0;
                    "3 spencer bonus: spencer stole all your winnings".to_string()
                }
                _ => {
                    format!(
                        "**MATCH 3 BONUS** {}: **{}**x",
                        payout_row[0].name(),
                        payout_row[0].match_data().1
                    )
                }
            })
        }

        if !dollar_event && !spencer_event {
            for p in payout_row {
                payout_multiplier += p.match_data().0;
                payout_information.push(format!("{}: {}x", p.name(), p.match_data().0))
            }
        }

        let mut casino_display: Vec<String> = Vec::new();
        for i in 0..5 {
            casino_display.push(format!(
                "{}{}{}",
                if i == 2 { "`>`" } else { "` `" },
                [wheels[0][i], wheels[1][i], wheels[2][i]]
                    .iter()
                    .map(|ic| ic.emoji().to_string())
                    .collect::<Vec<&str>>()
                    .join(""),
                if i == 2 { "`<`" } else { "` `" }
            ));
        }

        let original_payout = ((amount_to_gamble as f64) * payout_multiplier) as i64;
        if spencer_event {
            adjust_currency_amount(SPENCER_ID, original_payout, &mut db_conn).await?;
            crate::db::log::log_currency_change(
                SPENCER_ID,
                0,
                original_payout,
                crate::db::log::LogType::Gamble,
                &mut db_conn,
            )
            .await?;
        }
        let payout = if dollar_event {
            1
        } else if spencer_event {
            0
        } else {
            original_payout
        };

        let diff = payout - amount_to_gamble;
        adjust_currency_amount(message.author.id.0, diff, &mut db_conn).await?;
        crate::db::log::log_currency_change(
            message.author.id.0,
            amount_to_gamble,
            payout,
            crate::db::log::LogType::Gamble,
            &mut db_conn,
        )
        .await?;

        let new_balance = get_current(&message.author.id, &mut db_conn).await?;
        let modifier = if dollar_event {
            crate::model::emoji::Emoji::TheDollar
                .to_string()
                .to_string()
        } else if spencer_event {
            crate::model::emoji::Emoji::WhatsappSpencer
                .to_string()
                .to_string()
        } else {
            format!("x{:.2}", payout_multiplier)
        };
        let final_str = format!(
            "<@{}>\n{}\n---\n{}\n{} in >> {} >> {} | You {} __{}__ (now at {})",
            &message.author.id.0,
            casino_display.join("\n"),
            payout_information.join(", "),
            crate::util::format_currency(amount_to_gamble),
            modifier,
            crate::util::format_currency(payout),
            if diff >= 0 { "got" } else { "lost" },
            crate::util::format_currency(diff.abs()),
            crate::util::format_currency(new_balance),
        );

        message
            .channel_id
            .say(&ctx, final_str)
            .await
            .map_err(crate::model::error::Error::Serenity)
    }
}
