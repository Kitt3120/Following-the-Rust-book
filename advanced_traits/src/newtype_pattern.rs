/*
    In a previous chapter it was discussed that Rust allows you to only implement traits
    for a type if either the trait or the type is local to your crate.
    By using the newtype pattern, you can circumvent this restriction.

    The newtype pattern involves creating a new type in a tuple struct with one field.
    At compile time, Rust will elide the extra layer.
    There is no runtime penalty for using this pattern.

    The downside of using the newtype pattern is that,
    as you wrap your target type inside a new type,
    you lose out on the type’s original methods.

    This can be handled in two ways:
    1. You selectively implement wrapper methods on the new type that call the inner type’s methods.
    2. You implement the Deref [+DerefMut] trait on the new type to make all the inner type’s methods available on the new type.

    Below, we will implement the Deref trait.
*/

use std::{
    fmt::{Display, Formatter, Result},
    ops::{Deref, DerefMut},
};

struct Wrapper(String);

impl Display for Wrapper {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "This is our own Display implementation for String: {}",
            self.0
        )
    }
}

impl Deref for Wrapper {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Wrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn run() {
    let mut w = Wrapper(String::from("Inner String"));
    w.push_str(" says hi!");
    println!("{}", w);
}
