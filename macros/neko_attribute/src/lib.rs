/*
    Edit the Cargo.toml file to make the following changes:
    1. Add [lib] section and set proc-macro = true
    2. Add syn dependency
    3. Add quote dependency
    4. Add neko_attribute as a local dependency to the workspace
*/

use std::fmt::Display;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, Item, LitStr, Token,
};

#[derive(Debug)]
enum NyaType {
    Nya,
    Meow,
}

impl Display for NyaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NyaType::Nya => write!(f, "Nya"),
            NyaType::Meow => write!(f, "Meow"),
        }
    }
}

impl Parse for NyaType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        match ident.to_string().as_str() {
            "Nya" => Ok(NyaType::Nya),
            "Meow" => Ok(NyaType::Meow),
            kind => Err(syn::Error::new(
                ident.span(),
                format!("Invalid NyaType: {}", kind),
            )),
        }
    }
}

struct NekoAttribute {
    nya_type: NyaType,
    name: String,
}

impl Parse for NekoAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let nya_type: NyaType = input.parse()?;
        input.parse::<Token![,]>()?;
        let name: LitStr = input.parse()?;
        let name = name.value();
        Ok(NekoAttribute { nya_type, name })
    }
}

#[proc_macro_attribute]
pub fn neko(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let attribute = parse_macro_input!(attribute as NekoAttribute);
    let item = parse_macro_input!(item as Item);

    neko_generate_impl(&attribute, &item)
}

fn neko_generate_impl(attribute: &NekoAttribute, item: &Item) -> TokenStream {
    let signature = quote!(#item);
    let nya_type = attribute.nya_type.to_string();
    let name = attribute.name.as_str();

    let gen = quote! {
        fn neko_function() {
            println!("Implemented on function with following signature: {}", stringify!(#signature));
            println!("Nya type: {}, name: {}", stringify!(#nya_type), stringify!(#name));
        }
    };

    gen.into()
}
