use serenity::model::prelude::UserId;
use sqlx::{pool::PoolConnection, Row, Sqlite};

pub const DAY_SECONDS: i64 = 86400;
pub const DAILY_STREAK_BONUS_SCALE: [i64; 7] = [20, 30, 40, 50, 60, 80, 100];
pub const DAILY_STREAK_AMOUNT: i64 = 100;
pub const CURRENCY_MAX: i64 = 1_000_000_000_000_000;

// Ok() will return (prev. daily streak, new daily streak, best record)
pub async fn process_daily_claim(
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
    adjust_currency_amount(author.0, credits_earned, db_conn).await?;
    crate::db::log::log_currency_change(
        author.0,
        0,
        credits_earned,
        super::log::LogType::Daily,
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

pub async fn get_current(
    author: &UserId,
    db_conn: &mut PoolConnection<Sqlite>,
) -> Result<i64, crate::model::error::Error> {
    let q = sqlx::query("SELECT currency_held FROM users WHERE user_id = ?")
        .bind(author.0 as i64)
        .fetch_one(db_conn.as_mut())
        .await
        .map_err(crate::model::error::Error::Db)?;
    let currency_held: i64 = q
        .try_get("currency_held")
        .map_err(crate::model::error::Error::Db)?;
    Ok(currency_held)
}

pub async fn adjust_currency_amount(
    author: u64,
    by: i64,
    db_conn: &mut PoolConnection<Sqlite>,
) -> Result<(), crate::model::error::Error> {
    let c_query = sqlx::query("SELECT currency_best, currency_held FROM users WHERE user_id = ?")
        .bind(author as i64)
        .fetch_one(db_conn.as_mut())
        .await
        .map_err(crate::model::error::Error::Db)?;
    let currency_best: i64 = c_query
        .try_get("currency_best")
        .map_err(crate::model::error::Error::Db)?;
    let currency_held: i64 = c_query
        .try_get("currency_held")
        .map_err(crate::model::error::Error::Db)?;
    if currency_held + by > currency_best {
        sqlx::query("UPDATE users SET currency_best = ? WHERE user_id = ?")
            .bind(if currency_held + by >= CURRENCY_MAX {
                CURRENCY_MAX
            } else {
                currency_held + by
            })
            .bind(author as i64)
            .execute(db_conn.as_mut())
            .await
            .map_err(crate::model::error::Error::Db)?;
    }
    sqlx::query("UPDATE users SET currency_held = currency_held + ? WHERE user_id = ?")
        .bind(if currency_held + by >= CURRENCY_MAX {
            CURRENCY_MAX
        } else {
            by
        })
        .bind(author as i64)
        .execute(db_conn.as_mut())
        .await
        .map_err(crate::model::error::Error::Db)?;
    Ok(())
}
