use sqlx::{pool::PoolConnection, Sqlite};

pub enum LogType {
    Claim,
    Passive,
    Gamble,
    Manual,
}

pub async fn currency_change(
    author: u64,
    by: i64,
    t: LogType,
    db_conn: &mut PoolConnection<Sqlite>,
) -> Result<(), crate::model::error::Error> {
    sqlx::query("INSERT INTO log (user_id, ts, type, by) VALUES (?, ?, ?, ?)")
        .bind(author as i64)
        .bind(chrono::Utc::now().timestamp())
        .bind(match t {
            LogType::Claim => 0,
            LogType::Passive => 1,
            LogType::Gamble => 2,
            LogType::Manual => 99,
        })
        .bind(by)
        .execute(db_conn.as_mut())
        .await
        .map_err(crate::model::error::Error::Db)?;
    Ok(())
}
