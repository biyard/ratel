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
    let mut inner_arms = Vec::new();

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
            syn::Fields::Unnamed(fields) if fields.unnamed.len() <= 2 => {
                let l = fields.unnamed.len();

                if l == 1 {
                    let prefix_wo_sharp = format!(
                        "{}",
                        variant_name
                            .to_string()
                            .to_case(convert_case::Case::UpperSnake)
                    );
                    let prefix = format!(
                        "{}#",
                        variant_name
                            .to_string()
                            .to_case(convert_case::Case::UpperSnake)
                    );
                    arms.push(quote! {
                        s if s.eq(#prefix_wo_sharp) => {
                            #name::#variant_name("".to_string())
                        },
                        s if s.starts_with(#prefix) => {
                            let parts: Vec<&str> = s.splitn(2, '#').collect();
                            if parts.len() == 2 {
                                #name::#variant_name(parts[1].to_string())
                            } else {
                                #name::#variant_name("".to_string())
                            }
                        } ,
                    });

                    display_arms.push(quote! {
                        Self::#variant_name(value) => write!(f, "{}#{}", #prefix_wo_sharp, value),
                    });
                    inner_arms.push(quote! {
                        Self::#variant_name(v) => Ok(format!("{v}")),
                    });
                } else if l == 2 {
                    let prefix_wo_sharp = format!(
                        "{}",
                        variant_name
                            .to_string()
                            .to_case(convert_case::Case::UpperSnake)
                    );
                    let prefix = format!(
                        "{}#",
                        variant_name
                            .to_string()
                            .to_case(convert_case::Case::UpperSnake)
                    );

                    arms.push(quote! {
                        s if s.eq(#prefix_wo_sharp) => {
                            #name::#variant_name("".to_string(), "".to_string())
                        },
                        s if s.starts_with(#prefix) => {
                            let parts: Vec<&str> = s.splitn(3, '#').collect();
                            if parts.len() == 3 {
                                #name::#variant_name(parts[1].to_string(), parts[2].to_string())
                            } else if parts.len() == 2 {
                                #name::#variant_name(parts[1].to_string(), "".to_string())
                            } else {
                                #name::#variant_name("".to_string(), "".to_string())
                            }
                        } ,
                    });

                    display_arms.push(quote! {
                        Self::#variant_name(v1, v2) => {
                            if v2.is_empty() {
                                write!(f, "{}#{}", #prefix_wo_sharp, v1)
                            } else {
                                write!(f, "{}#{}#{}", #prefix_wo_sharp, v1, v2)
                            }
                        } ,
                    });

                    inner_arms.push(quote! {
                        Self::#variant_name(v1, v2) => {
                            if v2.is_empty() {
                                Ok(v1.clone())
                            } else {
                                Ok(format!("{}#{}", v1, v2))
                            }
                        },
                    });
                };
            }
            // Handle unit variants (no fields)
            syn::Fields::Unit => {
                // For unit variants, match the exact string representation
                let variant_str = variant_name
                    .to_string()
                    .to_case(convert_case::Case::UpperSnake);
                arms.push(quote! {
                    #variant_str => #name::#variant_name,
                });
                display_arms.push(quote! {
                    Self::#variant_name => write!(f, #variant_str),
                });
                inner_arms.push(quote! {
                    Self::#variant_name => Err("Cannot extract inner value from unit variant".to_string())?,
                });
            }
            _ => {
                // Skip variants that don't have the expected structure
                return syn::Error::new_spanned(
                    variant_name,
                    "DynamoEnum supports variants with 0 (unit), 1, or 2 unnamed fields only",
                )
                .to_compile_error()
                .into();
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
                let s = percent_encoding::percent_decode_str(s)
                    .decode_utf8().map_err(|e| format!("Invalid percent-encoding: {}", e))?;
                let s = s.into_owned();

                Ok(match s.as_str() {
                    #(#arms)*
                    #error_case
                })
            }
        }

        impl #name {
            pub fn try_into_inner(&self) -> Result<String, crate::Error2> {
                match self {
                    #(#inner_arms)*
                }
            }
        }
    };

    write_file(name.to_string(), "dynamo_enum", expanded.to_string());

    TokenStream::from(expanded)
}
