/*
    So far, you have used macros a lot of time, but they haven't been covered yet.
    Macros are the functions you can call with ! at the end, like println!.
    There are declarative macros (defined with macro_rules!) and three types of procedural macros.

    Fundamentally, macros are a way of writing code that writes other code, which is known as metaprogramming.
    You can think of them as code generators.

    Essentially, macros help you to avoid writing redundant code.
    But functions and methods also do that, so what's the difference?

    Functions and methods are evaluated at runtime.
    Traits are evaluated at compile time.
    But macros are evaluated before the compiler interprets your code, so they can have an even higher impact.
    For example, a macro can automatically implement a trait on a given type, before compiling.
    So at compile time, the compiler will see and compile that trait implementation.
    You can't do that with a function or method, as it is evaluated at runtime.

    Another difference is that macro parameters are not limited to types.
    Think about the vec! macro, which creates a vector of any type.
    Imagine trying to write a function that does the same thing.
    You can't know the type of the vector elements upfront, so you can't write a function that does the same thing as vec!.
    You also don't know how many parameters the user will pass to the function.

    The downside of macros is that they are hard to write and understand.
    Essentially, you are writing Rust code that writes Rust code.
    This makes macro definitions harder to implement, read, debug and maintain, compared to functions and methods.
*/

mod attribute_macro;
mod declarative_macros;
mod derive_macro;
mod function_macro;
mod procedural_macros;

fn main() {
    println!("=== Declarative Macros ===");
    declarative_macros::run();

    println!();

    println!("=== Procedural Macros ===");
    procedural_macros::run();

    println!();

    println!("=== Derive Macros ===");
    derive_macro::run();

    println!();

    println!("=== Attribute Macros ===");
    attribute_macro::run();

    println!();

    println!("=== Function-like Macros ===");
    function_macro::run();
}
