use convert_case::{Case, Casing};
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

/// Parsed info from the route attribute.
struct RouteAttrInfo {
    method: String,
    path: String,
    path_params: Vec<String>,
    query_params: Vec<String>,
}

fn parse_route_attr(attr: &syn::Attribute) -> Option<RouteAttrInfo> {
    let path_ident = attr.path().get_ident()?;
    let name = path_ident.to_string();
    if !matches!(name.as_str(), "post" | "get" | "put" | "patch" | "delete") {
        return None;
    }

    let method = name.to_uppercase();

    if let syn::Meta::List(meta_list) = &attr.meta {
        let tokens = meta_list.tokens.clone();
        let parser = |input: ParseStream| -> syn::Result<String> {
            let path: LitStr = input.parse()?;
            while !input.is_empty() {
                let _ = input.parse::<proc_macro2::TokenTree>();
            }
            Ok(path.value())
        };

        match syn::parse::Parser::parse2(parser, tokens) {
            Ok(route_path) => {
                let (path_params, query_params) = extract_route_params(&route_path);
                Some(RouteAttrInfo {
                    method,
                    path: route_path,
                    path_params,
                    query_params,
                })
            }
            Err(err) => {
                panic!("failed to parse route attribute `{}`: {}", name, err);
            }
        }
    } else {
        None
    }
}

fn extract_route_params(route: &str) -> (Vec<String>, Vec<String>) {
    let mut path_params = Vec::new();
    let mut query_params = Vec::new();

    let (path_part, query_part) = if let Some(idx) = route.find('?') {
        (&route[..idx], Some(&route[idx + 1..]))
    } else {
        (route, None)
    };

    for segment in path_part.split('/') {
        if segment.starts_with('{') && segment.ends_with('}') {
            path_params.push(segment[1..segment.len() - 1].to_string());
        } else if segment.starts_with(':') {
            path_params.push(segment[1..].to_string());
        }
    }

    if let Some(qp) = query_part {
        for param in qp.split('&') {
            let param = param.trim();
            if !param.is_empty() {
                query_params.push(param.to_string());
            }
        }
    }

    (path_params, query_params)
}

fn build_format_path(route: &str) -> String {
    let path_part = if let Some(idx) = route.find('?') {
        &route[..idx]
    } else {
        route
    };

    let mut result = String::new();
    for segment in path_part.split('/') {
        if !result.is_empty() || path_part.starts_with('/') {
            if !result.is_empty() || segment.is_empty() {
                result.push('/');
            }
        }
        if segment.starts_with('{') && segment.ends_with('}') {
            result.push_str("{}");
        } else if segment.starts_with(':') {
            result.push_str("{}");
        } else {
            result.push_str(segment);
        }
    }

    result
}

fn is_option_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

/// Extract `#[mcp(description = "...")]` from a param's attributes.
/// Returns the description string if found.
fn extract_mcp_description(attrs: &[syn::Attribute]) -> Option<String> {
    for attr in attrs {
        if !attr.path().is_ident("mcp") {
            continue;
        }
        if let syn::Meta::List(meta_list) = &attr.meta {
            let parser = |input: ParseStream| -> syn::Result<String> {
                let pairs =
                    Punctuated::<syn::MetaNameValue, Token![,]>::parse_terminated(input)?;
                for pair in &pairs {
                    if pair.path.is_ident("description") {
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(s),
                            ..
                        }) = &pair.value
                        {
                            return Ok(s.value());
                        }
                    }
                }
                Err(input.error("expected description = \"...\""))
            };
            if let Ok(desc) = syn::parse::Parser::parse2(parser, meta_list.tokens.clone()) {
                return Some(desc);
            }
        }
    }
    None
}

/// Strip `#[mcp(...)]` attributes from a PatType, returning a new PatType.
fn strip_mcp_attrs(p: &PatType) -> PatType {
    let mut cleaned = p.clone();
    cleaned.attrs.retain(|a| !a.path().is_ident("mcp"));
    cleaned
}

fn param_ident(p: &PatType) -> &Ident {
    match &*p.pat {
        Pat::Ident(pat_ident) => &pat_ident.ident,
        other => panic!(
            "#[mcp_tool]: unsupported parameter pattern `{}`.",
            quote! { #other }
        ),
    }
}

enum ParamClassification<'a> {
    Path(&'a PatType),
    Query(&'a PatType),
    Body(&'a PatType),
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

    let route_info = function
        .attrs
        .iter()
        .find_map(parse_route_attr)
        .expect("#[mcp_tool] requires a route attribute (#[post], #[get], #[put], #[patch], #[delete])");

    let http_method = &route_info.method;

    // Collect function params
    let fn_params: Vec<&PatType> = function
        .sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(pat_type) => Some(pat_type),
            _ => None,
        })
        .collect();

    // Classify params
    let fn_param_info: Vec<ParamClassification> = fn_params
        .iter()
        .map(|p| {
            let name = param_ident(p).to_string();
            if route_info.path_params.contains(&name) {
                ParamClassification::Path(p)
            } else if route_info.query_params.contains(&name) {
                ParamClassification::Query(p)
            } else {
                ParamClassification::Body(p)
            }
        })
        .collect();

    let fn_name = &function.sig.ident;
    let impl_fn_name = format_ident!("{}_mcp_impl", fn_name);
    let handler_fn_name = format_ident!("{}_mcp_handler", fn_name);
    let vis = &function.vis;
    let return_type = &function.sig.output;

    // Generate request struct name: create_discussion → CreateDiscussionMcpRequest
    let struct_name = format_ident!(
        "{}McpRequest",
        fn_name.to_string().to_case(Case::Pascal)
    );

    // ── Build _mcp_impl params ──────────────────────────────────────

    let mut impl_params: Vec<TokenStream> = vec![quote! { mcp_secret: String }];
    let mut path_param_names: Vec<Ident> = Vec::new();
    let mut query_param_entries: Vec<(Ident, bool)> = Vec::new();
    let mut body_param_entries: Vec<Ident> = Vec::new();

    // All params for the request struct (with descriptions and cleaned types)
    let mut struct_fields: Vec<TokenStream> = Vec::new();
    // Field names for destructuring the request struct in _mcp_handler
    let mut all_field_names: Vec<Ident> = Vec::new();

    for path_param_name in &route_info.path_params {
        for info in &fn_param_info {
            if let ParamClassification::Path(p) = info {
                let ident = param_ident(p);
                if ident.to_string() == *path_param_name {
                    let cleaned = strip_mcp_attrs(p);
                    let ty = &cleaned.ty;
                    let desc = extract_mcp_description(&p.attrs);
                    let desc_attr = desc.map(|d| quote! { #[schemars(description = #d)] });
                    struct_fields.push(quote! { #desc_attr pub #ident: #ty });
                    impl_params.push(quote! { #cleaned });
                    path_param_names.push(ident.clone());
                    all_field_names.push(ident.clone());
                }
            }
        }
    }

    for info in &fn_param_info {
        if let ParamClassification::Query(p) = info {
            let ident = param_ident(p);
            let cleaned = strip_mcp_attrs(p);
            let ty = &cleaned.ty;
            let is_option = is_option_type(&p.ty);
            let desc = extract_mcp_description(&p.attrs);
            let desc_attr = desc.map(|d| quote! { #[schemars(description = #d)] });
            struct_fields.push(quote! { #desc_attr pub #ident: #ty });
            impl_params.push(quote! { #cleaned });
            query_param_entries.push((ident.clone(), is_option));
            all_field_names.push(ident.clone());
        }
    }

    for info in &fn_param_info {
        if let ParamClassification::Body(p) = info {
            let ident = param_ident(p);
            let cleaned = strip_mcp_attrs(p);
            let ty = &cleaned.ty;
            let desc = extract_mcp_description(&p.attrs);
            let desc_attr = desc.map(|d| quote! { #[schemars(description = #d)] });
            struct_fields.push(quote! { #desc_attr pub #ident: #ty });
            impl_params.push(quote! { #cleaned });
            body_param_entries.push(ident.clone());
            all_field_names.push(ident.clone());
        }
    }

    // ── Generate request struct ─────────────────────────────────────

    let request_struct = if struct_fields.is_empty() {
        quote! {}
    } else {
        quote! {
            #[cfg(feature = "server")]
            #[derive(Debug, serde::Serialize, serde::Deserialize, rmcp::schemars::JsonSchema)]
            #vis struct #struct_name {
                #(#struct_fields),*
            }
        }
    };

    // ── Generate _mcp_impl (oneshot) ────────────────────────────────

    let format_path = build_format_path(&route_info.path);

    let path_construction = if path_param_names.is_empty() {
        quote! { let mut __path = #format_path.to_string(); }
    } else {
        quote! { let mut __path = format!(#format_path, #(#path_param_names),*); }
    };

    let query_param_code = if query_param_entries.is_empty() {
        quote! {}
    } else {
        let query_pushes: Vec<TokenStream> = query_param_entries
            .iter()
            .map(|(ident, is_option)| {
                let name_str = ident.to_string();
                if *is_option {
                    quote! {
                        if let Some(ref __v) = #ident {
                            __qp.push(format!("{}={}", #name_str, urlencoding::encode(&format!("{}", __v))));
                        }
                    }
                } else {
                    quote! {
                        __qp.push(format!("{}={}", #name_str, urlencoding::encode(&format!("{}", #ident))));
                    }
                }
            })
            .collect();

        quote! {
            let mut __qp: Vec<String> = Vec::new();
            #(#query_pushes)*
            if !__qp.is_empty() {
                __path = format!("{}?{}", __path, __qp.join("&"));
            }
        }
    };

    let body_code = if body_param_entries.is_empty() {
        quote! { let __body_bytes: Option<Vec<u8>> = None; }
    } else if body_param_entries.len() == 1 {
        let ident = &body_param_entries[0];
        let name_str = ident.to_string();
        quote! {
            let __body = serde_json::json!({ #name_str: #ident });
            let __body_bytes: Option<Vec<u8>> = serde_json::to_vec(&__body).ok();
        }
    } else {
        let json_entries: Vec<TokenStream> = body_param_entries
            .iter()
            .map(|ident| {
                let name_str = ident.to_string();
                quote! { #name_str: #ident }
            })
            .collect();
        quote! {
            let __body = serde_json::json!({ #(#json_entries),* });
            let __body_bytes: Option<Vec<u8>> = serde_json::to_vec(&__body).ok();
        }
    };

    let impl_fn = quote! {
        #[cfg(feature = "server")]
        #vis async fn #impl_fn_name(#(#impl_params),*) #return_type {
            #path_construction
            #query_param_code
            #body_code
            crate::common::mcp::mcp_oneshot(#http_method, &__path, &mcp_secret, __body_bytes).await
        }
    };

    // ── Generate _mcp_handler ───────────────────────────────────────

    let handler_fn = if struct_fields.is_empty() {
        // No request struct — handler takes only mcp_secret
        quote! {
            #[cfg(feature = "server")]
            #vis async fn #handler_fn_name(
                mcp_secret: &str,
            ) -> crate::common::mcp::McpResult {
                use crate::common::mcp::IntoMcpResult;
                #impl_fn_name(mcp_secret.to_string()).await.into_mcp()
            }
        }
    } else {
        // Destructure request struct into individual params
        let field_refs: Vec<TokenStream> = all_field_names
            .iter()
            .map(|n| quote! { #n })
            .collect();

        quote! {
            #[cfg(feature = "server")]
            #vis async fn #handler_fn_name(
                mcp_secret: &str,
                req: #struct_name,
            ) -> crate::common::mcp::McpResult {
                use crate::common::mcp::IntoMcpResult;
                let #struct_name { #(#field_refs),* } = req;
                #impl_fn_name(mcp_secret.to_string(), #(#field_refs),*).await.into_mcp()
            }
        }
    };

    // ── Generate metadata constant ──────────────────────────────────

    let meta_const_name = format_ident!(
        "MCP_TOOL_META_{}",
        fn_name.to_string().to_uppercase()
    );
    let impl_fn_name_str = impl_fn_name.to_string();

    // ── Strip #[mcp(...)] from function params before emitting ──────

    for input in &mut function.sig.inputs {
        if let FnArg::Typed(pat_type) = input {
            pat_type.attrs.retain(|a| !a.path().is_ident("mcp"));
        }
    }

    // ── Emit everything ─────────────────────────────────────────────

    let output = quote! {
        #request_struct

        #impl_fn

        #handler_fn

        /// MCP tool metadata generated by `#[mcp_tool]`.
        #[cfg(feature = "server")]
        #vis const #meta_const_name: (&str, &str, &str) = (#tool_name, #tool_description, #impl_fn_name_str);

        #function
    };

    output
}
