/*
    Function-like macros are defined in a similar way as derive and attribute macros.
    They are very similar to declarative macros, but don't have to be defined by pattern matching.
    Function-like macros can be called with the same syntax as functions,
    but they are expanded into code before compilation.

    Function-like macros, just like derive and attribute macros, can only be implemented as their own crate.
    So we will create a new crate for this example within the same workspace.
    We'll use the command: cargo new neko_function --lib
*/

use neko_function::nya;

pub fn run() {
    nya!();
}
