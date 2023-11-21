pub struct Book<'a>{
    name : &'a str
}


impl<'a> Book<'a>{
    pub fn new(name: &str) -> Book{
        Book {name}
    }
    pub fn set_name(&mut self, name: &'a str) {
        self.name = name;
    }
    pub fn get_name(&self) -> &str{
        self.name
    }
}

