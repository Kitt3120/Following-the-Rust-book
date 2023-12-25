/*
    Edit the Cargo.toml file to make the following changes:
    1. Add [lib] section and set proc-macro = true
    2. Add syn dependency
    3. Add quote dependency
    4. Add neko_function as a local dependency to the workspace
*/

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn nya(input: TokenStream) -> TokenStream {
    let string = match input.is_empty() {
        true => None,
        false => Some(parse_macro_input!(input as LitStr)),
    };

    nya_generate_impl(&string)
}

fn nya_generate_impl(string: &Option<LitStr>) -> TokenStream {
    let name = match string {
        Some(string) => string.value(),
        None => String::from("No string provided"),
    };

    let gen = quote! {
        println!("Nya! {}", stringify!(#name));
    };

    gen.into()
}
