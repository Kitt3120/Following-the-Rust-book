/*
    According to the book "Design Patterns: Elements of Reusable Object-Oriented Software" by Erich Gamma et al. (1994),
    object-orientated programming is defined as follows:
    "Object-oriented programs are made up of objects.
     An object packages both data and the procedures that operate on that data.
     The procedures are typically called methods or operations."

    In Rust, we can define a struct that contains data and provide methods that operate on that data using impl blocks.
    Thus, Rust is object-oriented according to this definition. Structs and enums provide the same functionality as objects in other languages.
*/

struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: &str, age: u8) -> Person {
        Person {
            name: name.to_string(),
            age,
        }
    }

    fn say_hello(&self) {
        println!("{}: I am {} years old.", self.name, self.age);
    }
}

pub fn run() {
    let john = Person::new("John", 32);
    john.say_hello();
}
