/*
    For this example, we will define a Neko trait, which will provide a nya method.
    We will then write a derive macro that implements the nya method for a given type.
    This way, users of our crate can derive the Neko trait for their types instead of implementing it manually on each type.

    When you now think about default trait implementations, you might wonder why we don't just use them.
    The reason is that Rust doesn't have reflections yet, so we can't get a type's name at runtime.
    So, if the default implementation of the nya method should print the type's name, it's impossible to do with a default implementation.

    As of now, derivce macros can only be implemented as their own crate.
    So we will create a new crate for this example within the same workspace.
    We'll use the command: cargo new neko_derive --lib
*/

use neko_derive::NekoMacro;

trait Neko {
    fn nya();
}

#[derive(NekoMacro)]
struct NekoCat {}

pub fn run() {
    NekoCat::nya();
}
