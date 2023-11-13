extern crate dotenv;
use dotenv::dotenv;
use sqlx::{mysql::MySqlPoolOptions, Executor, MySqlConnection, Row};
//use sqlx::mysql::MySqlPool;
use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Hello, world!");

    dotenv().ok();

    println!("{}", env::var("DATABASE_URL").unwrap());

    // let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await;

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    assert_eq!(
        "mysql://wsl:password@localhost/rust_test",
        &env::var("DATABASE_URL").unwrap()
    );

    println!("{:?}", pool);

    
    let book = sqlx::query!(
        r#"
            INSERT INTO book ( name )
            VALUES ( ? )
        "#,
        "TEST"
    )
    .execute(&pool)
    .await;

    let update_book = sqlx::query!(
            r#"
                UPDATE book SET name = ? WHERE id = ?
            "#,
            "TEST_NEW",
            100
        )
        .execute(&pool)
        .await.unwrap()
        .rows_affected()
        ;

    if (update_book < 0 ){
        println!("Updated !!");
    }
    else {
        println!("invalid id");
    }



    let query = "SELECT * FROM book";

    let rows = sqlx::query(query).fetch_all(&pool).await.unwrap();
    
    for row in rows {
        let id: i32 = row.get("id");
        let name: String = row.get("name");
        println!("ID: {}, Name: {}", id, name);
    }


    

    
}
