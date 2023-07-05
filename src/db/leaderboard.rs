use serenity::{
    futures::{Stream, TryStreamExt},
    model::prelude::UserId,
};
use sqlx::{pool::PoolConnection, sqlite::SqliteRow, Row, Sqlite};

pub enum LeaderboardType {
    CurrencyBest,
    CurrencyHeld,
    StreakBest,
}

impl LeaderboardType {
    fn to_str(&self) -> &str {
        match self {
            LeaderboardType::CurrencyBest => "currency_best",
            LeaderboardType::CurrencyHeld => "currency_held",
            LeaderboardType::StreakBest => "daily_streak_best",
        }
    }
}

pub struct LeaderboardRow {
    pub user_id: i64,
    pub user_name: String,
    pub discriminator: u16,
    pub value: i64,
}

const LEADERBOARD_MAX_ENTRIES: i64 = 10;
const MAX_PAGES: i64 = 100;

pub async fn get_leaderboard(
    db_conn: &mut PoolConnection<Sqlite>,
    mut page: i64,
    by_type: &LeaderboardType,
    author: &UserId,
) -> Result<((i64, LeaderboardRow), Vec<LeaderboardRow>), crate::model::error::Error> {
    if page > MAX_PAGES {
        page = MAX_PAGES;
    }
    let mut lb_items: Vec<LeaderboardRow> = Vec::new();
    let sp_q = format!(
        "SELECT COUNT(*) FROM users WHERE {} >= (SELECT {} FROM users WHERE user_id = ?)",
        by_type.to_str(),
        by_type.to_str()
    );
    let self_position = sqlx::query(&sp_q)
        .bind(author.0 as i64)
        .try_map(
            |row: SqliteRow| -> std::result::Result<u32, sqlx::error::Error> {
                let v: u32 = row.try_get("COUNT(*)")?;
                Ok(v)
            },
        )
        .fetch_one(db_conn.as_mut())
        .await
        .map_err(crate::model::error::Error::Db)?;

    let sr_q = format!(
        "SELECT {}, user_id, cached_username, cached_discriminator FROM users WHERE user_id = ?",
        by_type.to_str()
    );
    let self_row = sqlx::query(&sr_q)
        .bind(author.0 as i64)
        .try_map(
            |row: SqliteRow| -> std::result::Result<LeaderboardRow, sqlx::error::Error> {
                Ok(LeaderboardRow {
                    user_id: row.try_get("user_id")?,
                    value: row.try_get(by_type.to_str())?,
                    user_name: row.try_get("cached_username")?,
                    discriminator: row.try_get("cached_discriminator")?,
                })
            },
        )
        .fetch_one(db_conn.as_mut())
        .await
        .map_err(crate::model::error::Error::Db)?;

    let r_q = format!("SELECT {}, user_id, cached_username, cached_discriminator FROM users ORDER BY {} DESC LIMIT ? OFFSET ?", by_type.to_str(), by_type.to_str());
    let mut rows: std::pin::Pin<
        Box<dyn Stream<Item = Result<LeaderboardRow, sqlx::Error>> + Send>,
    > = sqlx::query(&r_q)
        .bind(LEADERBOARD_MAX_ENTRIES)
        .bind(if page < 0 { 0 } else { page } * LEADERBOARD_MAX_ENTRIES)
        .try_map(
            |row: SqliteRow| -> std::result::Result<LeaderboardRow, sqlx::error::Error> {
                let r = LeaderboardRow {
                    user_id: row.try_get("user_id")?,
                    value: row.try_get(by_type.to_str())?,
                    user_name: row.try_get("cached_username")?,
                    discriminator: row.try_get("cached_discriminator")?,
                };
                Ok(r)
            },
        )
        .fetch(db_conn.as_mut());

    while let Some(row) = rows
        .try_next()
        .await
        .map_err(crate::model::error::Error::Db)?
    {
        lb_items.push(row);
    }

    Ok(((self_position as i64, self_row), lb_items))
}
