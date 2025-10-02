use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    ItemFn, Token,
};

use crate::write_file::write_file;

struct OpenApiArgs {
    method: String,
    tag: String,
    id: String,
    response: Option<syn::Type>,
}

impl Parse for OpenApiArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut method = None;
        let mut tag = None;
        let mut id = None;
        let mut response = None;

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match ident.to_string().as_str() {
                "method" => {
                    let lit: syn::LitStr = input.parse()?;
                    method = Some(lit.value());
                }
                "tag" => {
                    let lit: syn::LitStr = input.parse()?;
                    tag = Some(lit.value());
                }
                "id" => {
                    let lit: syn::LitStr = input.parse()?;
                    id = Some(lit.value());
                }
                "response" => {
                    response = Some(input.parse()?);
                }
                _ => return Err(syn::Error::new(ident.span(), "unknown attribute")),
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(OpenApiArgs {
            method: method.ok_or_else(|| input.error("missing method"))?,
            tag: tag.ok_or_else(|| input.error("missing tag"))?,
            id: id.ok_or_else(|| input.error("missing id"))?,
            response,
        })
    }
}

fn extract_state_type(item_fn: &ItemFn) -> Option<syn::Type> {
    for arg in &item_fn.sig.inputs {
        if let syn::FnArg::Typed(pat_type) = arg {
            if let syn::Type::Path(type_path) = &*pat_type.ty {
                // Check if this is State<T>
                if let Some(last_segment) = type_path.path.segments.last() {
                    if last_segment.ident == "State" {
                        // Extract the generic parameter T from State<T>
                        if let syn::PathArguments::AngleBracketed(args) = &last_segment.arguments {
                            if let Some(syn::GenericArgument::Type(state_type)) = args.args.first()
                            {
                                return Some(state_type.clone());
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

pub fn openapi_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args: OpenApiArgs = match syn::parse2(attr) {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("Error parsing attributes: {}", e);
            return e.to_compile_error();
        }
    };

    let original_fn = match syn::parse2::<ItemFn>(item) {
        Ok(f) => f,
        Err(e) => {
            tracing::error!("Error parsing function: {}", e);
            return e.to_compile_error();
        }
    };

    let original_fn_name = &original_fn.sig.ident;
    let new_fn_name = format_ident!("{}_with_doc", original_fn_name);

    let method_str = args.method.to_lowercase();
    let method_ident = format_ident!("{}_with", method_str);

    let tag = args.tag;
    let id = args.id;
    let response_call = if let Some(resp_type) = args.response {
        quote! { .response::<200, #resp_type>() }
    } else {
        quote! { .response::<200, ()>() }
    };

    let state_type = extract_state_type(&original_fn);
    let return_type = if let Some(state) = state_type {
        quote! { bdk::prelude::axum::routing::ApiMethodRouter<#state> }
    } else {
        quote! { bdk::prelude::axum::routing::ApiMethodRouter }
    };

    let generated_code = quote! {
        #original_fn

        pub fn #new_fn_name() -> #return_type {
            bdk::prelude::axum::routing::#method_ident(
                #original_fn_name,
                |op| op
                    .tag(#tag)
                    .id(#id)
                    #response_call
            )
        }
    };
    write_file(
        original_fn_name.to_string(),
        "openapi",
        generated_code.to_string(),
    );

    generated_code
}
