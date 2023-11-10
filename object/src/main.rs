mod entity;

use entity::Person;

fn main() {
    let mut person = Person::new("xrzqx", 23);
    person.greeting();
    println!("{} {} years old", person.get_name(), person.get_age());
    person.set_name("xrzqx_new");
    person.set_age(18);
    println!("Updated Person: {} {} years old", person.get_name(), person.get_age());
}
