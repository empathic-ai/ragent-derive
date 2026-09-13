use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use quote::quote;

#[proc_macro_derive(Task)]
pub fn task(input: TokenStream) -> TokenStream {
    // Parse tokens directly to preserve spans in compiler diagnostics.
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

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
