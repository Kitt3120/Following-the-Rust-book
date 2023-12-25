/*
    Edit the Cargo.toml file to make the following changes:
    1. Add [lib] section and set proc-macro = true
    2. Add syn dependency
    3. Add quote dependency
    4. Add neko_derive as a local dependency to the workspace
*/

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(NekoMacro)] // This defines the name of the macro
pub fn neko_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput); // Parse Rust code into a syntax tree. We panic here because we can't return Result.

    neko_generate_impl(&ast) // We just call another function to do the actual work. This makes the actual macro definition very simple.
}

fn neko_generate_impl(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident; // Get the name of the type we are deriving the trait for

    let gen = quote! {
        impl Neko for #name {
            fn nya() {
                println!("{}: Nya!", stringify!(#name));
            }
        }
    };

    gen.into()
}
