trait Book<'a>{
    fn new(name: &str) -> BookImpl;
    fn get_name(&self) -> &str;
    fn set_name(&mut self, name: &'a str);
    fn delete(&mut self);
}

#[derive(Debug)]
struct BookImpl<'a>{
    name: &'a str,
}

impl<'a> Book<'a> for BookImpl<'a>{
    fn new(name: &str) -> BookImpl{
        BookImpl{name}
    }
    fn get_name(&self) -> &str{
        self.name
    }
    fn set_name(&mut self, name: &'a str){
        self.name = name
    }
    fn delete(&mut self){
        println!("delete on book object");
    }

}

trait BookRepository{
    fn new(name: &str) -> BookRepositoryImpl;
    fn save(&self, book: &BookImpl);
    fn update(&self, book: &BookImpl);
    fn delete(&self, book: &mut BookImpl);
}

struct BookRepositoryImpl<'a>{
    dbpool: &'a str,
}

impl<'a> BookRepository for BookRepositoryImpl<'a>{
    fn new(dbpool: &str) -> BookRepositoryImpl{
        BookRepositoryImpl{dbpool}
    }
    fn save(&self, book: &BookImpl){
        println!("Save this book with name: {} in database", book.get_name());
    }
    fn update(&self, book: &BookImpl){
        println!("Update this book with name: {} in database", book.get_name());
    }
    fn delete(&self, book: &mut BookImpl){
        println!("Delete this book with name: {} in database", book.get_name());
        book.delete();
    }
}


fn main() {
    let mut book: BookImpl = <BookImpl as Book>::new("Book Name");
    println!("{}",book.get_name());


    let book_repository: BookRepositoryImpl = <BookRepositoryImpl as BookRepository>::new("mysql/xrzqx");
    println!("{}",book_repository.dbpool);

    book_repository.save(&book);
    book_repository.update(&book);
    book_repository.delete(&mut book);


}
