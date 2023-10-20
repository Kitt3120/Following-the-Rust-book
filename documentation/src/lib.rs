// This is a single comment for simple annotations

/*
    This is a multi-line comment
    for more complex annotations
*/

//! This is a document comment.
//! Instead of targeting the next item, it targets the whole document..
//! As these comments are placed in no particular item, they are placed in the crate root.
//! Which means that this is the main crate documentation.

/// This is an example for documenting a function.
pub fn run() {
    let result = add(1, 2);
    println!("1 + 2 = {}", result);
}

/// This is a doc comment for a function.
/// By using the doc comment, these comments will be attached to the generated documentation.
/// You can also implement documentation tests by using the assert macros.
/// By doing so, you can write actual tests instead of commenting "variable x should hold value y now"
/// Documentation is generated by running `cargo doc` and can be viewed with `cargo doc --open`
///
/// # Examples
///
/// ```
/// let number1 = 5;
/// let number2 = 6;
/// let answer = documentation::add(number1, number2);
///
/// assert_eq!(answer, 11);
/// ```
pub fn add(number1: i32, number2: i32) -> i32 {
    number1 + number2
}