extern crate dotenv;
mod repository;
mod entity;
use entity::Book;
//use repository::{BookRepository,BookRepositoryImpl};
use repository::BookRepositoryImpl;
use dotenv::dotenv;
use sqlx::{ Row};
use std::env;
mod database;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Hello, world!");

    dotenv().ok();

    println!("{}", &env::var("DATABASE_URL").unwrap());

    let pool = database::new_db().await.unwrap();
    let book_repo: BookRepositoryImpl = BookRepositoryImpl::new(&pool);
    println!("{:?}",book_repo);

    let book = Book::new("new object");
    let add_book = book_repo.save(&book).await;

    if add_book < 0 {
        println!("Added new book to database");
    }else {
        println!("Fail");
    }

    let update_book = book_repo.update_name("Update object", 1).await;

    if update_book < 0 {
        println!("Updated !!");
    } else {
        println!("invalid id");
    }

    let rows = book_repo.find_all().await;
    //println!("{rows:?}");
    
    for row in rows {
        let id: i32 = row.get("id");
        let name: String = row.get("name");
        println!("ID: {}, Name: {}", id, name);
    }
    
    
    
    
    

}
