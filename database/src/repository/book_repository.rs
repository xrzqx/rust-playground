use sqlx::{mysql::MySql, pool::Pool};
use crate::entity::Book;
use sqlx_mysql;

/*
pub trait BookRepository{
    fn new(pool: &Pool<MySql>) -> BookRepositoryImpl;
    //fn save(&self, book: &Book) -> impl Future<Output = ()>;
    async fn save(&self, book: &Book);
}
*/

#[derive(Debug)]
pub struct BookRepositoryImpl<'a> {
    pool: &'a Pool<MySql>,
}

impl<'a> BookRepositoryImpl<'a> {
    pub fn new(pool: &Pool<MySql>) -> BookRepositoryImpl{
        BookRepositoryImpl{pool}
    }
    pub async fn save(&self, book: &Book<'a>) -> u64 {
        let book = sqlx::query!(
            r#"
                INSERT INTO book (name)
                VALUES (?)
            "#,
            book.get_name())
            .execute(self.pool)
            .await.unwrap()
            .rows_affected();
        book

    }
    
    pub async fn find_all(&self) -> Vec<sqlx_mysql::MySqlRow>{
        let rows = sqlx::query("SELECT * FROM book")
            .fetch_all(self.pool)
            .await
            .unwrap();
        rows
    }

    pub async fn update_name(&self,name: &str, id: u32) -> u64{
        let update_name = sqlx::query!(
                r#"
                    UPDATE book SET name = ? WHERE id = ?
                "#,
                name,
                id
            )
            .execute(self.pool)
            .await
            .unwrap()
            .rows_affected();
        update_name
    }
    
}
