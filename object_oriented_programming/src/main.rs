/*
    Today, there is no clear, single definition of object-oriented programming.
    However, depending on the definition used, some of the traits of a object-oriented language apply to Rust, and some do not.
    In this sub-directory, we will explore how object-oriented programming concepts translate to Rust.
*/

mod encapsulation;
mod inheritance;
mod objects;

fn main() {
    println!("===== Objects =====");
    objects::run();

    println!();

    println!("===== Encapsulation =====");
    encapsulation::run();

    println!();

    println!("===== Inheritance =====");
    inheritance::run();
}
