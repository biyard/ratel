use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    FnArg, Ident, ItemFn, LitStr, Pat, PatType, Token,
};

/// Parsed attributes from `#[mcp_tool(name = "...", description = "...")]`
struct McpToolArgs {
    name: LitStr,
    description: LitStr,
}

impl Parse for McpToolArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut name = None;
        let mut description = None;

        let pairs = Punctuated::<syn::MetaNameValue, Token![,]>::parse_terminated(input)?;
        for pair in pairs {
            let key = pair
                .path
                .get_ident()
                .ok_or_else(|| syn::Error::new_spanned(&pair.path, "expected identifier"))?
                .to_string();

            let value = match &pair.value {
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(s),
                    ..
                }) => s.clone(),
                _ => {
                    return Err(syn::Error::new_spanned(
                        &pair.value,
                        "expected string literal",
                    ))
                }
            };

            match key.as_str() {
                "name" => name = Some(value),
                "description" => description = Some(value),
                other => {
                    return Err(syn::Error::new_spanned(
                        &pair.path,
                        format!("unknown attribute: {other}"),
                    ))
                }
            }
        }

        Ok(McpToolArgs {
            name: name.ok_or_else(|| input.error("missing `name` attribute"))?,
            description: description
                .ok_or_else(|| input.error("missing `description` attribute"))?,
        })
    }
}

/// Parsed info from the next `#[post("...", user: User)]` or `#[get("...", user: User)]` attribute.
struct RouteAttrInfo {
    /// Extracted "user" bindings from the route attribute (e.g., `user: User`)
    extractors: Vec<(Ident, syn::Type)>,
}

fn parse_route_attr(attr: &syn::Attribute) -> Option<RouteAttrInfo> {
    let path_ident = attr.path().get_ident()?;
    let name = path_ident.to_string();
    if !matches!(name.as_str(), "post" | "get" | "put" | "patch" | "delete") {
        return None;
    }

    // Parse the attribute tokens: ("path", key: Type, ...)
    let mut extractors = Vec::new();
    if let syn::Meta::List(meta_list) = &attr.meta {
        let tokens = meta_list.tokens.clone();
        // We need to parse: "path", ident: Type, ident: Type, ...
        // Use a simple approach: parse comma-separated items after the path string
        let parser = |input: ParseStream| -> syn::Result<Vec<(Ident, syn::Type)>> {
            let mut result = Vec::new();
            // Skip the path string
            let _path: LitStr = input.parse()?;

            while input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
                if input.is_empty() {
                    break;
                }
                // Try to parse `ident: Type`
                if input.peek(Ident) && input.peek2(Token![:]) {
                    let ident: Ident = input.parse()?;
                    let _: Token![:] = input.parse()?;
                    let ty: syn::Type = input.parse()?;
                    result.push((ident, ty));
                }
            }
            Ok(result)
        };

        match syn::parse::Parser::parse2(parser, tokens) {
            Ok(pairs) => {
                extractors = pairs;
            }
            Err(err) => {
                // Surface a clear diagnostic when a recognized route attribute
                // cannot be parsed, instead of silently ignoring the error.
                panic!("failed to parse route attribute `{}`: {}", name, err);
            }
        }
    }

    Some(RouteAttrInfo { extractors })
}

pub fn mcp_tool_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = match syn::parse2::<McpToolArgs>(attr) {
        Ok(args) => args,
        Err(e) => return e.to_compile_error(),
    };

    let mut function = match syn::parse2::<ItemFn>(item.clone()) {
        Ok(f) => f,
        Err(e) => return e.to_compile_error(),
    };

    let tool_name = &args.name;
    let tool_description = &args.description;

    // Find the route attribute (#[post], #[get], etc.) to extract user/session params
    let route_info = function
        .attrs
        .iter()
        .find_map(parse_route_attr);

    let extractor_params: Vec<(Ident, syn::Type)> = route_info
        .map(|info| info.extractors)
        .unwrap_or_default();

    // Collect the function's own params (from the signature)
    let fn_params: Vec<&PatType> = function
        .sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(pat_type) => Some(pat_type),
            _ => None,
        })
        .collect();

    // Build the _mcp_impl function signature:
    //   pub async fn create_post_handler_mcp_impl(user: User, param1: T1, ...) -> Result<...>
    let fn_name = &function.sig.ident;
    let impl_fn_name = format_ident!("{}_mcp_impl", fn_name);
    let vis = &function.vis;
    let return_type = &function.sig.output;
    let original_body = &function.block;
    let fn_attrs: Vec<_> = function
        .attrs
        .iter()
        .filter(|a| {
            let path = a.path();
            // Keep doc attrs, skip route/mcp_tool attrs
            path.is_ident("doc") || path.is_ident("cfg")
        })
        .collect();

    // Build extractor param tokens for the _impl function
    let extractor_param_tokens: Vec<_> = extractor_params
        .iter()
        .map(|(name, ty)| quote! { #name: #ty })
        .collect();

    // Build fn param tokens for the _impl function
    let fn_param_tokens: Vec<_> = fn_params.iter().map(|p| quote! { #p }).collect();

    // All params for the _impl function
    let all_impl_params = {
        let mut params = Vec::new();
        params.extend(extractor_param_tokens.iter().cloned());
        params.extend(fn_param_tokens.iter().cloned());
        params
    };

    // Build the call args for the original function body rewrite
    let extractor_names: Vec<_> = extractor_params.iter().map(|(name, _)| name).collect();
    let fn_param_names: Vec<_> = fn_params
        .iter()
        .map(|p| match &*p.pat {
            Pat::Ident(pat_ident) => &pat_ident.ident,
            other => panic!(
                "#[mcp_tool]: unsupported parameter pattern `{}`. \
                 Only simple identifier patterns are supported (e.g., `name: Type`). \
                 Destructuring patterns and wildcards are not supported.",
                quote! { #other }
            ),
        })
        .collect();

    let all_call_args = {
        let mut args: Vec<_> = extractor_names.iter().map(|n| quote! { #n }).collect();
        args.extend(fn_param_names.iter().map(|n| quote! { #n }));
        args
    };

    // Generate the _mcp_impl function with the original body
    let impl_fn = quote! {
        #(#fn_attrs)*
        #[cfg(feature = "server")]
        #vis async fn #impl_fn_name(#(#all_impl_params),*) #return_type
            #original_body
    };

    // Rewrite the original function body to call _mcp_impl
    let new_body: syn::Block = syn::parse2(quote! {
        {
            #impl_fn_name(#(#all_call_args),*).await
        }
    })
    .expect("failed to parse rewritten body");

    function.block = Box::new(new_body);

    // Generate a constant that stores the tool metadata for MCP registration
    let meta_const_name = format_ident!(
        "MCP_TOOL_META_{}",
        fn_name.to_string().to_uppercase()
    );

    let impl_fn_name_str = impl_fn_name.to_string();

    let output = quote! {
        #impl_fn

        /// MCP tool metadata generated by `#[mcp_tool]`.
        #[cfg(feature = "server")]
        #vis const #meta_const_name: (&str, &str, &str) = (#tool_name, #tool_description, #impl_fn_name_str);

        #function
    };

    output
}
