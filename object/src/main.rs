mod entity;

use entity::Person;
use entity::NewPerson;

use std::mem;

fn main() {
    let mut person = Person::new("xrzqx", 23);
    person.greeting();
    println!("{} {} years old", person.get_name(), person.get_age());
    person.set_name("xrzqx_new");
    person.set_age(18);
    println!("Updated Person: {} {} years old", person.get_name(), person.get_age());

    let size_of_my_variable_refrence = mem::size_of_val(&person);
    println!("Size of Refrence lifetime Entity: {} bytes", size_of_my_variable_refrence);

    let person2 = NewPerson::new(String::from("xrzqx_new"), 18);

    let size_of_my_variable = mem::size_of_val(&person2);
    println!("Size of String Entity: {} bytes", size_of_my_variable);
}
