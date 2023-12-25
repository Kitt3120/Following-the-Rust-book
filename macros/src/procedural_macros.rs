/*
    Procedural macros do not use pattern matching.
    They take in Rust code as an input, operate on it, and produce Rust code as an output.

    There are three types of procedural macros:
        - Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
        - Attribute-like macros that define custom attributes usable on any item
        - Function-like macros that look like function calls but operate on the tokens specified as their argument

    A procedural macro is defined like this:

    #[one_of_three_attributes]
    pub fn some_name(input: TokenStream) -> TokenStream {
        //code
    }

    It has one of three attributes:
        - #[proc_macro]: for function-like macros
        - #[proc_macro_derive]: for custom derive macros
        - #[proc_macro_attribute]: for attribute-like macros

    It takes an input TokenStream and returns a TokenStream.
    TokenStream is a type that can be brought into scope with use proc_macro::TokenStream.
    The proc_macro crate is included in the standard library.

    A TokenStream is a sequence of tokens that the compiler can parse.
    That sequence of tokens is made up of source code.

*/

pub fn run() {
    println!("Nothing to do, just explanations.");
}
