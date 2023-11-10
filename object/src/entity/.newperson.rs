pub struct Person<'a> {
    name: &'a str,
    age: u8
}

impl<'a> Person<'a> {
    pub fn new(name: &str, age: u8) -> Person{
        Person{name,age}
    }
    pub fn greeting(&self){
        println!("Helloo {}!", self.name);
    }
    pub fn get_name(&self) -> &str{
        self.name
    }
    pub fn get_age(&self) -> u8{
        self.age
    }

    pub fn set_age(&mut self, age: u8){
        self.age = age;
    }
    pub fn set_name(&mut self, name: &'a str){
        self.name = name;
    }
    
}

