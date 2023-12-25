/*
    Attribute macros are similar to derive macros, but instead of adding types to the derive attribute, you add entire new attributes.
    Additionally, derive macros can only be used on structs and enums, while attribute macros can be used on any item.
    Also, if your attribute's pattern isn't just a simple string, you have to define a struct and implement parse on it in the macro.

    Attribute macros, just like derive macros, can only be implemented as their own crate.
    So we will create a new crate for this example within the same workspace.
    We'll use the command: cargo new neko_attribute --lib
*/

use neko_attribute::neko;

#[neko(Nya, "Nyanners")]
fn neko_function() {
    println!("Nya!"); // This will be overwritten by the attribute macro
}

pub fn run() {
    neko_function();
}
