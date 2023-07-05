use crate::util::format_currency;

#[derive(Debug)]
pub enum Error {
    Redis(redis::RedisError),
    Db(sqlx::Error),
    Serenity(serenity::Error),
    Cooldown(i64),
    Broke(i64),
    DailyClaimed(i64),
    CommandNotFound,
    ParseInt,
    OutOfBounds,
    NoPermission,
    InvalidArguments,
}

impl Error {
    pub fn is_unexpected(&self) -> bool {
        !matches!(
            self,
            Error::Broke(_)
                | Error::Cooldown(_)
                | Error::Serenity(_)
                | Error::DailyClaimed(_)
                | Error::ParseInt
                | Error::CommandNotFound
                | Error::OutOfBounds
                | Error::NoPermission
                | Error::InvalidArguments
        )
    }

    pub fn message(&self) -> String {
        match self {
            Error::InvalidArguments => "invalid arguments".to_string(),
            Error::NoPermission => "my g you cant access that".to_string(),
            Error::Broke(by_amount) => format!(
                "sorry, but you are short {} to roll that amount",
                format_currency(*by_amount)
            ),
            Error::Cooldown(s_left) => {
                format!("wait {} more seconds before using that again", s_left)
            }
            Error::DailyClaimed(s_left) => {
                let d = std::time::Duration::new(*s_left as u64, 0);
                format!(
                    "wait {:0>2}:{:0>2}:{:0>2} to claim again",
                    (d.as_secs() / 60) / 60,
                    (d.as_secs() / 60) % 60,
                    d.as_secs() % 60
                )
            }
            Error::Redis(e) => format!("redis error: {}", e),
            Error::Db(e) => format!("database error: {}", e),
            Error::Serenity(e) => format!("serenity (client library) error: {}", e),
            Error::CommandNotFound => "command by that name was not found".to_string(),
            Error::ParseInt => "could not resolve that to a valid number".to_string(),
            Error::OutOfBounds => "that number is out of bounds".to_string(),
        }
    }
}
