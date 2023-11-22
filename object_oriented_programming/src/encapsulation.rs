/*
    Another detail of object-oriented programming is encapsulation, which is the idea that data
    can be hidden from other code. This is typically done by marking fields as private.
    Said data can only be accessed by using the object's public interface.
    This ensures that data is only modified in a way that is intended by the author of the object.
    Said API can also be used to hook into the object's behavior, such as logging when a field is modified, or sending messages to channels.

    Rust's structs and enums can also achieve encapsulation.
    If encapsulation is meant to be object-oriented, then Rust is object-oriented in this regard.
*/

use std::fmt::Display;

pub struct Person {
    name: String,
    age: u8,
}

impl Person {
    pub fn new(name: &str, age: u8) -> Person {
        Person {
            name: name.to_string(),
            age,
        }
    }

    pub fn age(&self) -> u8 {
        self.age
    }

    pub fn say_hello(&self) {
        println!("{}: I am {} years old.", self.name, self.age);
    }

    pub fn age_one_year(&mut self) {
        self.age += 1;
        println!("DEBUG: {} aged 1 year.", self.name);
    }
}

struct AveragedCollection {
    items: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            items: Vec::new(),
            average: f64::NAN,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    pub fn push(&mut self, value: i32) {
        self.items.push(value);
        self.update_average();
    }

    pub fn pop(&mut self) -> Option<i32> {
        let result = self.items.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    // Note that this method is private, so it can only be called from within the struct.
    fn update_average(&mut self) {
        let total: i32 = self.items.iter().sum();
        let count = self.items.len();
        self.average = total as f64 / count as f64;
    }
}

impl Display for AveragedCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Elements: {:?}; Average: {}", self.items, self.average)
    }
}

// Image the following code is in another scope where we don't have access to the struct's fields.
pub fn run() {
    println!("=== Person ===");
    let mut john = Person::new("John", 32);
    let _age: u8 = john.age(); // Even though we have a mutable reference to the instance, we can only read the age through the public API.
    john.say_hello();
    john.age_one_year(); // To change the age, we must use the public API of the instance. This also notifies the instance that it has aged.
    john.say_hello();

    println!();

    println!("=== Averaged Collection ===");
    let mut collection = AveragedCollection::new();
    let _average = collection.average(); // Same here, we can only read the average through the public API.

    println!("{}", collection);

    for i in 1..=10 {
        collection.push(i);
        println!("{}", collection);
    }

    for _ in 1..=10 {
        collection.pop();
        println!("{}", collection);
    }
}

/*
    A benefit of this is that, by hiding the underlying architecture of the object, we can change it without breaking the code that uses it.
    As long as the public API remains the same, meaning the method signatures don't change, the code that uses the object will still work.
    On the other hand, if the object's fields were public, and the code that uses it accessed those fields directly, then changing the fields would break the code that uses it.
*/
