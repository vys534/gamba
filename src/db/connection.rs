pub async fn get_sqlite_connection(
    p: &sqlx::pool::Pool<sqlx::sqlite::Sqlite>,
) -> Result<sqlx::pool::PoolConnection<sqlx::sqlite::Sqlite>, crate::model::error::Error> {
    p.acquire().await.map_err(crate::model::error::Error::Db)
}
