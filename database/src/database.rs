//use sqlx::pool::PoolOptions;
use sqlx::{pool, mysql, mysql::MySqlPoolOptions};                                                    
use std::env;

//sqlx_core::pool::Pool<sqlx_mysql::database::MySql>

pub async fn newDB() -> pool::Pool<mysql::MySql> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    pool
}
