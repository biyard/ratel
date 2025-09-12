use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn query_display_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let expanded = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let query = serde_urlencoded::to_string(&self)
                    .map_err(|_| std::fmt::Error)?;
                write!(f, "{}", query)
            }
        }
    };

    TokenStream::from(expanded)
}
