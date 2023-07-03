use rand::Rng;
use serenity::model::user::User;
use sqlx::{pool::PoolConnection, Row, Sqlite};

pub const MIN_CURRENCY_EARNED: usize = 8;
pub const MAX_CURRENCY_EARNED: usize = 10;
pub const TALK_INTERVAL: usize = 60;

pub async fn handle(
    author: &User,
    mut redis_conn: redis::aio::ConnectionManager,
    db_conn: &mut PoolConnection<Sqlite>,
) -> Result<(), crate::model::error::Error> {
    let k = format!("{}:{}", author.id.0, "talk");
    let is_currency_cooldown_active = redis::Cmd::get(&k)
        .query_async::<_, Option<String>>(&mut redis_conn)
        .await
        .map_err(crate::model::error::Error::Redis)?;

    if is_currency_cooldown_active.is_none() {
        let row = sqlx::query("SELECT COUNT(*) FROM users WHERE user_id = ?")
            .bind(author.id.0 as i64)
            .fetch_one(db_conn.as_mut())
            .await
            .map_err(crate::model::error::Error::Db)?;

        let ct: u32 = row
            .try_get("COUNT(*)")
            .map_err(crate::model::error::Error::Db)?;

        if ct == 0 {
            // Insert
            sqlx::query("INSERT INTO users (user_id, cached_username, cached_discriminator, ts, currency_held, currency_best, daily_last_claimed, daily_streak, daily_streak_best) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(author.id.0 as i64)
                .bind(&author.name)
                .bind(author.discriminator)
                .bind(chrono::Utc::now().timestamp())
                .bind(500)
                .bind(500)
                .bind(0)
                .bind(0)
                .bind(0)
                .execute(db_conn.as_mut())
                .await
                .map_err(crate::model::error::Error::Db)?;
        } else {
            sqlx::query(
                "UPDATE users SET cached_username = ?, cached_discriminator = ? WHERE user_id = ?",
            )
            .bind(&author.name)
            .bind(author.discriminator)
            .bind(author.id.0 as i64)
            .execute(db_conn.as_mut())
            .await
            .map_err(crate::model::error::Error::Db)?;
        }

        let social_credits_earned: usize =
            rand::thread_rng().gen_range(MIN_CURRENCY_EARNED..=MAX_CURRENCY_EARNED);
        crate::db::currency::adjust_currency_amount(
            author.id.0,
            social_credits_earned as i64,
            db_conn,
        )
        .await?;
        crate::db::log::log_currency_change(
            author.id.0,
            0,
            social_credits_earned as i64,
            super::log::LogType::Talk,
            db_conn,
        )
        .await?;

        redis::Cmd::set_ex(&k, String::new(), TALK_INTERVAL)
            .query_async(&mut redis_conn)
            .await
            .map_err(crate::model::error::Error::Redis)?;
    }

    Ok(())
}
