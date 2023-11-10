pub struct NewPerson {
    name: String,
    age: u8
}

impl NewPerson {
    pub fn new(name: String, age: u8) -> NewPerson{
        NewPerson{name,age}
    }
    pub fn greeting(&self){
        println!("Helloo {}!", self.name);
    }
    /*
    pub fn get_name(&self) -> String{
        self.name
    }
    pub fn get_age(&self) -> u8{
        self.age
    }

    pub fn set_age(&mut self, age: u8){
        self.age = age;
    }
    pub fn set_name(&mut self, name: String){
        self.name = name;
    }
    */
    
}

