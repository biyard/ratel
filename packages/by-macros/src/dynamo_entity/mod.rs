mod dynamo_index;

use std::collections::HashMap;

use convert_case::Casing;
use dynamo_index::DynamoIndex;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, Attribute, Data, DataEnum, DataStruct, DeriveInput, Fields, Ident,
    PathArguments, Type, TypePath,
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
    // #[allow(dead_code)]
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

    // Parse Option<A> => A, otherwise return original type
    pub fn inner_type(&self) -> Type {
        if let Type::Path(TypePath { path, .. }) = &self.ty {
            if let Some(seg) = path.segments.last() {
                if seg.ident == "Option" {
                    if let PathArguments::AngleBracketed(args) = &seg.arguments {
                        for arg in &args.args {
                            if let syn::GenericArgument::Type(inner_ty) = arg {
                                return inner_ty.clone();
                            }
                        }
                    }
                }
            }
        }
        self.ty.clone()
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
        error_ctor: "crate::Error".into(),
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
                    if v.is_empty()
                        || v.trim_matches('"') == "None"
                        || v.trim_matches('"') == "none"
                    {
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

fn parse_fields(
    ds: &DataStruct,
    cfg: &StructCfg,
) -> Result<
    (
        Vec<FieldInfo>,
        HashMap<String, String>,
        HashMap<String, DynamoIndex>,
    ),
    syn::Error,
> {
    let mut out = vec![];
    let pk = &cfg.pk_name;
    let sk = cfg.sk_name.clone().unwrap_or_default();
    let mut indice_fn: HashMap<String, String> = HashMap::new();
    let mut indice_v2: HashMap<String, DynamoIndex> = HashMap::new();

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
                let mut order: Option<i32> = None;

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
                    } else if meta.path.is_ident("order") {
                        if let Ok(value) = meta.value() {
                            if let Ok(lit) = value.parse::<syn::LitInt>() {
                                order = lit.base10_parse::<i32>().ok();
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
                    let idx_name = idx_name.clone().unwrap();

                    info.indice.push(IndexInfo {
                        name: fn_name.clone(),
                        base_index_name: idx_name.clone(),
                        pk: idx_pk,
                        sk: idx_sk,
                        prefix: idx_prefix.clone(),
                    });

                    let idx = if let Some(idx) = indice_v2.get_mut(&idx_name) {
                        idx
                    } else {
                        indice_v2.insert(
                            idx_name.clone(),
                            DynamoIndex {
                                name: format!("query_on_{}", &idx_name),
                                base_index_name: idx_name.clone(),
                                ..Default::default()
                            },
                        );
                        indice_v2.get_mut(&idx_name).unwrap()
                    };

                    if let Some(fn_name) = fn_name {
                        idx.name = fn_name.clone();
                    }

                    if idx_pk {
                        if let Some(prefix) = idx_prefix {
                            idx.pk.prefix = Some(prefix.to_case(convert_case::Case::UpperSnake));
                        }

                        let order = if let Some(i) = order {
                            i
                        } else {
                            if let Some(ref last) = idx.pk.fields.last() {
                                last.2 + 1
                            } else {
                                0
                            }
                        };

                        idx.pk.fields.push((ident.clone(), f.ty.clone(), order));
                        idx.pk.fields.sort_by_key(|t| t.2);
                    } else if idx_sk {
                        let sk = if let Some(ref mut sk) = idx.sk {
                            sk
                        } else {
                            idx.sk = Some(dynamo_index::DynamoIndexKey {
                                prefix: None,
                                fields: vec![],
                            });
                            idx.sk.as_mut().unwrap()
                        };

                        if let Some(prefix) = idx_prefix {
                            sk.prefix = Some(prefix.to_case(convert_case::Case::UpperSnake));
                        }

                        let order = if let Some(i) = order {
                            i
                        } else {
                            if let Some(ref last) = sk.fields.last() {
                                last.2 + 1
                            } else {
                                0
                            }
                        };

                        sk.fields.push((ident.clone(), f.ty.clone(), order));
                        sk.fields.sort_by_key(|t| t.2);
                    }
                }
            }
            out.push(info);
        }
    }

    Ok((out, indice_fn, indice_v2))
}

fn generate_key_composers(fields: &Vec<FieldInfo>) -> Vec<proc_macro2::TokenStream> {
    let mut out = vec![];
    let mut created_functions = HashMap::new();

    for f in fields.iter() {
        for idx in f.indice.iter() {
            let idx_base = idx.base_index_name.clone();
            let fk = format!("compose_{}_{}", idx_base, if idx.pk { "pk" } else { "sk" });
            let cname = Ident::new(&fk, proc_macro2::Span::call_site());

            if created_functions.contains_key(&fk) {
                continue;
            }
            created_functions.insert(fk.clone(), true);

            let token = if let Some(ref prefix) = idx.prefix {
                let comp = syn::LitStr::new(&format!("{}#", prefix), Span::call_site());

                quote! {
                    pub fn #cname(key: impl std::fmt::Display) -> String {
                        let key = key.to_string();
                        if key.starts_with(#comp) {
                            return key;
                        }

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

    let create_key_condition = if let Some(sk_name) = &s_cfg.sk_name {
        let condition = format!(
            "attribute_not_exists({}) AND attribute_not_exists({})",
            &s_cfg.pk_name, sk_name
        );
        syn::LitStr::new(&condition, proc_macro2::Span::call_site())
    } else {
        let condition = format!("attribute_not_exists({})", &s_cfg.pk_name);
        syn::LitStr::new(&condition, proc_macro2::Span::call_site())
    };

    let update_key_condition = if let Some(sk_name) = &s_cfg.sk_name {
        let condition = format!(
            "attribute_exists({}) AND attribute_exists({})",
            &s_cfg.pk_name, sk_name
        );
        syn::LitStr::new(&condition, proc_macro2::Span::call_site())
    } else {
        let condition = format!("attribute_exists({})", &s_cfg.pk_name);
        syn::LitStr::new(&condition, proc_macro2::Span::call_site())
    };

    let mut update_fns = vec![];

    for f in fields.iter() {
        if f.is_pk || f.is_sk {
            continue;
        }
        let var_name = &f.ident;
        let var_ty = f.inner_type();
        let fn_setter = Ident::new(
            &format!(
                "with_{}",
                var_name.to_string().to_case(convert_case::Case::Snake)
            ),
            proc_macro2::Span::call_site(),
        );
        let is_opt = f.is_option();
        let inner_setter = if is_opt {
            quote! {
                self.inner.#var_name = Some(#var_name);
            }
        } else {
            quote! {
                self.inner.#var_name = #var_name;
            }
        };
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
                    "get_{}_for_{}",
                    if idx.pk { "pk" } else { "sk" },
                    idx_base_snake,
                ),
                proc_macro2::Span::call_site(),
            );
            let key_base_name = format!(
                "{}_{}",
                idx.base_index_name,
                if idx.pk { "pk" } else { "sk" }
            );

            let idx_key_name = syn::LitStr::new(&key_base_name, proc_macro2::Span::call_site());

            let an_var = syn::LitStr::new(
                &format!("#{}", key_base_name.to_string()),
                proc_macro2::Span::call_site(),
            );
            let av_var = syn::LitStr::new(
                &format!(":{}", key_base_name.to_string()),
                proc_macro2::Span::call_site(),
            );

            let f_str = syn::LitStr::new(
                &format!(
                    "#{} = :{}",
                    key_base_name.to_string(),
                    key_base_name.to_string()
                ),
                proc_macro2::Span::call_site(),
            );

            gsi_put_updates.push(quote! {
                let value = self.inner.#composer_ident();

                if !value.is_empty() {
                    self.m.insert(
                        #idx_key_name.to_string(),
                        aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
                            .value(aws_sdk_dynamodb::types::AttributeValue::S(
                                self.inner.#composer_ident()
                            ))
                            .action(aws_sdk_dynamodb::types::AttributeAction::Put)
                            .build(),
                    );

                    if !self.set_update_expressions.contains(&#f_str.to_string()) {
                        self.set_update_expressions.push(#f_str.to_string());
                    }

                    self.expression_attribute_names.insert(#an_var.to_string(), #idx_key_name.to_string());
                    self.expression_attribute_values.insert(#av_var.to_string(), aws_sdk_dynamodb::types::AttributeValue::S(
                        self.inner.#composer_ident()
                    ));

                }
            });
        }

        // Build additional GSI updates for this field (DELETE on remove)
        let mut gsi_delete_updates: Vec<proc_macro2::TokenStream> = vec![];
        for idx in f.indice.iter() {
            let key_base_name = format!(
                "{}_{}",
                idx.base_index_name,
                if idx.pk { "pk" } else { "sk" }
            );

            let idx_key_name = syn::LitStr::new(&key_base_name, proc_macro2::Span::call_site());
            let an_var = syn::LitStr::new(
                &format!("#{}", key_base_name.to_string()),
                proc_macro2::Span::call_site(),
            );

            let f_str = syn::LitStr::new(
                &format!("#{}", key_base_name.to_string()),
                proc_macro2::Span::call_site(),
            );

            gsi_delete_updates.push(quote! {
                self.m.insert(
                    #idx_key_name.to_string(),
                    aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
                        .action(aws_sdk_dynamodb::types::AttributeAction::Delete)
                        .build(),
                );

                self.remove_update_expressions.push(#f_str.to_string());
                self.expression_attribute_names.insert(#an_var.to_string(), #idx_key_name.to_string());

            });
        }

        let av_var = syn::LitStr::new(
            &format!(":{}", var_name.to_string()),
            proc_macro2::Span::call_site(),
        );

        let an_var = syn::LitStr::new(
            &format!("#{}", var_name.to_string()),
            proc_macro2::Span::call_site(),
        );

        // setter
        let f_str = syn::LitStr::new(
            &format!("#{} = :{}", var_name.to_string(), var_name.to_string(),),
            proc_macro2::Span::call_site(),
        );

        update_fns.push(quote! {
            pub fn #fn_setter(mut self, #var_name: #var_ty) -> Self {
                let av:aws_sdk_dynamodb::types::AttributeValue = serde_dynamo::to_attribute_value(&#var_name)
                    .expect("failed to serialize field");
                let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
                    .value(av.clone())
                    .action(aws_sdk_dynamodb::types::AttributeAction::Put)
                    .build();
                self.m.insert(stringify!(#var_name).to_string(), v);

                #inner_setter

                self.set_update_expressions.push(#f_str.to_string());
                self.expression_attribute_names.insert(#an_var.to_string(), stringify!(#var_name).to_string());
                self.expression_attribute_values.insert(#av_var.to_string(), av);


                // Update derived GSI attributes for this field
                #(#gsi_put_updates)*
                self
            }
        });
        // remove
        let f_str = syn::LitStr::new(
            &format!("#{}", var_name.to_string()),
            proc_macro2::Span::call_site(),
        );

        update_fns.push(quote! {
            pub fn #fn_remove(mut self) -> Self {
                let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
                    .action(aws_sdk_dynamodb::types::AttributeAction::Delete)
                    .build();
                self.m.insert(stringify!(#var_name).to_string(), v);

                self.remove_update_expressions.push(#f_str.to_string());
                self.expression_attribute_names.insert(#an_var.to_string(), stringify!(#var_name).to_string());

                // Remove derived GSI attributes for this field
                #(#gsi_delete_updates)*
                self
            }
        });

        if !f.is_number_type() {
            continue;
        }

        // increase
        let f_str = syn::LitStr::new(
            &format!(
                "#{} = if_not_exists(#{}, :z) + :{}",
                var_name.to_string(),
                var_name.to_string(),
                var_name.to_string(),
            ),
            proc_macro2::Span::call_site(),
        );

        update_fns.push(quote! {
            pub fn #fn_increase(mut self, by: i64) -> Self {
                let av:aws_sdk_dynamodb::types::AttributeValue = serde_dynamo::to_attribute_value(by)
                    .expect("failed to serialize field");
                let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
                    .value(av.clone())
                    .action(aws_sdk_dynamodb::types::AttributeAction::Add)
                    .build();
                self.m.insert(stringify!(#var_name).to_string(), v);

                self.set_update_expressions.push(#f_str.to_string());
                self.expression_attribute_names.insert(#an_var.to_string(), stringify!(#var_name).to_string());
                self.expression_attribute_values.insert(#av_var.to_string(), av.clone());
                self.expression_attribute_values.insert(":z".to_string(), aws_sdk_dynamodb::types::AttributeValue::N("0".to_string()));

                self
            }
        });
        // decrease
        update_fns.push(quote! {
            pub fn #fn_decrease(mut self, by: i64) -> Self {
                let av:aws_sdk_dynamodb::types::AttributeValue = serde_dynamo::to_attribute_value(-by)
                    .expect("failed to serialize field");
                let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
                    .value(av.clone())
                    .action(aws_sdk_dynamodb::types::AttributeAction::Add)
                    .build();
                self.m.insert(stringify!(#var_name).to_string(), v);

                self.set_update_expressions.push(#f_str.to_string());
                self.expression_attribute_names.insert(#an_var.to_string(), stringify!(#var_name).to_string());
                self.expression_attribute_values.insert(#av_var.to_string(), av.clone());
                self.expression_attribute_values.insert(":z".to_string(), aws_sdk_dynamodb::types::AttributeValue::N("0".to_string()));


                self
            }
        });
    }
    let err_ctor: syn::Path = syn::parse_str(&s_cfg.error_ctor).unwrap();
    let result_ty: syn::Type = syn::parse_str(&s_cfg.result_ty).unwrap();

    quote! {
        pub struct #updater_ident {
            #key_fields
            inner: #ident,
            m: std::collections::HashMap<String, aws_sdk_dynamodb::types::AttributeValueUpdate>,
            set_update_expressions: Vec<String>,
            remove_update_expressions: Vec<String>,
            expression_attribute_names: std::collections::HashMap<::std::string::String, ::std::string::String>,
            expression_attribute_values: std::collections::HashMap<::std::string::String, aws_sdk_dynamodb::types::AttributeValue>,

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
                    inner: Default::default(),
                    m: std::collections::HashMap::new(),
                    k,
                    set_update_expressions: vec![],
                    remove_update_expressions: vec![],
                    expression_attribute_names: std::collections::HashMap::new(),
                    expression_attribute_values: std::collections::HashMap::new(),
                }
            }

            pub fn create_transact_write_item(&self) -> aws_sdk_dynamodb::types::TransactWriteItem {
                let item = serde_dynamo::to_item(self)
                    .expect("failed to serialize struct to dynamodb item");
                let item = self.indexed_fields(item);

                let req = aws_sdk_dynamodb::types::Put::builder()
                    .table_name(Self::table_name())
                    .condition_expression(#create_key_condition)
                    .set_item(Some(item))
                    .build().unwrap();

                aws_sdk_dynamodb::types::TransactWriteItem::builder()
                    .put(req)
                    .build()
            }

            pub fn delete_transact_write_item(pk: impl std::fmt::Display, #sk_param) -> aws_sdk_dynamodb::types::TransactWriteItem {
                let k = std::collections::HashMap::from([
                    (
                        #pk_field.to_string(),
                        aws_sdk_dynamodb::types::AttributeValue::S(pk.to_string()),
                    ),
                    #sk_key
                ]);

                let req = aws_sdk_dynamodb::types::Delete::builder()
                    .table_name(Self::table_name())
                    .condition_expression(#update_key_condition)
                    .set_key(Some(k))
                    .build().unwrap();

                aws_sdk_dynamodb::types::TransactWriteItem::builder()
                    .delete(req)
                    .build()
            }
        }

        impl #updater_ident {
            #(#update_fns)*

            pub fn transact_write_item(self) -> aws_sdk_dynamodb::types::TransactWriteItem {
                let mut req = aws_sdk_dynamodb::types::Update::builder()
                    .table_name(#ident::table_name())
                    .condition_expression(#update_key_condition)
                    .set_key(Some(self.k));

                let mut update_expr = "".to_string();
                if !self.remove_update_expressions.is_empty() {
                    update_expr = format!("REMOVE {}", self.remove_update_expressions.join(", "));
                }

                if !self.set_update_expressions.is_empty() {
                    update_expr = format!("SET {} {}", self.set_update_expressions.join(", "), update_expr);
                };

                if !update_expr.is_empty() {
                    req = req.update_expression(update_expr);
                }
                if !self.expression_attribute_names.is_empty() {
                    req = req.set_expression_attribute_names(Some(self.expression_attribute_names));
                }
                if !self.expression_attribute_values.is_empty() {
                    req = req.set_expression_attribute_values(Some(self.expression_attribute_values));
                }

                aws_sdk_dynamodb::types::TransactWriteItem::builder()
                    .update(req.build().expect("invalid transact write item request"))
                    .build()
            }

            pub fn transact_upsert_item(self) -> aws_sdk_dynamodb::types::TransactWriteItem {
                let mut req = aws_sdk_dynamodb::types::Update::builder()
                    .table_name(#ident::table_name())
                    .set_key(Some(self.k));

                let mut update_expr = "".to_string();
                if !self.remove_update_expressions.is_empty() {
                    update_expr = format!("REMOVE {}", self.remove_update_expressions.join(", "));
                }

                if !self.set_update_expressions.is_empty() {
                    update_expr = format!("SET {} {}", self.set_update_expressions.join(", "), update_expr);
                };

                if !update_expr.is_empty() {
                    req = req.update_expression(update_expr);
                }
                if !self.expression_attribute_names.is_empty() {
                    req = req.set_expression_attribute_names(Some(self.expression_attribute_names));
                }
                if !self.expression_attribute_values.is_empty() {
                    req = req.set_expression_attribute_values(Some(self.expression_attribute_values));
                }

                aws_sdk_dynamodb::types::TransactWriteItem::builder()
                    .update(req.build().expect("invalid transact write item request"))
                    .build()
            }

            pub async fn execute(
                self,
                cli: &aws_sdk_dynamodb::Client,
            ) -> #result_ty <#ident, #err_ctor> {
                let res = cli.update_item()
                    .table_name(#ident::table_name())
                    .set_key(Some(self.k))
                    .set_attribute_updates(Some(self.m))
                    .return_values(aws_sdk_dynamodb::types::ReturnValue::AllNew)
                    .send()
                    .await
                    .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

                Ok(serde_dynamo::from_item(res.attributes.unwrap_or_default())?)
            }
        }
    }
}

fn generate_builder_functions(fields: &Vec<FieldInfo>) -> proc_macro2::TokenStream {
    let mut fns = vec![];

    for f in fields.iter() {
        let var_name = &f.ident;
        let var_ty = f.inner_type();
        let fn_setter = Ident::new(
            &format!(
                "with_{}",
                var_name.to_string().to_case(convert_case::Case::Snake)
            ),
            proc_macro2::Span::call_site(),
        );

        let is_opt = f.is_option();
        let inner_setter = if is_opt {
            quote! {
                self.#var_name = Some(#var_name);
            }
        } else {
            quote! {
                self.#var_name = #var_name;
            }
        };

        fns.push(quote! {
            pub fn #fn_setter(mut self, #var_name: #var_ty) -> Self {
                #inner_setter
                self
            }
        });
    }

    quote! {
        pub fn builder() -> Self {
            Self {
                ..Default::default()
            }
        }

        #(#fns)*
    }
}

fn generate_struct_impl(
    ident: Ident,
    ds: &DataStruct,
    s_cfg: StructCfg,
) -> proc_macro2::TokenStream {
    let st_name = ident.to_string();

    let (fields, indice_fn, indice_v2) = match parse_fields(ds, &s_cfg) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error(),
    };

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

    let batch_get_param = if s_cfg.sk_name.is_some() {
        quote! { (impl std::fmt::Display, impl std::fmt::Display) }
    } else {
        quote! {impl std::fmt::Display}
    };

    let batch_get_key = if let Some(ref sk_name) = s_cfg.sk_name {
        let sk_name = syn::LitStr::new(sk_name, proc_macro2::Span::call_site());

        quote! {
            (
                #pk_field_name.to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(key.0.to_string()),
            ),
            (
                #sk_name.to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(key.1.to_string()),
            ),
        }
    } else {
        quote! {
            (
                #pk_field_name.to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(key.to_string()),
            ),
        }
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
    let sk_fn = if let Some(ref sk) = s_cfg.sk_name {
        let sk_field_name = syn::LitStr::new(sk, proc_macro2::Span::call_site());

        quote! {
            pub async fn query_begins_with_sk(
                cli: &aws_sdk_dynamodb::Client,
                pk: impl std::fmt::Display,
                sk: impl std::fmt::Display,
            ) -> #result_ty <(Vec<#ident>, Option<String>), #err_ctor> {
                let resp = cli
                    .query()
                    .table_name(#table_lit_str)
                    .limit(100)
                    .scan_index_forward(false)
                    .key_condition_expression("#pk = :pk AND begins_with(#sk, :sk)")
                    .expression_attribute_names("#pk", #pk_field_name)
                    .expression_attribute_names("#sk", #sk_field_name)
                    .expression_attribute_values(
                        ":pk",
                        aws_sdk_dynamodb::types::AttributeValue::S(pk.to_string()),
                    )
                    .expression_attribute_values(
                        ":sk",
                        aws_sdk_dynamodb::types::AttributeValue::S(sk.to_string()),
                    )
                    .send()
                    .await
                    .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

                let items = resp
                    .items
                    .unwrap_or_default()
                    .into_iter()
                    .map(|item| serde_dynamo::from_item(item))
                    .collect::<std::result::Result<Vec<#ident>, _>>()?;

                let bookmark = if let Some(ref last_evaluated_key) = resp.last_evaluated_key {
                    Some(Self::encode_lek_all(last_evaluated_key)?)
                } else {
                    None
                };

                Ok((items, bookmark))
            }

        }
    } else {
        quote! {}
    };

    let st_query_option = generate_query_option(&st_name, &s_cfg);
    let query_fn = generate_query_fn(&st_name, &s_cfg, &fields, &indice_fn, &indice_v2);
    let key_composers = generate_key_composers(&fields);
    let updater = generate_updater(&ident, &s_cfg, &fields);
    let opt_name = format!("{}QueryOption", st_name.to_case(convert_case::Case::Pascal));
    let opt_ident = Ident::new(&opt_name, proc_macro2::Span::call_site());
    let create_key_condition = if let Some(sk_name) = &s_cfg.sk_name {
        let condition = format!(
            "attribute_not_exists({}) AND attribute_not_exists({})",
            &s_cfg.pk_name, sk_name
        );
        syn::LitStr::new(&condition, proc_macro2::Span::call_site())
    } else {
        let condition = format!("attribute_not_exists({})", &s_cfg.pk_name);
        syn::LitStr::new(&condition, proc_macro2::Span::call_site())
    };

    let update_key_condition = if let Some(sk_name) = &s_cfg.sk_name {
        let condition = format!(
            "attribute_exists({}) AND attribute_exists({})",
            &s_cfg.pk_name, sk_name
        );
        syn::LitStr::new(&condition, proc_macro2::Span::call_site())
    } else {
        let condition = format!("attribute_exists({})", &s_cfg.pk_name);
        syn::LitStr::new(&condition, proc_macro2::Span::call_site())
    };

    let mut idx_fns_v2 = vec![];
    for (_, idx) in indice_v2.iter() {
        idx_fns_v2.push(idx.generate());
    }
    let builder_fns = generate_builder_functions(&fields);

    let out = quote! {
        #st_query_option

        #query_fn

        #updater


        impl #ident {
            #(#key_composers)*

            #builder_fns

            #(#idx_fns_v2)*

            pub fn table_name() -> &'static str {
                #table_lit_str
            }

            pub fn pk_field() -> &'static str { #pk_field_name }
            pub fn sk_field() -> Option<&'static str> {
                #sk_field_method
            }

            pub async fn query(
                cli: &aws_sdk_dynamodb::Client,
                pk: impl std::fmt::Display,
                opt: #opt_ident,
            ) -> #result_ty <(Vec<#ident>, Option<String>), #err_ctor> {
                let key_condition = if opt.sk.is_some() {
                    "#pk = :pk AND begins_with(#sk, :sk)"
                } else {
                    "#pk = :pk"
                };

                let mut req = cli
                    .query()
                    .table_name(#table_lit_str)
                    .key_condition_expression(key_condition)
                    .expression_attribute_names("#pk", #pk_field_name)
                    .expression_attribute_values(
                        ":pk",
                        aws_sdk_dynamodb::types::AttributeValue::S(pk.to_string()),
                    );

                if let Some(sk) = opt.sk {
                    req = req
                        .expression_attribute_names("#sk", "sk")
                        .expression_attribute_values(":sk", aws_sdk_dynamodb::types::AttributeValue::S(sk.to_string()));
                }

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
                    .map(|item| serde_dynamo::from_item(item))
                    .collect::<std::result::Result<Vec<_>, _>>()?;

                let bookmark = if let Some(ref last_evaluated_key) = resp.last_evaluated_key {
                    Some(Self::encode_lek_all(last_evaluated_key)?)
                } else {
                    None
                };

                Ok((items, bookmark))
            }

            #sk_fn

            pub async fn create(
                &self,
                cli: &aws_sdk_dynamodb::Client,
            ) -> #result_ty <(), #err_ctor> {
                let item = serde_dynamo::to_item(self)?;

                let item = self.indexed_fields(item);

                cli.put_item()
                    .table_name(Self::table_name())
                    .condition_expression(#create_key_condition)
                    .set_item(Some(item))
                    .send()
                    .await.map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

                Ok(())
            }

            pub async fn upsert(
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
            ) -> #result_ty <Self, #err_ctor> {
                let mut req = cli.delete_item().table_name(Self::table_name())
                    .condition_expression(#update_key_condition)
                    .key(
                        Self::pk_field(),
                        aws_sdk_dynamodb::types::AttributeValue::S(pk.to_string()),
                    );

                #sk_condition

                let old = req
                    .return_values(aws_sdk_dynamodb::types::ReturnValue::AllOld)
                    .send()
                    .await
                    .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

                if let Some(item) = old.attributes {
                    let ev: Self = serde_dynamo::from_item(item)?;
                    Ok(ev)
                } else {
                    Err("Item not found".to_string().into())
                }
            }

            pub async fn batch_get(
                cli: &aws_sdk_dynamodb::Client,
                keys: Vec<#batch_get_param>,
            ) -> std::result::Result<Vec<Self>, #err_ctor> {
                if keys.is_empty() {
                    return Ok(vec![]);
                }

                let keys = keys
                    .iter()
                    .map(|key| {
                        std::collections::HashMap::from([
                            #batch_get_key
                        ])
                    })
                    .collect::<Vec<_>>();

                let keys_and_attributes = aws_sdk_dynamodb::types::KeysAndAttributes::builder()
                    .set_keys(Some(keys))
                    .consistent_read(false)
                    .build()
                    .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

                let table_name = Self::table_name();

                let response = cli
                    .batch_get_item()
                    .request_items(table_name, keys_and_attributes)
                    .send()
                    .await
                    .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

                let items = if let Some(responses) = response.responses() {
                    if let Some(items) = responses.get(table_name) {
                        serde_dynamo::from_items(items.to_vec())?
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                };

                Ok(items)
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
    let sk_fn = if let Some(ref sk) = s_cfg.sk_name {
        let sk_field_name = syn::LitStr::new(sk, proc_macro2::Span::call_site());

        quote! {
            pub async fn query_begins_with_sk(
                cli: &aws_sdk_dynamodb::Client,
                pk: impl std::fmt::Display,
                sk: impl std::fmt::Display,
            ) -> #result_ty <Vec<#ident>, #err_ctor> {
                let resp = cli
                    .query()
                    .table_name(#table_lit_str)
                    .key_condition_expression("#pk = :pk AND begins_with(#sk, :sk)")
                    .expression_attribute_names("#pk", #pk_field_name)
                    .expression_attribute_names("#sk", #sk_field_name)
                    .expression_attribute_values(
                        ":pk",
                        aws_sdk_dynamodb::types::AttributeValue::S(pk.to_string()),
                    )
                    .expression_attribute_values(
                        ":sk",
                        aws_sdk_dynamodb::types::AttributeValue::S(sk.to_string()),
                    )
                    .send()
                    .await
                    .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

                let items = resp
                    .items
                    .unwrap_or_default()
                    .into_iter()
                    .map(|item| serde_dynamo::from_item(item))
                    .collect::<std::result::Result<Vec<#ident>, _>>()?;
                Ok(items)
            }

        }
    } else {
        quote! {}
    };

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
                    .map(|item| serde_dynamo::from_item(item))
                    .collect::<std::result::Result<Vec<#ident>, _>>()?;
                Ok(items)
            }

            #sk_fn

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

    let sk_field_default = if cfg.sk_name.is_some() {
        quote! { sk: None, }
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
                    #sk_field_default
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
        ) -> std::result::Result<String, crate::Error> {
            let mut bookmark = vec![];
            for (k, v) in lek.iter() {
                match v {
                    aws_sdk_dynamodb::types::AttributeValue::S(s) => {
                        bookmark.push(format!("{};;;{}", k, s));
                    }
                    _ => {
                        return Err(crate::Error::InternalServerError(
                            "Unsupported AttributeValue type in LEK".into(),
                        ));
                    }
                }
            }
            let bookmark = bookmark.join(";;;").to_owned();

            use base64::Engine as _;
            let encoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bookmark);

            Ok(encoded)
        }

        pub fn decode_bookmark_all(
            bookmark: &str,
        ) -> std::result::Result<
            std::collections::HashMap<String, aws_sdk_dynamodb::types::AttributeValue>,
        crate::Error,
        > {
            use base64::Engine as _;

            let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(bookmark)?;
            let s = String::from_utf8(bytes).map_err(|e| e.to_string())?;
            let parts: Vec<&str> = s.split(";;;").collect();
            if parts.len() % 2 != 0 {
                return Err(crate::Error::InvalidBookmark);
            }
            let mut v = std::collections::HashMap::new();
            for i in (0..parts.len()).step_by(2) {
                let key = parts[i];
                let value = parts[i + 1];
                v.insert(
                    key.to_string(),
                    aws_sdk_dynamodb::types::AttributeValue::S(value.to_string()),
                );
            }

            Ok(v)
        }

    }
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
                .map(|item| serde_dynamo::from_item(item))
                .collect::<std::result::Result<Vec<_>, _>>()?;

            let bookmark = if let Some(ref last_evaluated_key) = resp.last_evaluated_key {
                Some(Self::encode_lek_all(last_evaluated_key)?)
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
    indice: &HashMap<String, DynamoIndex>,
) -> proc_macro2::TokenStream {
    let opt_name = format!("{}QueryOption", st_name.to_case(convert_case::Case::Pascal));
    let _opt_ident = Ident::new(&opt_name, proc_macro2::Span::call_site());
    let ident = Ident::new(st_name, proc_macro2::Span::call_site());
    let _pk = &cfg.pk_name;
    let _sk = cfg.sk_name.clone().unwrap_or_default();
    let mut idx_fields_insert = vec![];

    // for f in fields.iter() {
    //     let mut idx_fields = get_additional_fields_for_indice(f);
    //     idx_fields_insert.append(&mut idx_fields);
    // }

    for (_, idx) in indice.iter() {
        idx_fields_insert.push(idx.get_additional_fields());
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
