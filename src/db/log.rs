use sqlx::{pool::PoolConnection, Sqlite};

pub enum LogType {
    Daily,
    Talk,
    Gamble,
}

pub async fn log_currency_change(
    author: u64,
    in_credits: i64,
    out_credits: i64,
    t: LogType,
    db_conn: &mut PoolConnection<Sqlite>,
) -> Result<(), crate::model::error::Error> {
    sqlx::query("INSERT INTO log (user_id, ts, currency_in, currency_out, is_daily, is_talk) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(author as i64)
        .bind(chrono::Utc::now().timestamp())
        .bind(in_credits)
        .bind(out_credits)
        .bind(match t { LogType::Daily => 1, _ => 0 })
        .bind(match t { LogType::Talk => 1, _ => 0 })
        .execute(db_conn.as_mut())
        .await
        .map_err(crate::model::error::Error::Db)?;
    Ok(())
}
