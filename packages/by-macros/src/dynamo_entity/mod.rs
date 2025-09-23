use std::collections::HashMap;

use convert_case::Casing;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, Attribute, Data, DataEnum, DataStruct, DeriveInput, Fields, Ident, Type,
};

use crate::write_file;

#[derive(Default, Clone, Debug)]
struct StructCfg {
    table: String,        // "main"
    table_prefix: String, // "DYNAMO_TABLE_PREFIX"
    result_ty: String,    // "crate::Result"
    error_ctor: String,   // "crate::Error::DynamoDbError"
    pk_name: String,
    sk_name: Option<String>,
    indice: Vec<StructIndexCfg>,
}

#[derive(Default, Clone, Debug)]
struct StructIndexCfg {
    pk_prefix: Option<String>,
    sk_prefix: Option<String>,
    index: String,
    name: String,
    enable_sk: bool,
}

#[derive(Clone, Debug)]
struct IndexInfo {
    #[allow(dead_code)]
    name: Option<String>, // "find_by_email_and_code"
    base_index_name: String, // "gsi1"
    pk: bool,                // "gsi1_pk"
    #[allow(dead_code)]
    sk: bool, // "gsi1_sk"
    prefix: Option<String>,  // optional prefix for pk
}

#[derive(Clone, Debug)]
struct FieldInfo {
    ident: Ident,
    #[allow(dead_code)]
    ty: Type,
    #[allow(dead_code)]
    is_pk: bool,
    #[allow(dead_code)]
    is_sk: bool,
    // For index mapping:
    // e.g., index="gsi1", pk=true => produce attr "gsi1_pk" with optional "prefix"
    //       index="gsi1", sk=true => produce attr "gsi1_sk"
    indice: Vec<IndexInfo>,
}

impl FieldInfo {
    pub fn is_option(&self) -> bool {
        use syn::{Type, TypePath};
        match &self.ty {
            Type::Path(TypePath { path, .. }) => path
                .segments
                .last()
                .map(|seg| seg.ident == "Option")
                .unwrap_or(false),
            _ => false,
        }
    }
    // pub fn is_option(&self) -> bool {
    //     self.ty
    //         .to_token_stream()
    //         .to_string()
    //         .starts_with("Option <")
    // }
    pub fn native_type(&self) -> Ident {
        let ty_str = self.ty.to_token_stream().to_string();
        let ty_str = if self.is_option() {
            ty_str
                .trim_start_matches("Option <")
                .trim_end_matches('>')
                .trim()
                .to_string()
        } else {
            ty_str
        };

        Ident::new(&ty_str, proc_macro2::Span::call_site())
    }

    pub fn is_number_type(&self) -> bool {
        let ty_str = self.ty.to_token_stream().to_string();
        matches!(
            ty_str.as_str(),
            "i8" | "i16"
                | "i32"
                | "i64"
                | "i128"
                | "u8"
                | "u16"
                | "u32"
                | "u64"
                | "u128"
                | "f32"
                | "f64"
        )
    }
}

fn parse_struct_cfg(attrs: &[Attribute]) -> StructCfg {
    let mut cfg = StructCfg {
        table: "main".into(),
        table_prefix: env!("DYNAMO_TABLE_PREFIX").into(),
        result_ty: "std::result::Result".into(),
        // FIXME: rename after finishing migration
        error_ctor: "crate::Error2".into(),
        pk_name: "pk".into(),
        sk_name: Some("sk".into()),
        indice: vec![],
    };

    for attr in attrs {
        if !attr.path().is_ident("dynamo") {
            continue;
        }
        let mut index_cfg = StructIndexCfg {
            pk_prefix: None,
            sk_prefix: None,
            index: String::new(),
            name: String::new(),
            enable_sk: false,
        };

        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("table") {
                if let Ok(value) = meta.value() {
                    if let Ok(s) = value.parse::<syn::LitStr>() {
                        cfg.table = s.value();
                    }
                }
            } else if meta.path.is_ident("result") {
                if let Ok(value) = meta.value() {
                    if let Ok(s) = value.parse::<syn::LitStr>() {
                        cfg.result_ty = s.value();
                    }
                }
            } else if meta.path.is_ident("error_ctor") {
                if let Ok(value) = meta.value() {
                    if let Ok(s) = value.parse::<syn::LitStr>() {
                        cfg.error_ctor = s.value();
                    }
                }
            } else if meta.path.is_ident("pk_name") {
                if let Ok(value) = meta.value() {
                    if let Ok(s) = value.parse::<syn::LitStr>() {
                        cfg.pk_name = s.value();
                    }
                }
            } else if meta.path.is_ident("sk_name") {
                if let Ok(value) = meta.value() {
                    let v = value.to_string();
                    if v.is_empty() || &v == "None" || &v == "none" {
                        cfg.sk_name = None;
                    } else if let Ok(s) = value.parse::<syn::LitStr>() {
                        cfg.sk_name = Some(s.value());
                    }
                }
            } else if meta.path.is_ident("pk_prefix") {
                if let Ok(value) = meta.value() {
                    if let Ok(s) = value.parse::<syn::LitStr>() {
                        index_cfg.pk_prefix = Some(s.value());
                    }
                }
            } else if meta.path.is_ident("sk_prefix") {
                if let Ok(value) = meta.value() {
                    if let Ok(s) = value.parse::<syn::LitStr>() {
                        index_cfg.sk_prefix = Some(s.value());
                    }
                }
            } else if meta.path.is_ident("index") {
                if let Ok(value) = meta.value() {
                    if let Ok(s) = value.parse::<syn::LitStr>() {
                        index_cfg.index = s.value();
                        if index_cfg.name.is_empty() {
                            index_cfg.name = format!("find_by_{}", s.value());
                        }
                    }
                }
            } else if meta.path.is_ident("name") {
                if let Ok(value) = meta.value() {
                    if let Ok(s) = value.parse::<syn::LitStr>() {
                        index_cfg.name = s.value();
                    }
                }
            } else if meta.path.is_ident("enable_sk") {
                index_cfg.enable_sk = true;
            }

            Ok(())
        });

        if !index_cfg.index.is_empty() {
            cfg.indice.push(index_cfg);
        }
    }
    cfg
}

fn parse_fields(ds: &DataStruct, cfg: &StructCfg) -> (Vec<FieldInfo>, HashMap<String, String>) {
    let mut out = vec![];
    let pk = &cfg.pk_name;
    let sk = cfg.sk_name.clone().unwrap_or_default();
    let mut indice_fn: HashMap<String, String> = HashMap::new();

    if let Fields::Named(named) = &ds.fields {
        for f in &named.named {
            let ident = f.ident.clone().unwrap();
            let mut info = FieldInfo {
                ident: ident.clone(),
                ty: f.ty.clone(),
                is_pk: ident == pk,
                is_sk: ident == &sk,
                indice: vec![],
            };

            for attr in &f.attrs {
                if !attr.path().is_ident("dynamo") {
                    continue;
                }
                let mut fn_name: Option<String> = None;
                let mut idx_name: Option<String> = None;
                let mut idx_pk = false;
                let mut idx_sk = false;
                let mut idx_prefix: Option<String> = None;

                let _ = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("pk") {
                        idx_pk = true;
                    } else if meta.path.is_ident("sk") {
                        idx_sk = true;
                    } else if meta.path.is_ident("index") {
                        if let Ok(value) = meta.value() {
                            if let Ok(s) = value.parse::<syn::LitStr>() {
                                idx_name = Some(s.value());
                            }
                        }
                    } else if meta.path.is_ident("prefix") {
                        if let Ok(value) = meta.value() {
                            if let Ok(s) = value.parse::<syn::LitStr>() {
                                idx_prefix = Some(s.value());
                            }
                        }
                    } else if meta.path.is_ident("name") {
                        if let Ok(value) = meta.value() {
                            if let Ok(s) = value.parse::<syn::LitStr>() {
                                fn_name = Some(s.value());
                            }
                        }
                    }

                    Ok(())
                });

                if let Some(ref fn_name) = fn_name {
                    indice_fn.insert(
                        idx_name
                            .clone()
                            .expect("`name` must be paired with `index`"),
                        fn_name.clone(),
                    );
                }

                if idx_name.is_some() && (idx_pk || idx_sk) {
                    info.indice.push(IndexInfo {
                        name: fn_name,
                        base_index_name: idx_name.unwrap(),
                        pk: idx_pk,
                        sk: idx_sk,
                        prefix: idx_prefix.clone(),
                    });
                }
            }
            out.push(info);
        }
    }

    (out, indice_fn)
}

fn generate_key_composers(fields: &Vec<FieldInfo>) -> Vec<proc_macro2::TokenStream> {
    let mut out = vec![];

    for f in fields.iter() {
        for idx in f.indice.iter() {
            let idx_base = idx.base_index_name.clone();
            let cname = Ident::new(
                &format!("compose_{}_{}", idx_base, if idx.pk { "pk" } else { "sk" }),
                proc_macro2::Span::call_site(),
            );

            let token = if let Some(ref prefix) = idx.prefix {
                quote! {
                    pub fn #cname(key: impl std::fmt::Display) -> String {
                        format!("{}#{}", #prefix, key)
                    }
                }
            } else {
                quote! {
                    pub fn #cname(key: impl std::fmt::Display) -> String {
                        key.to_string()
                    }
                }
            };

            out.push(token);
        }
    }

    out.into()
}

fn generate_updater(
    ident: &Ident,
    s_cfg: &StructCfg,
    fields: &Vec<FieldInfo>,
) -> proc_macro2::TokenStream {
    let st_name = ident.to_string();
    let updater_name = format!("{}Updater", st_name.to_case(convert_case::Case::Pascal));
    let updater_ident = Ident::new(&updater_name, proc_macro2::Span::call_site());

    let pk_field = syn::LitStr::new(&s_cfg.pk_name, proc_macro2::Span::call_site());
    let sk_field = if let Some(ref sk_name) = s_cfg.sk_name {
        syn::LitStr::new(sk_name, proc_macro2::Span::call_site())
    } else {
        syn::LitStr::new("", proc_macro2::Span::call_site())
    };

    let key_fields = if s_cfg.sk_name.is_some() {
        quote! {
            k: std::collections::HashMap<String, aws_sdk_dynamodb::types::AttributeValue>,
        }
    } else {
        quote! {
            k: std::collections::HashMap<String, aws_sdk_dynamodb::types::AttributeValue>,
        }
    };

    let sk_param = if s_cfg.sk_name.is_some() {
        quote! { sk: impl std::fmt::Display, }
    } else {
        quote! {}
    };

    let sk_key = if s_cfg.sk_name.is_some() {
        quote! {
            (
                #sk_field.to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(sk.to_string()),
            ),
        }
    } else {
        quote! {}
    };

    let mut update_fns = vec![];

    for f in fields.iter() {
        if f.is_pk || f.is_sk {
            continue;
        }
        let var_name = &f.ident;
        let var_ty = f.native_type();

        let fn_setter = Ident::new(
            &format!(
                "with_{}",
                var_name.to_string().to_case(convert_case::Case::Snake)
            ),
            proc_macro2::Span::call_site(),
        );
        let fn_increase = Ident::new(
            &format!(
                "increase_{}",
                var_name.to_string().to_case(convert_case::Case::Snake)
            ),
            proc_macro2::Span::call_site(),
        );
        let fn_decrease = Ident::new(
            &format!(
                "decrease_{}",
                var_name.to_string().to_case(convert_case::Case::Snake)
            ),
            proc_macro2::Span::call_site(),
        );
        let fn_remove = Ident::new(
            &format!(
                "remove_{}",
                var_name.to_string().to_case(convert_case::Case::Snake)
            ),
            proc_macro2::Span::call_site(),
        );
        // Build additional GSI updates for this field (PUT on setter)
        let mut gsi_put_updates: Vec<proc_macro2::TokenStream> = vec![];
        for idx in f.indice.iter() {
            let idx_base_snake = &idx.base_index_name;
            let composer_ident = Ident::new(
                &format!(
                    "compose_{}_{}",
                    idx_base_snake,
                    if idx.pk { "pk" } else { "sk" }
                ),
                proc_macro2::Span::call_site(),
            );
            let idx_key_name = syn::LitStr::new(
                &format!(
                    "{}_{}",
                    idx.base_index_name,
                    if idx.pk { "pk" } else { "sk" }
                ),
                proc_macro2::Span::call_site(),
            );

            gsi_put_updates.push(quote! {
                self.m.insert(
                    #idx_key_name.to_string(),
                    aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
                        .value(aws_sdk_dynamodb::types::AttributeValue::S(
                            #ident::#composer_ident(#var_name.clone())
                        ))
                        .action(aws_sdk_dynamodb::types::AttributeAction::Put)
                        .build(),
                );
            });
        }

        // Build additional GSI updates for this field (DELETE on remove)
        let mut gsi_delete_updates: Vec<proc_macro2::TokenStream> = vec![];
        for idx in f.indice.iter() {
            let idx_key_name = syn::LitStr::new(
                &format!(
                    "{}_{}",
                    idx.base_index_name,
                    if idx.pk { "pk" } else { "sk" }
                ),
                proc_macro2::Span::call_site(),
            );
            gsi_delete_updates.push(quote! {
                self.m.insert(
                    #idx_key_name.to_string(),
                    aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
                        .action(aws_sdk_dynamodb::types::AttributeAction::Delete)
                        .build(),
                );
            });
        }

        // setter
        update_fns.push(quote! {
            pub fn #fn_setter(mut self, #var_name: #var_ty) -> Self {
                let v = serde_dynamo::to_attribute_value(&#var_name)
                    .expect("failed to serialize field");
                let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
                    .value(v)
                    .action(aws_sdk_dynamodb::types::AttributeAction::Put)
                    .build();
                self.m.insert(stringify!(#var_name).to_string(), v);
                // Update derived GSI attributes for this field
                #(#gsi_put_updates)*
                self
            }
        });
        // remove
        update_fns.push(quote! {
            pub fn #fn_remove(mut self) -> Self {
                let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
                    .action(aws_sdk_dynamodb::types::AttributeAction::Delete)
                    .build();
                self.m.insert(stringify!(#var_name).to_string(), v);
                // Remove derived GSI attributes for this field
                #(#gsi_delete_updates)*
                self
            }
        });

        if !f.is_number_type() {
            continue;
        }

        // increase
        update_fns.push(quote! {
            pub fn #fn_increase(mut self, by: i64) -> Self {
                let v = serde_dynamo::to_attribute_value(by)
                    .expect("failed to serialize field");
                let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
                    .value(v)
                    .action(aws_sdk_dynamodb::types::AttributeAction::Add)
                    .build();
                self.m.insert(stringify!(#var_name).to_string(), v);
                self
            }
        });
        // decrease
        update_fns.push(quote! {
            pub fn #fn_decrease(mut self, by: i64) -> Self {
                let v = serde_dynamo::to_attribute_value(-by)
                    .expect("failed to serialize field");
                let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
                    .value(v)
                    .action(aws_sdk_dynamodb::types::AttributeAction::Add)
                    .build();
                self.m.insert(stringify!(#var_name).to_string(), v);
                self
            }
        });
    }
    let err_ctor: syn::Path = syn::parse_str(&s_cfg.error_ctor).unwrap();
    let result_ty: syn::Type = syn::parse_str(&s_cfg.result_ty).unwrap();

    quote! {
        pub struct #updater_ident {
            #key_fields
            m: std::collections::HashMap<String, aws_sdk_dynamodb::types::AttributeValueUpdate>,
        }

        impl #ident {
            pub fn updater(pk: impl std::fmt::Display, #sk_param) -> #updater_ident {
                let k = std::collections::HashMap::from([
                    (
                        #pk_field.to_string(),
                        aws_sdk_dynamodb::types::AttributeValue::S(pk.to_string()),
                    ),
                    #sk_key
                ]);

                #updater_ident {
                    m: std::collections::HashMap::new(),
                    k,
                }
            }
        }

        impl #updater_ident {
            #(#update_fns)*

            pub async fn execute(
                self,
                cli: &aws_sdk_dynamodb::Client,
            ) -> #result_ty <(), #err_ctor> {
                cli.update_item()
                    .table_name(#ident::table_name())
                    .set_key(Some(self.k))
                    .set_attribute_updates(Some(self.m))
                    .send()
                    .await
                    .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

                Ok(())
            }
        }
    }
}

fn generate_struct_impl(
    ident: Ident,
    ds: &DataStruct,
    s_cfg: StructCfg,
) -> proc_macro2::TokenStream {
    let st_name = ident.to_string();

    let (fields, indice_fn) = parse_fields(ds, &s_cfg);

    let table_suffix = s_cfg.table.clone();
    let table_prefix = s_cfg.table_prefix.clone();
    let result_ty: syn::Type = syn::parse_str(&s_cfg.result_ty).unwrap();
    let err_ctor: syn::Path = syn::parse_str(&s_cfg.error_ctor).unwrap();
    let table_lit_str = syn::LitStr::new(
        &format!("{}-{}", table_prefix, table_suffix),
        proc_macro2::Span::call_site(),
    );

    let pk_field_name = syn::LitStr::new(&s_cfg.pk_name, proc_macro2::Span::call_site());
    let sk_field_method = if let Some(ref sk_name) = s_cfg.sk_name {
        let sk_name = syn::LitStr::new(sk_name, proc_macro2::Span::call_site());

        quote! { Some(#sk_name) }
    } else {
        quote! { None }
    };

    let sk_param = if s_cfg.sk_name.is_some() {
        quote! { sk: Option<impl std::fmt::Display>, }
    } else {
        quote! {}
    };

    let sk_condition = if s_cfg.sk_name.is_some() {
        quote! {
            if let Some(sk) = sk {
                req = req.key(
                    Self::sk_field().expect("sk field is required"),
                    aws_sdk_dynamodb::types::AttributeValue::S(format!("{}", sk)),
                );
            }
        }
    } else {
        quote! {}
    };

    let st_query_option = generate_query_option(&st_name, &s_cfg);
    let query_fn = generate_query_fn(&st_name, &s_cfg, &fields, &indice_fn);
    let key_composers = generate_key_composers(&fields);
    let updater = generate_updater(&ident, &s_cfg, &fields);

    let out = quote! {
        #st_query_option

        #query_fn

        #updater


        impl #ident {
            #(#key_composers)*

            pub fn table_name() -> &'static str {
                #table_lit_str
            }

            pub fn pk_field() -> &'static str { #pk_field_name }
            pub fn sk_field() -> Option<&'static str> {
                #sk_field_method
            }

            pub async fn create(
                &self,
                cli: &aws_sdk_dynamodb::Client,
            ) -> #result_ty <(), #err_ctor> {
                let item = serde_dynamo::to_item(self)?;

                let item = self.indexed_fields(item);

                cli.put_item()
                    .table_name(Self::table_name())
                    .set_item(Some(item))
                    .send()
                    .await.map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

                Ok(())
            }

            pub async fn get(
                cli: &aws_sdk_dynamodb::Client,
                pk: impl std::fmt::Display,
                #sk_param
            ) -> #result_ty <Option<Self>, #err_ctor> {
                let mut req = cli
                    .get_item()
                    .table_name(Self::table_name())
                    .key(
                        Self::pk_field(),
                        aws_sdk_dynamodb::types::AttributeValue::S(pk.to_string()),
                    );

                #sk_condition

                let item = req
                    .send()
                    .await
                    .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

                if let Some(item) = item.item {
                    let ev: Self = serde_dynamo::from_item(item)?;
                    Ok(Some(ev))
                } else {
                    Ok(None)
                }
            }

            pub async fn delete(
                cli: &aws_sdk_dynamodb::Client,
                pk: impl std::fmt::Display,
                #sk_param
            ) -> #result_ty <(), #err_ctor> {
                let mut req = cli.delete_item().table_name(Self::table_name()).key(
                    Self::pk_field(),
                    aws_sdk_dynamodb::types::AttributeValue::S(pk.to_string()),
                );

                #sk_condition

                req.send()
                    .await
                    .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

                Ok(())
            }

        }
    };

    out.into()
}

fn generate_index_fn_for_enum(s_cfg: &StructCfg) -> Vec<proc_macro2::TokenStream> {
    let mut out = vec![];
    let result_ty: syn::Type = syn::parse_str(&s_cfg.result_ty).unwrap();
    let err_ctor: syn::Path = syn::parse_str(&s_cfg.error_ctor).unwrap();
    let table_name = syn::LitStr::new(
        &format!("{}-{}", s_cfg.table_prefix, s_cfg.table),
        proc_macro2::Span::call_site(),
    );

    for idx in s_cfg.indice.iter() {
        let fn_name = format!("{}", idx.name.to_case(convert_case::Case::Snake));
        let fn_ident = Ident::new(&fn_name, proc_macro2::Span::call_site());
        let idx_base = idx.index.clone();
        let pk_field = format!("{}_pk", idx_base);
        let sk_field = format!("{}_sk", idx_base);

        let pk_field_lit = syn::LitStr::new(&pk_field, proc_macro2::Span::call_site());
        let sk_field_lit = syn::LitStr::new(&sk_field, proc_macro2::Span::call_site());
        let idx_ident = syn::LitStr::new(
            &format!("{}-index", idx.index),
            proc_macro2::Span::call_site(),
        );

        let pk_param = if idx.pk_prefix.is_some() {
            quote! { pk: impl std::fmt::Display, }
        } else {
            quote! { pk: impl std::fmt::Display, }
        };

        let sk_param = if idx.enable_sk || idx.sk_prefix.is_some() {
            quote! { sk: Option<impl std::fmt::Display>, }
        } else {
            quote! {}
        };

        let pk_value = if let Some(ref prefix) = idx.pk_prefix {
            let prefix = syn::LitStr::new(prefix, proc_macro2::Span::call_site());
            quote! { aws_sdk_dynamodb::types::AttributeValue::S(format!("{}#{}", #prefix, pk)), }
        } else {
            quote! { aws_sdk_dynamodb::types::AttributeValue::S(format!("{}", pk)), }
        };

        let sk_condition = if let Some(ref prefix) = idx.sk_prefix.clone() {
            let prefix = syn::LitStr::new(prefix, proc_macro2::Span::call_site());

            quote! {
                if let Some(sk) = sk {
                    key_condition.push_str(" AND begins_with(#sk, :sk)");
                    req = req
                        .expression_attribute_names("#sk", #sk_field_lit)
                        .expression_attribute_values(
                            ":sk",
                            aws_sdk_dynamodb::types::AttributeValue::S(format!("{}#{}", #prefix, sk)),
                        );
                }
            }
        } else if idx.enable_sk {
            quote! {
                if let Some(sk) = sk {
                    key_condition.push_str(" AND begins_with(#sk, :sk)");
                    req = req
                        .expression_attribute_names("#sk", #sk_field_lit)
                        .expression_attribute_values(
                            ":sk",
                            aws_sdk_dynamodb::types::AttributeValue::S(format!("{}", sk)),
                        );
                }
            }
        } else {
            quote! {}
        };

        out.push(quote! {
            pub async fn #fn_ident(
                cli: &aws_sdk_dynamodb::Client,
                #pk_param
                #sk_param
            ) -> #result_ty <Vec<Self>, #err_ctor> {
                let mut key_condition = String::from("#pk = :pk");
                let mut req = cli
                    .query()
                    .table_name(#table_name)
                    .index_name(#idx_ident)
                    .expression_attribute_names("#pk", #pk_field_lit)
                    .expression_attribute_values(
                        ":pk",
                        #pk_value
                    );

                #sk_condition

                let resp = req
                    .key_condition_expression(key_condition)
                    .send()
                    .await
                    .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

                let items = resp
                    .items
                    .unwrap_or_default()
                    .into_iter()
                    .map(|item| serde_dynamo::from_item(item).expect("failed to parse item"))
                    .collect();

                Ok(items)
            }
        });
    }

    out.into()
}

fn generate_enum_impl(ident: Ident, _ds: &DataEnum, s_cfg: StructCfg) -> proc_macro2::TokenStream {
    let table_suffix = s_cfg.table.clone();
    let table_prefix = s_cfg.table_prefix.clone();
    let result_ty: syn::Type = syn::parse_str(&s_cfg.result_ty).unwrap();
    let err_ctor: syn::Path = syn::parse_str(&s_cfg.error_ctor).unwrap();
    let table_lit_str = syn::LitStr::new(
        &format!("{}-{}", table_prefix, table_suffix),
        proc_macro2::Span::call_site(),
    );

    let pk_field_name = syn::LitStr::new(&s_cfg.pk_name, proc_macro2::Span::call_site());
    let idx_fn = generate_index_fn_for_enum(&s_cfg);

    quote! {
        impl #ident {
            #(#idx_fn)*

            pub fn table_name() -> &'static str {
                #table_lit_str
            }

            pub fn pk_field() -> &'static str { #pk_field_name }


            pub async fn query(
                cli: &aws_sdk_dynamodb::Client,
                pk: impl std::fmt::Display,
            ) -> #result_ty <Vec<#ident>, #err_ctor> {
                let resp = cli
                    .query()
                    .table_name(#table_lit_str)
                    .key_condition_expression("#pk = :pk")
                    .expression_attribute_names("#pk", #pk_field_name)
                    .expression_attribute_values(
                        ":pk",
                        aws_sdk_dynamodb::types::AttributeValue::S(pk.to_string()),
                    )
                    .send()
                    .await
                    .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

                let items = resp
                    .items
                    .unwrap_or_default()
                    .into_iter()
                    .map(|item| serde_dynamo::from_item(item).expect("failed to parse item"))
                    .collect();

                Ok(items)
            }
        }
    }
}

pub fn dynamo_entity_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident.clone();
    let s_cfg = parse_struct_cfg(&input.attrs);

    let out = match &input.data {
        Data::Struct(ds) => generate_struct_impl(ident.clone(), ds, s_cfg),
        Data::Enum(ds) => generate_enum_impl(ident.clone(), ds, s_cfg),
        _ => {
            return syn::Error::new_spanned(
                input,
                "#[derive(DynamoEntity)] only supports structs and enum",
            )
            .to_compile_error()
            .into();
        }
    };

    // record default/consts
    write_file::write_file(ident.to_string(), "dynamo_entities", out.to_string());

    out.into()
}

fn generate_query_option(st_name: &str, cfg: &StructCfg) -> proc_macro2::TokenStream {
    let opt_name = format!("{}QueryOption", st_name.to_case(convert_case::Case::Pascal));
    let opt_ident = Ident::new(&opt_name, proc_macro2::Span::call_site());

    let sk_field = if cfg.sk_name.is_some() {
        quote! {
            pub sk: Option<String>,

        }
    } else {
        quote! {}
    };

    let sk_fn = if cfg.sk_name.is_some() {
        quote! {
            pub fn sk(mut self, sk: String) -> Self {
                self.sk = Some(sk);
                self
            }

        }
    } else {
        quote! {}
    };

    quote! {
        pub struct #opt_ident {
            #sk_field
            pub bookmark: Option<String>,
            pub limit: i32,
            pub scan_index_forward: bool,
        }

        impl Default for #opt_ident {
            fn default() -> Self {
                Self {
                    sk: None,
                    bookmark: None,
                    limit: 10,
                    scan_index_forward: false,
                }
            }
        }

        impl #opt_ident {
            pub fn builder() -> Self {
                Self::default()
            }

            #sk_fn

            pub fn bookmark(mut self, bookmark: String) -> Self {
                self.bookmark = Some(bookmark);
                self
            }

            pub fn limit(mut self, limit: i32) -> Self {
                self.limit = limit;
                self
            }

            pub fn scan_index_forward(mut self, scan_index_forward: bool) -> Self {
                self.scan_index_forward = scan_index_forward;
                self
            }
        }

    }
}

fn generate_query_common_fn() -> proc_macro2::TokenStream {
    quote! {
        pub fn encode_lek_all(
            lek: &std::collections::HashMap<String, aws_sdk_dynamodb::types::AttributeValue>,
        ) -> String {
            use base64::{Engine as _, engine::general_purpose::STANDARD as B64};

            let v: serde_json::Value =
                serde_dynamo::from_item(lek.clone()).expect("failed to convert lek to json");

            B64.encode(v.to_string().as_bytes())
        }

        pub fn decode_bookmark_all(
            bookmark: &str,
        ) -> std::result::Result<
            std::collections::HashMap<String, aws_sdk_dynamodb::types::AttributeValue>,
        crate::Error2,
        > {
            use base64::{Engine as _, engine::general_purpose::STANDARD as B64};

            let bytes = B64.decode(bookmark).expect("failed to decode base64");
            let v = serde_json::to_value(&bytes).expect("failed to parse json");

            Ok(serde_dynamo::to_item(v)?)
        }

    }
}

fn get_additional_fields_for_indice(field: &FieldInfo) -> Vec<proc_macro2::TokenStream> {
    let mut out = vec![];
    let is_option = field.is_option();

    for idx in field.indice.iter() {
        let key_name = format!(
            "{}_{}",
            idx.base_index_name,
            if idx.pk { "pk" } else { "sk" }
        );
        let key_name = syn::LitStr::new(&key_name, proc_macro2::Span::call_site());
        let var_name = &field.ident;
        if let Some(ref prefix) = idx.prefix {
            out.push(
                if is_option {
                    quote! {
                        if let Some(ref v) = self.#var_name {
                            item.insert(
                                #key_name.to_string(),
                                aws_sdk_dynamodb::types::AttributeValue::S(format!("{}#{}", #prefix, v)),
                            );
                        }
                    }
                } else {
                    quote! {
                        item.insert(
                            #key_name.to_string(),
                            aws_sdk_dynamodb::types::AttributeValue::S(format!("{}#{}", #prefix, self.#var_name)),
                        );
                    }
                });
        } else {
            out.push(if is_option {
                quote! {
                    if let Some(ref v) = self.#var_name {
                        item.insert(
                            #key_name.to_string(),
                            aws_sdk_dynamodb::types::AttributeValue::S(v.to_string()),
                        );
                    }
                }
            } else {
                quote! {
                    item.insert(
                        #key_name.to_string(),
                        aws_sdk_dynamodb::types::AttributeValue::S(self.#var_name.to_string()),
                    );
                }
            });
        };
    }

    out.into()
}

fn generate_index_fn(
    st_name: &str,
    cfg: &StructCfg,
    idx_base_name: &str,
    idx_name: String,
    _fields: &Vec<&FieldInfo>,
) -> proc_macro2::TokenStream {
    let opt_name = format!("{}QueryOption", st_name.to_case(convert_case::Case::Pascal));
    let opt_ident = Ident::new(&opt_name, proc_macro2::Span::call_site());
    let err_ctor = syn::parse_str::<syn::Path>(&cfg.error_ctor).unwrap();
    let result_ty = syn::parse_str::<syn::Type>(&cfg.result_ty).unwrap();
    let idx_ident = Ident::new(&idx_name, proc_macro2::Span::call_site());
    let idx_name = syn::LitStr::new(
        &format!("{}-index", idx_base_name),
        proc_macro2::Span::call_site(),
    );
    let idx_pk_var = syn::LitStr::new(
        &format!("{}_pk", idx_base_name),
        proc_macro2::Span::call_site(),
    );
    let idx_sk_var = syn::LitStr::new(
        &format!("{}_sk", idx_base_name),
        proc_macro2::Span::call_site(),
    );

    let key_condition = quote! {
        let key_condition = if opt.sk.is_some() {
            "#pk = :pk AND begins_with(#sk, :sk)"
        } else {
            "#pk = :pk"
        };

    };
    let pk_composer = Ident::new(
        &format!("compose_{}_pk", idx_base_name),
        proc_macro2::Span::call_site(),
    );
    let sk_composer = Ident::new(
        &format!("compose_{}_sk", idx_base_name),
        proc_macro2::Span::call_site(),
    );

    let sk_condition = quote! {
        if let Some(sk) = opt.sk {
            req = req
                .expression_attribute_names("#sk", #idx_sk_var)
                .expression_attribute_values(":sk", aws_sdk_dynamodb::types::AttributeValue::S(Self::#sk_composer(sk)));
        }
    };

    quote! {
        pub async fn #idx_ident(
            cli: &aws_sdk_dynamodb::Client,
            pk: impl std::fmt::Display,
            opt: #opt_ident,
        ) -> #result_ty <(Vec<Self>, Option<String>), #err_ctor> {
            #key_condition

            let mut req = cli
                .query()
                .table_name(Self::table_name())
                .index_name(#idx_name)
                .expression_attribute_names("#pk", #idx_pk_var)
                .expression_attribute_values(":pk", aws_sdk_dynamodb::types::AttributeValue::S(Self::#pk_composer(pk)));

            #sk_condition

            if let Some(bookmark) = opt.bookmark {
                let lek = Self::decode_bookmark_all(&bookmark)?;
                req = req.set_exclusive_start_key(Some(lek));
            }

            let resp = req
                .limit(opt.limit)
                .scan_index_forward(opt.scan_index_forward)
                .key_condition_expression(key_condition)
                .send()
                .await
                .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

            let items = resp
                .items
                .unwrap_or_default()
                .into_iter()
                .map(|item| serde_dynamo::from_item(item).expect("failed to parse item"))
                .collect();

            let bookmark = if let Some(ref last_evaluated_key) = resp.last_evaluated_key {
                Some(Self::encode_lek_all(last_evaluated_key))
            } else {
                None
            };

            Ok((items, bookmark))
        }
    }
}

fn generate_index_fns(
    st_name: &str,
    cfg: &StructCfg,
    fields: &Vec<FieldInfo>,
    indice_name_map: &HashMap<String, String>,
) -> Vec<proc_macro2::TokenStream> {
    let mut out = vec![];

    let mut idx_map: HashMap<String, Vec<&FieldInfo>> = HashMap::new();

    for f in fields.iter() {
        for idx in f.indice.iter() {
            idx_map
                .entry(idx.base_index_name.clone())
                .or_default()
                .push(f);
        }
    }

    for idx in idx_map.keys() {
        let fields = idx_map.get(idx).unwrap();
        let fn_name = indice_name_map.get(idx).expect(&format!("find_by_{}", idx));
        let fn_tokens = generate_index_fn(st_name, cfg, idx, fn_name.clone(), fields);
        out.push(fn_tokens);
    }

    out.into()
}

fn generate_query_fn(
    st_name: &str,
    cfg: &StructCfg,
    fields: &Vec<FieldInfo>,
    indice_name_map: &HashMap<String, String>,
) -> proc_macro2::TokenStream {
    let opt_name = format!("{}QueryOption", st_name.to_case(convert_case::Case::Pascal));
    let _opt_ident = Ident::new(&opt_name, proc_macro2::Span::call_site());
    let ident = Ident::new(st_name, proc_macro2::Span::call_site());
    let _pk = &cfg.pk_name;
    let _sk = cfg.sk_name.clone().unwrap_or_default();
    let mut idx_fields_insert = vec![];

    for f in fields.iter() {
        let mut idx_fields = get_additional_fields_for_indice(f);
        idx_fields_insert.append(&mut idx_fields);
    }

    let common_query_fn = generate_query_common_fn();
    let index_fns = generate_index_fns(st_name, cfg, fields, indice_name_map);

    quote! {
        impl #ident {
            #common_query_fn

            pub fn indexed_fields(
                &self,
                mut item: std::collections::HashMap<String, aws_sdk_dynamodb::types::AttributeValue>,
            ) -> std::collections::HashMap<String, aws_sdk_dynamodb::types::AttributeValue> {
                #(#idx_fields_insert)*

                item
            }

            #(#index_fns)*
        }
    }
}
