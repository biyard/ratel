use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, spanned::Spanned, Attribute, Data, DataEnum, DeriveInput, Fields, Variant,
};

use crate::write_file;

pub fn rest_error_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_ident = input.clone().ident;

    let Data::Enum(DataEnum { variants, .. }) = input.data else {
        return syn::Error::new(input.span(), "RestError can only be derived for enums")
            .to_compile_error()
            .into();
    };

    let mut status_arms = Vec::new();
    let mut code_arms = Vec::new();
    let mut last_used_code: u64 = 0;

    for v in variants {
        let (status, code) = match extract_status_code(&v.attrs, last_used_code) {
            Ok(sc) => sc,
            Err(e) => return e.to_compile_error().into(),
        };

        last_used_code = code;

        let pat = variant_ignoring_payload(&enum_ident, &v);

        status_arms.push(quote! { #pat => #status });
        code_arms.push(quote! { #pat => #code });
    }

    let expanded = quote! {
        impl #enum_ident {
            pub fn status(&self) -> u16 {
                match self {
                    #(#status_arms),*
                }
            }
            pub fn code(&self) -> u64 {
                match self {
                    #(#code_arms),*
                }
            }
        }

        impl axum::response::IntoResponse for #enum_ident {
            fn into_response(self) -> axum::response::Response {
                let status = self.status();
                let body = ::serde_json::json!({
                    "code": self.code(),
                    "message": self.to_string(),
                });
                (axum::http::StatusCode::from_u16(status).unwrap(), axum::Json(body)).into_response()
            }
        }
    };

    write_file::write_file(enum_ident.to_string(), "rest_error", expanded.to_string());

    TokenStream::from(expanded)
}

fn extract_status_code(attrs: &[Attribute], last_used_code: u64) -> syn::Result<(u16, u64)> {
    let mut status: u16 = 400;
    let mut code: u64 = last_used_code + 1;

    for attr in attrs {
        if !attr.path().is_ident("rest_error") {
            continue;
        }

        let _ = attr.parse_nested_meta(|meta| {
            let path = &meta.path;

            if path.is_ident("status") {
                let lit = meta.value()?.parse::<syn::LitInt>()?;
                let v = lit.base10_parse::<u16>()?;
                status = v;
                Ok(())
            } else if path.is_ident("code") {
                let lit = meta.value()?.parse::<syn::LitInt>()?;
                let v = lit.base10_parse::<u64>()?;
                code = v;
                Ok(())
            } else {
                Err(syn::Error::new(
                    path.span(),
                    "unknown attribute key; expected `status` or `code`",
                ))
            }
        });
    }

    Ok((status, code))
}

fn variant_ignoring_payload(enum_ident: &syn::Ident, v: &Variant) -> proc_macro2::TokenStream {
    let ident = &v.ident;
    match &v.fields {
        Fields::Unit => quote! { #enum_ident::#ident },
        Fields::Unnamed(_) => quote! { #enum_ident::#ident (..) },
        Fields::Named(_) => quote! { #enum_ident::#ident { .. } },
    }
}
