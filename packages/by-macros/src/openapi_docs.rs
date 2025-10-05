use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    GenericArgument, ItemFn, PathArguments, ReturnType, Token, Type,
};

use crate::write_file::write_file;

struct OpenApiArgs {
    method: Option<String>,
    tag: Option<String>,
    id: Option<String>,
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
            method,
            tag,
            id,
            response,
        })
    }
}

fn extract_state_type(item_fn: &ItemFn) -> Option<syn::Type> {
    for arg in &item_fn.sig.inputs {
        if let syn::FnArg::Typed(pat_type) = arg {
            if let syn::Type::Path(type_path) = &*pat_type.ty {
                if let Some(last_segment) = type_path.path.segments.last() {
                    if last_segment.ident == "State" {
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

fn extract_result_types(return_type: &ReturnType) -> Option<(Type, Type)> {
    if let ReturnType::Type(_, ty) = return_type {
        if let Type::Path(type_path) = &**ty {
            if let Some(last_segment) = type_path.path.segments.last() {
                if last_segment.ident == "Result" {
                    if let PathArguments::AngleBracketed(args) = &last_segment.arguments {
                        let generic_args: Vec<_> = args.args.iter().collect();

                        if generic_args.len() == 2 {
                            if let (
                                GenericArgument::Type(success_type),
                                GenericArgument::Type(error_type),
                            ) = (generic_args[0], generic_args[1])
                            {
                                return Some((success_type.clone(), error_type.clone()));
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
    let inner_fn_name = format_ident!("{}_inner", original_fn_name);

    let (success_type, error_type) =
        extract_result_types(&original_fn.sig.output).unwrap_or_else(|| {
            tracing::warn!("Could not parse Result type, using defaults");
            (syn::parse_str("()").unwrap(), syn::parse_str("()").unwrap())
        });

    tracing::debug!("Extracted success type: {}", quote! { #success_type });
    tracing::debug!("Extracted error type: {}", quote! { #error_type });

    let method_str = args
        .method
        .unwrap_or_else(|| "GET".to_string())
        .to_lowercase();
    let method_ident = format_ident!("{}_with", method_str);

    let tag = args.tag.unwrap_or_else(|| "default".to_string());
    let id = args.id.unwrap_or_else(|| {
        if original_fn_name.to_string().ends_with("_handler") {
            original_fn_name
                .to_string()
                .trim_end_matches("_handler")
                .to_string()
        } else {
            original_fn_name.to_string()
        }
    });

    let response_type = args.response.unwrap_or(success_type.clone());

    let response_call = quote! { .response::<200, #response_type>() };

    let error_call = quote! { .response::<400, #error_type>() };

    let state_type = extract_state_type(&original_fn);
    let return_type = if let Some(state) = state_type {
        quote! { bdk::prelude::axum::routing::ApiMethodRouter<#state> }
    } else {
        quote! { bdk::prelude::axum::routing::ApiMethodRouter }
    };

    let fn_inputs = &original_fn.sig.inputs;
    let fn_output = &original_fn.sig.output;
    let fn_asyncness = &original_fn.sig.asyncness;
    let fn_body = &original_fn.block;

    let generated_code = quote! {
        #fn_asyncness fn #inner_fn_name(#fn_inputs) #fn_output {
            #fn_body
        }
        pub fn #original_fn_name() -> #return_type {
            bdk::prelude::axum::routing::#method_ident(
                #inner_fn_name,
                |op| op
                    .tag(#tag)
                    .id(#id)
                    #response_call
                    #error_call
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
