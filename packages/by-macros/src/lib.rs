extern crate proc_macro;

mod action;
mod api_model;
mod api_model_struct;
mod enum_prop;
pub(crate) mod parse_queryable_fields;
#[cfg(feature = "server")]
mod query_builder_functions;
mod query_display;
#[cfg(feature = "server")]
mod sql_model;

use api_model::api_model_impl;
use enum_prop::enum_prop_impl;
use proc_macro::TokenStream;
use query_display::query_display_impl;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, Fields};

#[proc_macro_derive(QueryDisplay)]
pub fn query_display_derive(input: TokenStream) -> TokenStream {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .try_init();
    query_display_impl(input)
}

#[proc_macro_attribute]
pub fn api_model(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .try_init();
    api_model_impl(attr.into(), item.into()).into()
}

#[proc_macro_derive(EnumProp)]
pub fn enum_prop_derive(input: TokenStream) -> TokenStream {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .try_init();
    enum_prop_impl(input)
}

#[proc_macro_derive(ApiModel)]
pub fn derive_api_model(input: TokenStream) -> TokenStream {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .try_init();

    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let Data::Enum(DataEnum { variants, .. }) = &input.data else {
        return syn::Error::new_spanned(input.ident, "ApiModel can only be derived for enums")
            .to_compile_error()
            .into();
    };

    let try_from_arms = variants.iter().map(|v| {
        let ident = &v.ident;
        let discriminant = match &v.discriminant {
            Some((_, expr)) => quote! { #expr },
            None => quote! { compile_error!("Enum variants must have explicit discriminants"); },
        };
        tracing::trace!("discriminant: {}", discriminant.to_string());
        quote! { val if val == #discriminant => Ok(#name::#ident), }
    });

    let expanded = quote! {
        impl std::convert::TryFrom<i32> for #name {
            type Error = String;

            fn try_from(value: i32) -> std::result::Result<Self, Self::Error> {
                match value {
                    #(#try_from_arms)*
                    _ => Err(format!("Invalid {}: {}", stringify!(#name), value)),
                }
            }
        }

        impl std::convert::Into<i32> for #name {
            fn into(self) -> i32 {
                self as i32
            }
        }

        #[cfg(feature = "server")]
        impl sqlx::Type<sqlx::Postgres> for #name {
            fn type_info() -> sqlx::postgres::PgTypeInfo {
                <i32 as sqlx::Type<sqlx::Postgres>>::type_info()
            }
        }

        #[cfg(feature = "server")]
        impl sqlx::Encode<'_, sqlx::Postgres> for #name {
            fn encode_by_ref(
                &self,
                buf: &mut sqlx::postgres::PgArgumentBuffer,
            ) -> std::result::Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
                let value: i32 = (*self).clone().into();
                <i32 as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&value, buf)
            }
        }

        #[cfg(feature = "server")]
        impl<'r> sqlx::Decode<'r, sqlx::Postgres> for #name {
            fn decode(
                value: sqlx::postgres::PgValueRef<'r>,
            ) -> std::result::Result<Self, sqlx::error::BoxDynError> {
                let int_value: i32 = <i32 as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
                #name::try_from(int_value)
                    .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)).into())
            }
        }

        impl serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_i32(self.clone() as i32)
            }
        }

        impl<'de> serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = i32::deserialize(deserializer)?;
                Self::try_from(value)
                    .map_err(|v| serde::de::Error::custom(format!("Failed to parse ApiModel: {}", v)))
            }
        }
    };

    tracing::trace!("ApiModel expanded: {}", expanded.to_string());

    TokenStream::from(expanded)
}

#[proc_macro_derive(DioxusController)]
pub fn derive_dioxus_controller(input: TokenStream) -> TokenStream {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .try_init();

    tracing::trace!("starting derive_dioxus_controller");
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let mut generated_methods = vec![];

    if let Data::Struct(data_struct) = input.data {
        if let Fields::Named(fields) = data_struct.fields {
            tracing::trace!("starting parsing fields");
            for field in fields.named {
                let field_name = &field.ident.unwrap();
                let field_type = field.ty.to_token_stream().to_string();
                let field_type = field_type.trim().replace(" ", "");

                tracing::trace!(
                    "field_name: {}, field_type: {}",
                    field_name.to_string(),
                    field_type
                );

                let method: proc_macro2::TokenStream = if field_type.starts_with("Signal") {
                    let t = field_type.trim_start_matches("Signal<");
                    let t: proc_macro2::TokenStream = t[..t.len() - 1].parse().unwrap();
                    quote! {
                        pub fn #field_name(&self) -> #t {
                            (self.#field_name)()
                        }
                    }
                } else if field_type.starts_with("ReadOnlySignal") {
                    let t = field_type.trim_start_matches("ReadOnlySignal<");
                    let t: proc_macro2::TokenStream = t[..t.len() - 1].parse().unwrap();
                    quote! {
                        pub fn #field_name(&self) -> #t {
                            (self.#field_name)()
                        }
                    }
                } else if field_type.starts_with("Memo") {
                    let t = field_type.trim_start_matches("Memo<");
                    let t: proc_macro2::TokenStream = t[..t.len() - 1].parse().unwrap();
                    quote! {
                        pub fn #field_name(&self) -> #t {
                            (self.#field_name)()
                        }
                    }
                } else if field_type.starts_with("Resource<") {
                    let t = field_type.trim_start_matches("Resource<");
                    let t: proc_macro2::TokenStream = t[..t.len() - 1].parse().unwrap();

                    quote! {
                        pub fn #field_name(&self) -> std::result::Result<#t, RenderError> {
                            Ok(self.#field_name.suspend()?())
                        }
                    }
                } else {
                    continue;
                };

                tracing::trace!("method: {}", method.to_string());

                generated_methods.push(method);
            }
        }
    }

    let expanded = quote! {
        impl #struct_name {
            #(#generated_methods)*
        }
    };

    save_file(struct_name.to_string().as_str(), &expanded.to_string());

    expanded.into()
}

pub(crate) fn save_file(st_name: &str, output: &str) {
    let dir_path = match option_env!("API_MODEL_ARTIFACT_DIR") {
        Some(dir) => dir.to_string(),
        None => {
            let current_dir = std::env::current_dir().unwrap();
            format!(
                "{}",
                current_dir
                    .join(".build/generated_api_models")
                    .to_str()
                    .unwrap()
            )
        }
    };
    use convert_case::Casing;

    let file_path = format!(
        "{}/{}.rs",
        dir_path,
        st_name.to_case(convert_case::Case::Snake)
    );

    let dir = std::path::Path::new(&dir_path);

    use std::fs;

    if !dir.exists() {
        if let Err(e) = fs::create_dir_all(dir) {
            tracing::error!("Failed to create directory: {}", e);
        }
    }

    if let Err(e) = fs::write(&file_path, output.to_string()) {
        tracing::error!("Failed to write file: {}", e);
    } else {
        tracing::info!("generated code {} into {}", st_name, file_path);
    }
}
