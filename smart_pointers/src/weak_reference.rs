/*
    The reference counting smart pointer, opposed to the strong count of references, also keeps track of the weak count of references.
    A weak reference is a reference that does not contribute to ownership and thus to the strong count.
    Instead, it contributes to the weak count.
    A reference counting smart pointer is dropped when only the strong count reaches 0.
    The weak reference count does not matter in that case.
    So when accessing the value behind the pointer through a weak reference, the value might already be dropped.
    This is why, when accessing through weak references, the developer has to check if the value still exists.
*/

use std::rc::{Rc, Weak};

pub fn run() {
    println!("=== Weak Reference ===");
    let weak_reference = weak_reference();
    println!("Reached main function");
    println!("Trying to unpack weak reference");
    if let Some(number) = weak_reference.upgrade() {
        println!("Number: {}", number);
        println!("Strong references: {}", Rc::strong_count(&number));
        println!("Weak references: {}", Rc::weak_count(&number));
    } else {
        println!("Error: Number has already been dropped because strong count reached 0");
    }
}

struct NumberRc {
    number: Rc<i32>,
}

impl NumberRc {
    fn new(number: i32) -> Self {
        Self {
            number: Rc::new(number),
        }
    }
}

fn weak_reference() -> Weak<i32> {
    println!("Creating new reference counting pointer holding number 42");
    let number_rc = NumberRc::new(42);

    println!("Creating a weak reference to the number");
    let weak_reference = Rc::downgrade(&number_rc.number);

    println!("Strong references: {}", Rc::strong_count(&number_rc.number));
    println!("Weak references: {}", Rc::weak_count(&number_rc.number));

    println!("Unpacking weak reference");
    if let Some(number) = weak_reference.upgrade() {
        println!("Number: {}", number);
        println!("Strong references: {}", Rc::strong_count(&number_rc.number));
        println!("Weak references: {}", Rc::weak_count(&number_rc.number));
        println!("Unpacking scope ends");
    } else {
        println!("Error: Number has already been dropped because strong count reached 0");
    }

    println!("Strong references: {}", Rc::strong_count(&number_rc.number));
    println!("Weak references: {}", Rc::weak_count(&number_rc.number));

    println!("Returning weak reference to main function");
    Rc::downgrade(&number_rc.number)
}
