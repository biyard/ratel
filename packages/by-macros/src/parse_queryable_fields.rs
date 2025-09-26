use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::*;

use crate::{action::ActionRequestField, api_model_struct::to_string};

pub fn parse_queryable_action_fields(
    queryable_fields: &Vec<ActionRequestField>,
    struct_name: &Ident,
    hashed_fields: &mut std::collections::HashSet<Option<syn::Ident>>,
    query_fields: &mut Vec<TokenStream>,
    query_builder_functions: &mut Vec<TokenStream>,
    parsers: &mut Vec<TokenStream>,
) {
    for field in queryable_fields {
        let field_name = Ident::new(&field.name, proc_macro2::Span::call_site());
        let field_type = Type::Verbatim(field.ty());
        hashed_fields.insert(Some(field_name.clone()));

        parse_queryable_field(
            &Some(field_name),
            &field_type,
            struct_name,
            query_fields,
            query_builder_functions,
            parsers,
            &vec![],
        );
    }
}

pub fn parse_queryable_fields(
    queryable_fields: &Vec<syn::Field>,
    struct_name: &Ident,
    hashed_fields: &mut std::collections::HashSet<Option<syn::Ident>>,
    query_fields: &mut Vec<TokenStream>,
    query_builder_functions: &mut Vec<TokenStream>,
    parsers: &mut Vec<TokenStream>,
) {
    for field in queryable_fields {
        let field_name = &field.ident;
        hashed_fields.insert(field_name.clone());
        let field_type = &field.ty;
        let mut validate_attributes = Vec::new();
        for attr in &field.attrs {
            if let Meta::List(meta_list) = attr.meta.clone() {
                if meta_list.path.is_ident("validate") {
                    validate_attributes.push(attr.clone());
                }
            }
        }
        parse_queryable_field(
            &field_name,
            &field_type,
            struct_name,
            query_fields,
            query_builder_functions,
            parsers,
            &validate_attributes,
        );
    }
}

fn parse_queryable_field(
    field_name: &Option<Ident>,
    field_type: &Type,
    struct_name: &Ident,
    query_fields: &mut Vec<TokenStream>,
    query_builder_functions: &mut Vec<TokenStream>,
    parsers: &mut Vec<TokenStream>,
    validate_attributes: &Vec<syn::Attribute>,
) {
    let convert = match to_string(&field_type).as_str() {
        "String" => quote! {},
        _ => {
            let fname = syn::Ident::new(
                &format!(
                    "parse_{}_of_{}_query",
                    field_name.clone().unwrap().to_string().to_case(Case::Snake),
                    struct_name.to_string().to_case(Case::Snake)
                ),
                struct_name.span(),
            );

            let fname_str = syn::LitStr::new(
                &format!(
                    "parse_{}_of_{}_query",
                    field_name.clone().unwrap().to_string().to_case(Case::Snake),
                    struct_name.to_string().to_case(Case::Snake)
                ),
                struct_name.span(),
            );

            parsers.push(quote! {
                    pub fn #fname<'de, D>(deserializer: D) -> std::result::Result<Option<#field_type>, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        use serde::Deserialize;

                        let s: Option<String> = Option::deserialize(deserializer)?;
                        match s {
                            Some(s) => {
                                s.parse::<#field_type>()
                                    .map_err(serde::de::Error::custom)
                                    .map(Some)
                            }
                            None => Ok(None),
                        }
                    }
                });
            quote! {
                #[serde(deserialize_with = #fname_str, default)]
            }
        }
    };

    query_fields.push(quote! {
        #(#validate_attributes)*
        #convert
        pub #field_name: Option<#field_type>,
    });
    let function_name = syn::Ident::new(
        &format!("with_{}", field_name.as_ref().unwrap()),
        struct_name.span(),
    );

    query_builder_functions.push(quote! {
        pub fn #function_name(mut self, #field_name: #field_type) -> Self {
            self.#field_name = Some(#field_name);
            self
        }
    });
}
