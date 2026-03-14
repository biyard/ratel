use convert_case::Casing;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, Ident};

use crate::write_file;

pub fn sub_partition_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident.clone();

    let out = match &input.data {
        Data::Enum(ds) => generate_enum_impl(ident.clone(), ds),
        _ => {
            return syn::Error::new_spanned(input, "#[derive(SubPartition)] only supports enum")
                .to_compile_error()
                .into();
        }
    };

    // record default/consts
    write_file::write_file(ident.to_string(), "sub_partition", out.to_string());

    out.into()
}

fn generate_enum_impl(ident: Ident, ds: &DataEnum) -> proc_macro2::TokenStream {
    let mut struct_defs = Vec::new();

    for variant in &ds.variants {
        let variant_name = &variant.ident;

        // Only process variants with exactly one String field
        if let syn::Fields::Unit = &variant.fields {
            let struct_name =
                syn::Ident::new(&format!("{}{}", variant_name, ident), variant_name.span());

            let prefix = syn::LitStr::new(
                &format!(
                    "{}",
                    variant_name
                        .to_string()
                        .to_case(convert_case::Case::UpperSnake)
                ),
                variant_name.span(),
            );

            let struct_def = quote! {
                #[derive(
                    Debug,
                    Clone,
                    serde_with::SerializeDisplay,
                    serde_with::DeserializeFromStr,
                    Default,
                    PartialEq,
                    Eq,
                )]
                #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
                pub struct #struct_name;

                impl std::fmt::Display for #struct_name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{}", #prefix)
                    }
                }

                impl std::str::FromStr for #struct_name {
                    type Err = crate::Error;

                    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                        if s == #prefix {
                            Ok(#struct_name)
                        } else {
                            Err(Self::Err::InvalidPartitionKey(format!("{} must be {}", stringify!(#struct_name), #prefix)))
                        }
                    }
                }

                impl Into<#ident> for #struct_name {
                    fn into(self) -> #ident {
                        #ident::#variant_name
                    }
                }

                impl From<#ident> for #struct_name {
                    fn from(partition: #ident) -> Self {
                        match partition {
                            #ident::#variant_name => Self,
                            _ => Self,
                        }
                    }
                }

                impl From<String> for #struct_name {
                    fn from(s: String) -> Self {
                        if &s == #prefix {
                            #struct_name
                        } else {
                            panic!("{}", format!("{} must be {}", stringify!(#struct_name), #prefix))
                        }
                    }
                }
            };

            struct_defs.push(struct_def);
        } else if let syn::Fields::Unnamed(fields) = &variant.fields {
            if fields.unnamed.len() == 1 {
                let struct_name =
                    syn::Ident::new(&format!("{}{}", variant_name, ident), variant_name.span());

                let prefix = syn::LitStr::new(
                    &format!(
                        "{}#",
                        variant_name
                            .to_string()
                            .to_case(convert_case::Case::UpperSnake)
                    ),
                    variant_name.span(),
                );

                let struct_def = quote! {
                    #[derive(
                        Debug,
                        Clone,
                        serde_with::SerializeDisplay,
                        serde_with::DeserializeFromStr,
                        Default,
                        PartialEq,
                        Eq,
                    )]
                    #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
                    pub struct #struct_name(pub String);

                    impl std::fmt::Display for #struct_name {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            write!(f, "{}", self.0)
                        }
                    }

                    impl std::str::FromStr for #struct_name {
                        type Err = crate::Error;

                        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                            let s = if s.starts_with(#prefix) {
                                s.replace(#prefix, "").to_string()
                            } else {
                                s.to_string()
                            };

                            Ok(#struct_name(s))
                        }
                    }

                    impl Into<#ident> for #struct_name {
                        fn into(self) -> #ident {
                            #ident::#variant_name(self.0)
                        }
                    }

                    impl From<#ident> for #struct_name {
                        fn from(partition: #ident) -> Self {
                            match partition {
                                #ident::#variant_name(id) => Self(id),
                                _ => Self("".to_string()),
                            }
                        }
                    }

                    impl From<String> for #struct_name {
                        fn from(s: String) -> Self {
                            let s = if s.starts_with(#prefix) {
                                s.replace(#prefix, "").to_string()
                            } else {
                                s.to_string()
                            };

                            #struct_name(s)
                        }
                    }
                };

                struct_defs.push(struct_def);
            }
        }
    }

    quote! {
        #(#struct_defs)*
    }
}
