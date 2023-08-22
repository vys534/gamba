use crate::util::handle_error;
use serenity::{
    model::prelude::Message,
    prelude::{Context, EventHandler},
};

pub struct Handler {
    pub db: sqlx::SqlitePool,
    pub redis: redis::aio::ConnectionManager,
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, m: Message) {
        if m.author.bot {
            return;
        }

        let mut db_conn = match crate::db::connection::get_sqlite_connection(&self.db).await {
            Ok(c) => c,
            Err(e) => {
                handle_error(ctx, m, e, false).await;
                return;
            }
        };

        if let Err(e) =
            crate::db::passive::handle(&m.author, self.redis.clone(), &mut db_conn).await
        {
            handle_error(ctx, m, e, false).await;
            return;
        }

        let m_split: Vec<&str> = m.content.split_whitespace().collect();

        if m_split.is_empty()
            || !m_split[0].starts_with(crate::PREFIX)
            || m_split[0].len() <= crate::PREFIX.len()
        {
            return;
        }

        let cmd_name = &m_split[0][crate::PREFIX.len()..m_split[0].len()];
        let args = crate::util::parse_args(m_split);
        let redis_conn = self.redis.clone();
        for c_test in crate::model::command::command_list().await {
            if cmd_name == c_test.name || cmd_name == c_test.shorthand {
                if let Err(e) = c_test.inner.exec(&ctx, &m, db_conn, redis_conn, args).await {
                    handle_error(ctx, m, e, true).await;
                }
                break;
            }
        }
    }
}
