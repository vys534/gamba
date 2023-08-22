use serenity::model::prelude::UserId;
use sqlx::{pool::PoolConnection, Row, Sqlite};

pub const DAILY_STREAK_BONUS_SCALE: [i64; 7] = [50, 60, 75, 90, 110, 130, 150];
pub const DAILY_STREAK_AMOUNT: i64 = 100;
pub const DAY_SECONDS: i64 = 86400;

// Ok() will return (prev. daily streak, new daily streak, best record)
// todo: reset time as an actual time of the day, not based on individual claim times
pub async fn handle(
    author: &UserId,
    db_conn: &mut PoolConnection<Sqlite>,
) -> Result<(i64, i64, i64), crate::model::error::Error> {
    let q_1 = sqlx::query(
        "SELECT daily_last_claimed, daily_streak, daily_streak_best FROM users WHERE user_id = ?",
    )
    .bind(author.0 as i64)
    .fetch_one(db_conn.as_mut())
    .await
    .map_err(crate::model::error::Error::Db)?;
    let daily_last_claimed: i64 = q_1
        .try_get("daily_last_claimed")
        .map_err(crate::model::error::Error::Db)?;
    let now = chrono::Utc::now().timestamp();
    if daily_last_claimed + DAY_SECONDS > now {
        return Err(crate::model::error::Error::DailyClaimed(
            (daily_last_claimed + DAY_SECONDS) - now,
        ));
    }

    let daily_streak: i64 = q_1
        .try_get("daily_streak")
        .map_err(crate::model::error::Error::Db)?;
    let daily_streak_best: i64 = q_1
        .try_get("daily_streak_best")
        .map_err(crate::model::error::Error::Db)?;

    let new_daily_streak = if daily_last_claimed + (DAY_SECONDS * 2) < now {
        1
    } else {
        daily_streak + 1
    };
    if new_daily_streak > daily_streak_best {
        sqlx::query("UPDATE users SET daily_streak_best = daily_streak_best + 1 WHERE user_id = ?")
            .bind(author.0 as i64)
            .execute(db_conn.as_mut())
            .await
            .map_err(crate::model::error::Error::Db)?;
    }

    sqlx::query("UPDATE users SET daily_last_claimed = ?, daily_streak = ? WHERE user_id = ?")
        .bind(now)
        .bind(new_daily_streak)
        .bind(author.0 as i64)
        .execute(db_conn.as_mut())
        .await
        .map_err(crate::model::error::Error::Db)?;

    let credits_earned = DAILY_STREAK_AMOUNT
        + DAILY_STREAK_BONUS_SCALE[usize::try_from(if new_daily_streak >= 7 {
            6
        } else {
            new_daily_streak - 1
        })
        .map_err(|_e| crate::model::error::Error::ParseInt)?];
    crate::db::currency::change_by(author.0, credits_earned, db_conn).await?;
    crate::db::log::currency_change(
        author.0,
        credits_earned,
        super::log::LogType::Claim,
        db_conn,
    )
    .await?;
    Ok((
        daily_streak,
        new_daily_streak,
        if new_daily_streak > daily_streak_best {
            new_daily_streak
        } else {
            daily_streak_best
        },
    ))
}
