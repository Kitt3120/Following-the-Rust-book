/*
    By nature, the Rust compiler is very strict and conservative.
    If it isn't sure whether something is safe, it will reject it, even if it were actually safe.
    This is a good thing, as it makes the compiler even more reliable and trustworthy.

    However, it can be a pain when you're trying to do something that the compiler doesn't know is safe.
    In these cases, you can use unsafe Rust to tell the compiler that you know better than it does.
    Remember that when writing unsafe code, you are responsible for ensuring that it is safe.
    If you write unsafe code that is not safe, you can get memory unsafety bugs.
    But you will always know that these kind of bugs are somewhere inside your unsafe blocks.

    Unsafe Rust allows you to:
        * Dereference raw pointers
        * Call unsafe functions or methods
        * Access or modify a mutable static variable
        * Implement an unsafe trait
        * Access fields of unions

    These five actions are sometimes referenced as "unsafe superpowers".

    It doesn't mean that the borrow checker is turned off or that you can do whatever you want.
    It also doesn't turn off Rust's other safety checks: If you use a reference in unsafe code, it will still be checked.
    It just means that you'll get access to these five features that are then not checked by the compiler for memory safety.
    So you'll still get some amount of safety inside unsafe blocks.

    In addition, unsafe doesn't mean the code in these blocks will necessarily perform poorly or is dangerous.
    The intent is that the programmer ensures the code inside an unsafe block is safe.
    As unsafe blocks are usually small, it's easier to ensure safety than if you had to write an entire program with no compiler checks.

    One thing not covered in these examples is that unsafe allows you to access fields of unions.
    Unions are a data structure that allows you to store one of multiple types in the same location in memory.
    In other words, they can have multiple attributes, like structs, but only one of them is used per instance at runtime.
    Rust doesn't really use unions anywhere, but they are used in C, so Rust supports them for interoperability with C.
*/

mod external_functions;
mod mutable_static_variables;
mod raw_pointers;
mod safe_abstractions;
mod unsafe_functions;
mod unsafe_traits;

fn main() {
    println!("=== Raw pointers ===");
    raw_pointers::run();

    println!("=== Unsafe functions ===");
    unsafe_functions::run();

    println!("=== Safe abstractions ===");
    safe_abstractions::run();

    println!("=== External functions ===");
    external_functions::run();

    println!("=== Mutable static variables ===");
    mutable_static_variables::run();

    println!("=== Unsafe traits ===");
    unsafe_traits::run();
}
