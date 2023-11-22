/*
    You would choose inheritance for two main reasons:
     - Ruse of code: You can implement particular behavior for one type, and inheritance enables you to reuse that implementation for a different type.
     - As a type system: You want to enable a child type to be used in the same places as the parent type.
*/

mod for_polymorphism;
mod for_polymorphism_better_example;
mod for_reuse_of_code;

pub fn run() {
    println!("=== Reuse of Code ===");
    for_reuse_of_code::run();

    println!();

    println!("=== Polymorphism ===");
    for_polymorphism::run();

    println!();

    println!("=== Polymorphism (Better Example) ===");
    for_polymorphism_better_example::run();
}
