/*
    See unsafe_functions.rs first.
    When defining and implementing traits, you can declare unsafe methods.
    You can also declare the entire trait as unsafe.
    However, in the example below, the concept shown in safe_abstractions.rs is used to not cascade the unsafe keyword up to the trait.
*/

static mut DOG_COUNT: u32 = 0;

struct Dogger {
    name: String,
}

trait Dog {
    unsafe fn new(name: String) -> Self;
    fn bark(&self);
}

impl Dog for Dogger {
    unsafe fn new(name: String) -> Self {
        DOG_COUNT += 1;
        println!("{} dogs created", DOG_COUNT);

        Dogger { name }
    }

    fn bark(&self) {
        println!("{} barks", self.name);
    }
}

pub fn run() {
    print_dog_count();
    println!("Creating dog");
    let dog = unsafe { Dogger::new(String::from("Cooper")) };
    dog.bark();
    print_dog_count();

    println!();
    let other_dog = unsafe { Dogger::new(String::from("")) };
    other_dog.bark();
    print_dog_count();
}

fn print_dog_count() {
    let dog_count = unsafe { DOG_COUNT };

    if dog_count == 1 {
        println!("There is {} dog", dog_count);
    } else {
        println!("There are {} dogs", dog_count);
    }
}
