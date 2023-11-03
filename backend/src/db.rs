use sqlx::{MySql, MySqlPool, Pool};

pub async fn db_pool() -> Pool<MySql> {
    MySqlPool::connect(std::env::var("DATABASE_URL").unwrap().as_str()).await?
}