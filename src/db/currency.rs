use serenity::model::prelude::UserId;
use sqlx::{pool::PoolConnection, Row, Sqlite};

pub const CURRENCY_MAX: i64 = 1_000_000_000_000_000;

pub async fn current(
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

pub async fn change_by(
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
