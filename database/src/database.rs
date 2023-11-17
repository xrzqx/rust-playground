use sqlx::{mysql, mysql::MySqlPoolOptions, pool};
use std::env;

pub async fn new_db() -> Result< pool::Pool<mysql::MySql>, sqlx::Error >  {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").unwrap())
        .await;

    match pool {
        Ok(e) => Ok(e),
        Err(e) => Err(e),
    }
}
