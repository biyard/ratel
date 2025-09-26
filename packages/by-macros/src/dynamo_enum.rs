use convert_case::Casing;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, Meta};

use crate::write_file::write_file;

pub fn dynamo_enum_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let Data::Enum(DataEnum { variants, .. }) = &input.data else {
        return syn::Error::new_spanned(input.ident, "EnumFromStr can only be derived for enums")
            .to_compile_error()
            .into();
    };

    let mut arms = Vec::new();
    let mut display_arms = Vec::new();

    let mut error_type = quote! { String };

    // Check if there's a container-level error attribute
    for attr in &input.attrs {
        if attr.path().is_ident("dynamo_enum") {
            if let Meta::List(meta_list) = &attr.meta {
                for nested in meta_list.tokens.to_string().split(',') {
                    let nested = nested.trim();
                    if let Some(error_str) = nested.strip_prefix("error = ") {
                        let error_str = error_str.trim_matches('"');
                        error_type = error_str.parse().unwrap();
                    }
                }
            }
        }
    }

    for variant in variants {
        let variant_name = &variant.ident;

        match &variant.fields {
            // Handle variants with one field and a prefix pattern from strum
            syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let l = fields.unnamed.len();

                if l == 1 {
                    let prefix = format!(
                        "{}#",
                        variant_name
                            .to_string()
                            .to_case(convert_case::Case::UpperSnake)
                    );
                    arms.push(quote! {
                        s if s.starts_with(#prefix) => #name::#variant_name(s[#prefix.len()..].to_string()),
                    });

                    display_arms.push(quote! {
                        Self::#variant_name(value) => write!(f, "{}{}", #prefix, value),
                    });
                } else if l == 2 {
                    return syn::Error::new_spanned(
                        input.ident,
                        "DynamoEnum does not support variants with 2 unnamed fields; use custom bridge function",
                    )
                    .to_compile_error()
                    .into();
                } else {
                    return syn::Error::new_spanned(
                        variant_name,
                        "DynamoEnum supports variants with 0 (unit), 1, or 2 unnamed fields only",
                    )
                    .to_compile_error()
                    .into();
                };
            }
            // Handle unit variants (no fields)
            syn::Fields::Unit => {
                // For unit variants, match the exact string representation
                let variant_str = variant_name.to_string();
                arms.push(quote! {
                    #variant_str => #name::#variant_name,
                });
                display_arms.push(quote! {
                    Self::#variant_name => write!(f, "{}", #variant_str),
                });
            }
            _ => {
                // Skip variants that don't have the expected structure
                continue;
            }
        }
    }

    // Generate the error case
    let error_case = if error_type.to_string() == "String" {
        quote! {
            _ => Err(format!("Invalid {}: {}", stringify!(#name), s))?,
        }
    } else {
        // Assume it's a custom error type with an InvalidPartitionKey constructor
        quote! {
            _ => Err(#error_type::InvalidPartitionKey(s.to_string()))?,
        }
    };

    let expanded = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display_arms)*
                }
            }
        }

        impl std::str::FromStr for #name {
            type Err = #error_type;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s {
                    #(#arms)*
                    #error_case
                })
            }
        }
    };

    write_file(name.to_string(), "dynamo_enum", expanded.to_string());

    TokenStream::from(expanded)
}
