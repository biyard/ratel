//! Proc-macro implementations for `#[get]`, `#[post]`, `#[put]`, `#[patch]`,
//! `#[delete]` attributes that wrap dioxus-fullstack's macros and add a
//! reqwest-based client stub for the `tauri-web` feature.
//!
//! Under `cfg(not(tauri-web))`, the original `#[::dioxus::fullstack::<method>]`
//! attribute is re-attached so dioxus generates server + browser-client code
//! exactly as before. Under `cfg(tauri-web)`, the original function body is
//! dropped on the client side and replaced by a small stub that calls
//! `crate::common::fullstack::server_fn::<method>` with a URL built from the
//! macro's path literal and the handler's own arguments.
//!
//! Attribute syntax matches dioxus's:
//!
//!     #[post("/api/posts/{post_id}/comments?after", user: User)]
//!     pub async fn add_comment(
//!         post_id: FeedPartition,
//!         after: Option<String>,
//!         req: AddCommentRequest,
//!     ) -> Result<Comment> { ... }
//!
//! - `{post_id}` in the path = path segment substitution; matched by name
//!   against the function's args.
//! - `?after` in the path = query string; matched by name against args.
//!   Multiple query keys separate with `&` (e.g. `?a&b`).
//! - `user: User`, `_x: Foo`, `role: SpaceUserRole`, etc. = server-side
//!   extractor params. Stripped from the client stub (server fills them
//!   from request context).
//! - Remaining args (not path, not query, not extractor) for POST/PUT/PATCH
//!   become the JSON body. Exactly one such arg is expected.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use std::collections::HashSet;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, FnArg, Ident, ItemFn, LitStr, Pat, PatType, Token, Type};

/// `#[get("/path", extractor: T, ...)]` — parsed attribute args.
struct RouteAttr {
    path: LitStr,
    /// Names of server-only extractor params (e.g. `user`, `role`, `_space`).
    /// Stripped from client stubs.
    extractors: HashSet<String>,
}

impl Parse for RouteAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path: LitStr = input.parse()?;
        let mut extractors = HashSet::new();
        // Accept three shapes used across biyard/asset and biyard/ratel:
        //   #[get("/path")]                                  — no extractors
        //   #[get("/path", name)]                            — name only
        //   #[get("/path", name: Type, name2: Type2, ...)]   — name + type
        //
        // The type half is meaningful only on the server side via
        // `dioxus::fullstack`'s own parser. The client-stub branch
        // emitted by this macro only needs the extractor *names* so
        // they can be stripped from the stub signature.
        while !input.is_empty() {
            // Optional leading `,` between args (path → first extractor,
            // and between successive extractors). Tolerated as optional
            // so a stray trailing comma doesn't fail the parse.
            let _ = input.parse::<Token![,]>();
            if input.is_empty() {
                break;
            }
            let name: Ident = input.parse()?;
            extractors.insert(name.to_string());
            // Optional `: <Type>` annotation — consumed and discarded.
            if input.peek(Token![:]) {
                let _: Token![:] = input.parse()?;
                let _: Type = input.parse()?;
            }
        }
        Ok(RouteAttr { path, extractors })
    }
}

/// Detect `Option<T>` syntactically by looking at the last segment of the
/// type path. Catches `Option<_>`, `std::option::Option<_>`,
/// `::std::option::Option<_>`. Conservative: anything else returns false.
fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(tp) = ty {
        if let Some(seg) = tp.path.segments.last() {
            return seg.ident == "Option";
        }
    }
    false
}

/// If `ty` is `Form<T>` (any path), return the inner `T`. Otherwise None.
/// Matches `Form<...>`, `dioxus::dioxus_fullstack::Form<...>`,
/// `dioxus_fullstack::Form<...>`, etc.
fn unwrap_form_type(ty: &Type) -> Option<Type> {
    if let Type::Path(tp) = ty {
        if let Some(seg) = tp.path.segments.last() {
            if seg.ident == "Form" {
                if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                    if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                        return Some(inner.clone());
                    }
                }
            }
        }
    }
    None
}

/// Parse `/api/posts/{id}/comments?after&before` into:
///  - format-string template `"/api/posts/{}/comments"` (path piece only)
///  - path placeholder names in order: `["id"]`
///  - query placeholder names: `["after", "before"]`
///
/// Both `{name}` (axum 0.8+ / dioxus-fullstack) and `:name` (axum 0.7 style)
/// placeholders are accepted. A `:name` segment begins immediately after a
/// `/` and runs until the next `/` or end of path.
fn parse_path(path: &str) -> (String, Vec<String>, Vec<String>) {
    let (path_part, query_part) = match path.find('?') {
        Some(i) => (&path[..i], Some(&path[i + 1..])),
        None => (path, None),
    };

    let mut path_args = Vec::new();
    let mut template = String::new();
    let mut chars = path_part.chars().peekable();
    let mut prev = '\0';
    while let Some(c) = chars.next() {
        if c == '{' {
            let mut name = String::new();
            while let Some(&nc) = chars.peek() {
                if nc == '}' {
                    chars.next();
                    break;
                }
                name.push(nc);
                chars.next();
            }
            path_args.push(name);
            template.push_str("{}");
            prev = '}';
        } else if c == ':' && (prev == '\0' || prev == '/') {
            // `:name` placeholder — read until next '/' or end.
            let mut name = String::new();
            while let Some(&nc) = chars.peek() {
                if nc == '/' {
                    break;
                }
                name.push(nc);
                chars.next();
            }
            prev = name.chars().last().unwrap_or(':');
            path_args.push(name);
            template.push_str("{}");
        } else if c == '}' {
            // unbalanced — keep literal
            template.push(c);
            prev = c;
        } else {
            template.push(c);
            prev = c;
        }
    }

    let query_args = query_part
        .map(|q| {
            q.split('&')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect()
        })
        .unwrap_or_default();

    (template, path_args, query_args)
}

pub fn server_fn_impl(method: &str, attr: TokenStream, item: TokenStream) -> TokenStream {
    let route = parse_macro_input!(attr as RouteAttr);
    let func = parse_macro_input!(item as ItemFn);

    let fn_vis = &func.vis;
    let fn_sig = &func.sig;
    let fn_name = &fn_sig.ident;
    let fn_generics = &fn_sig.generics;
    let fn_output = &fn_sig.output;
    let fn_attrs = &func.attrs;

    let (path_template, path_args, query_args) = parse_path(&route.path.value());
    let path_arg_set: HashSet<&String> = path_args.iter().collect();
    let query_arg_set: HashSet<&String> = query_args.iter().collect();

    // Collect typed args from the original signature.
    let typed_args: Vec<(&Ident, &Type)> = fn_sig
        .inputs
        .iter()
        .filter_map(|input| match input {
            FnArg::Typed(PatType { pat, ty, .. }) => {
                if let Pat::Ident(pi) = pat.as_ref() {
                    Some((&pi.ident, ty.as_ref()))
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect();

    // Args that survive on the client stub = everything except server
    // extractors.
    let client_args: Vec<(&Ident, &Type)> = typed_args
        .iter()
        .filter(|(name, _)| !route.extractors.contains(&name.to_string()))
        .copied()
        .collect();

    // Path-substituted args, by position in the template.
    let path_idents: Vec<&Ident> = path_args
        .iter()
        .filter_map(|name| {
            client_args
                .iter()
                .find(|(n, _)| n.to_string() == *name)
                .map(|(n, _)| *n)
        })
        .collect();

    // Query string args.
    let query_idents: Vec<&Ident> = query_args
        .iter()
        .filter_map(|name| {
            client_args
                .iter()
                .find(|(n, _)| n.to_string() == *name)
                .map(|(n, _)| *n)
        })
        .collect();
    let query_names: Vec<String> = query_idents.iter().map(|i| i.to_string()).collect();

    // Body args = client args that aren't path or query.
    let body_idents: Vec<&Ident> = client_args
        .iter()
        .filter(|(n, _)| {
            let s = n.to_string();
            !path_arg_set.contains(&s) && !query_arg_set.contains(&s)
        })
        .map(|(n, _)| *n)
        .collect();

    // Function-arg declarations for the stub signature (same as original
    // minus extractors). If a body arg is `Form<T>`, the stub takes `T`
    // directly — we don't want to leak dioxus's Form wrapper into the
    // tauri-web client path (it isn't Serialize).
    let stub_inputs: Vec<TokenStream2> = client_args
        .iter()
        .map(|(name, ty)| {
            if let Some(inner) = unwrap_form_type(ty) {
                quote! { #name: #inner }
            } else {
                quote! { #name: #ty }
            }
        })
        .collect();

    // --- Body for the tauri-web stub ----------------------------------------

    let path_format = if path_idents.is_empty() {
        quote! { let __path: ::std::string::String = #path_template.to_string(); }
    } else {
        // Go through `to_url_value` (serde-based) so any `Serialize` value
        // works, including enums with `#[serde(rename_all = ...)]`, without
        // requiring `Display`. Then percent-encode each segment.
        let tpl = LitStr::new(&path_template, route.path.span());
        quote! {
            let __path: ::std::string::String = format!(
                #tpl,
                #( ::urlencoding::encode(
                    &crate::common::fullstack::server_fn::to_url_value(&#path_idents)
                ) ),*
            );
        }
    };

    let query_attach = if query_idents.is_empty() {
        quote! { let __url: ::std::string::String = __path; }
    } else {
        // Build `?k=v&k2=v2`, skipping None values. We detect `Option<T>`
        // by inspecting the type syntactically so we can emit an
        // `if let Some(v) = ...` branch for those and an unconditional
        // push for non-Option args. Rendering uses Display via to_string().
        let pushers = query_idents.iter().zip(query_names.iter()).map(|(ident, name)| {
            // Look up the type for this ident.
            let ty = client_args
                .iter()
                .find(|(n, _)| n.to_string() == ident.to_string())
                .map(|(_, t)| *t);
            let is_option = ty.map(is_option_type).unwrap_or(false);
            if is_option {
                quote! {
                    if let ::std::option::Option::Some(v) = &#ident {
                        if __has_q { __url.push('&'); } else { __url.push('?'); __has_q = true; }
                        __url.push_str(#name);
                        __url.push('=');
                        __url.push_str(&::urlencoding::encode(
                            &crate::common::fullstack::server_fn::to_url_value(v)
                        ));
                    }
                }
            } else {
                quote! {
                    {
                        if __has_q { __url.push('&'); } else { __url.push('?'); __has_q = true; }
                        __url.push_str(#name);
                        __url.push('=');
                        __url.push_str(&::urlencoding::encode(
                            &crate::common::fullstack::server_fn::to_url_value(&#ident)
                        ));
                    }
                }
            }
        });
        quote! {
            let mut __url = __path;
            let mut __has_q = false;
            #( #pushers )*
        }
    };

    let send_call = match method {
        "GET" | "DELETE" => {
            let fn_name = format_ident!("{}", method.to_lowercase());
            quote! {
                crate::common::fullstack::server_fn::#fn_name(&__url)
                .await
                .map_err(::std::convert::Into::into)
            }
        }
        "POST" | "PUT" | "PATCH" => {
            let fn_name = format_ident!("{}", method.to_lowercase());
            // dioxus-fullstack's server-side request decoder expects the
            // body JSON to be an object keyed by the handler's argument
            // names, e.g. for `fn handler(req: T)` it parses
            // `{"req": <T>}`. The auto-generated args struct fails to
            // deserialize a bare `T`. Wrap accordingly.
            //
            // If none, send `&()` — an empty JSON object isn't required
            // (dioxus accepts an empty body for zero-arg POSTs).
            if let Some(body) = body_idents.first() {
                let body_name = LitStr::new(&body.to_string(), body.span());
                quote! {
                    crate::common::fullstack::server_fn::#fn_name(
                        &__url,
                        &::serde_json::json!({ #body_name: &#body }),
                    )
                    .await
                    .map_err(::std::convert::Into::into)
                }
            } else {
                quote! {
                    crate::common::fullstack::server_fn::#fn_name(&__url, &())
                .await
                .map_err(::std::convert::Into::into)
                }
            }
        }
        _ => unreachable!("unsupported method {method}"),
    };

    let tauri = quote! {
        #(#fn_attrs)*
        #[allow(unused_variables, unused_mut)]
        #fn_vis async fn #fn_name #fn_generics ( #( #stub_inputs ),* ) #fn_output {
            #path_format
            #query_attach
            #send_call
        }
    };

    quote! {
        #tauri
    }
    .into()
}
