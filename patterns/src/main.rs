/*
    Patterns are a way to match against the structure of types, for example structs, tuples and enums.
    When using a pattern, it is compared to some value.
    A pattern can define bindings, which are names that are used to hold the contents matched against the pattern.
    For example, valid patterns are: Some(x), (x, y), (0..10, 'a'..='z'), and _.

    If a pattern matches a given value, Rust will bind the value to the pattern's declared bindings.
    and (in some cases) execute the code associated with the pattern.

    You have been using patterns already all this time, without knowing it.
    For example, when you use a let or match statement, you are using patterns.

    Examples for possible patterns are shown in types_of_patterns.rs.

    Patterns can be refutable or irrefutable.
    This is demonstrated in refutability.rs.

    The possible pattern syntaxes are shown in pattern_syntax.rs.
*/

mod pattern_syntax;
mod refutability;
mod types_of_patterns;

fn main() {
    println!("=== Types of patterns ===");
    types_of_patterns::run();

    println!();

    println!("=== Refutability ===");
    refutability::run();

    println!();

    println!("=== Pattern syntax ===");
    pattern_syntax::run();
}
