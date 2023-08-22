use sqlx::{Pool, Sqlite};

pub mod claim;
pub mod connection;
pub mod currency;
pub mod leaderboard;
pub mod log;
pub mod passive;

pub async fn setup(db: &mut Pool<Sqlite>) -> Result<(), crate::model::error::Error> {
    let mut db_conn = crate::db::connection::get_sqlite_connection(db).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
        user_id INTEGER PRIMARY KEY,
        cached_username TEXT NOT NULL,
        cached_discriminator INTEGER NOT NULL,
        ts INTEGER NOT NULL,
        currency_held INTEGER NOT NULL,
        currency_best INTEGER NOT NULL,
        daily_last_claimed INTEGER NOT NULL,
        daily_streak INTEGER NOT NULL,
        daily_streak_best INTEGER NOT NULL
    )",
    )
    .execute(db_conn.as_mut())
    .await
    .map_err(crate::model::error::Error::Db)?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS log (
        user_id INTEGER NOT NULL,
        ts INTEGER NOT NULL,
        type INTEGER NOT NULL,
        by INTEGER NOT NULL
    )",
    )
    .execute(db_conn.as_mut())
    .await
    .map_err(crate::model::error::Error::Db)?;

    Ok(())
}
