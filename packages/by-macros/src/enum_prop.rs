use convert_case::{Case, Casing};
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

pub fn enum_prop_impl(input: TokenStream) -> TokenStream {
    tracing::debug!("enum_prop_impl");

    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    tracing::debug!("fields: {:?}", input.data);
    let fields = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => panic!("only enum is supported"),
    };

    let mut matches_for_display = vec![];
    let mut matches_for_from_str = vec![];
    let mut matches_for_to_string = vec![];

    for field in fields.iter() {
        let field_name = &field.ident;
        tracing::debug!("field_name: {:?}", field_name);
        let kebab_case = syn::LitStr::new(
            &field.ident.to_string().to_case(Case::Kebab),
            field.ident.span(),
        );
        matches_for_display.push(quote! {
           #name::#field_name => write!(f, #kebab_case),
        });
        matches_for_from_str.push(quote! {
            #kebab_case => Ok(#name::#field_name),
        });
        matches_for_to_string.push(quote! {
            #name::#field_name => #kebab_case.to_string(),
        });
    }

    let output = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#matches_for_display)*
                }
            }
        }

        impl std::str::FromStr for #name {
            type Err = String;

            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                match s {
                    #(#matches_for_from_str)*
                    _ => Err(format!("Invalid type: {}", s)),
                }
            }
        }

        impl #name {
            pub fn to_string(&self) -> String {
                match self {
                    #(#matches_for_to_string)*
                }
            }
        }
    };
    tracing::debug!("EnumProp output: {:?}", output.to_string());

    output.into()
}
