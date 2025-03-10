extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro_crate;

use proc_macro::TokenStream;
use proc_macro_crate::{crate_name, FoundCrate};

#[proc_macro_derive(Task)]
pub fn task(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast: syn::DeriveInput = syn::parse_str(&s).unwrap();

    // Determine the correct path to the ragent crate
    let found_crate = crate_name("ragent").expect("ragent is not found in Cargo.toml");

    let path = match found_crate {
        FoundCrate::Itself => quote!(crate::prelude::Task),
        FoundCrate::Name(crate_name) => {
            let ident = syn::Ident::new(&crate_name, proc_macro2::Span::call_site());
            quote!(::#ident::prelude::Task)
        }
    };

    // Build the impl
    let name = &ast.ident;
    let quote = quote! {
        impl #path for #name {
        }
    };
    TokenStream::from(quote)
}