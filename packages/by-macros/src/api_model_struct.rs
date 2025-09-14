#![allow(dead_code, unused)]

#[cfg(feature = "server")]
use crate::query_builder_functions::*;
use crate::{
    action::Actions,
    api_model::*,
    parse_queryable_fields::{parse_queryable_action_fields, parse_queryable_fields},
};
use convert_case::{Case, Casing};
use indexmap::IndexMap;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use std::{
    collections::{HashMap, HashSet},
    convert::From,
};
use syn::*;
use tracing::instrument;

#[cfg(feature = "server")]
use crate::sql_model::*;

pub enum Database {
    Postgres,
}

pub struct ApiModel<'a> {
    #[cfg(feature = "server")]
    pub table_name: String,
    #[cfg(feature = "server")]
    pub rename: Case,
    #[cfg(feature = "server")]
    pub fields: IndexMap<String, ApiField>,
    #[cfg(feature = "server")]
    pub primary_key: (String, String), // (sql_name, sql_type)

    #[cfg(feature = "server")]
    pub database: Option<Database>,

    pub result_type: proc_macro2::TokenStream,

    pub name: String,
    pub name_id: &'a syn::Ident,
    pub actions: Actions,

    pub has_validator: bool,
    pub iter_type: String,
    pub custom_query_type: Option<String>,
    pub base: String,
    pub parent_ids: Vec<String>,
    pub summary_fields: Vec<Field>,
    pub queryable_fields: Vec<Field>,
    pub action_names: IndexMap<String, ActionField>,
    pub action_by_id_names: IndexMap<String, ActionField>,
    pub query_action_names: IndexMap<String, ActionField>,
    pub read_action_names: IndexMap<String, ActionField>,
}

impl std::fmt::Debug for ApiModel<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(feature = "server")]
        return f
            .debug_struct("ApiModel")
            .field("table_name", &self.table_name)
            .field("rename", &self.rename)
            .field("name", &self.name)
            .field("iter_type", &self.iter_type)
            .field("base", &self.base)
            .field("parent_ids", &self.parent_ids)
            .field("has_validator", &self.has_validator)
            .field("summary_fields", &self.summary_fields)
            .field("queryable_fields", &self.queryable_fields)
            .finish();

        #[cfg(not(feature = "server"))]
        f.debug_struct("ApiModel")
            .field("name", &self.name)
            .field("iter_type", &self.iter_type)
            .field("base", &self.base)
            .field("parent_ids", &self.parent_ids)
            .field("has_validator", &self.has_validator)
            .field("summary_fields", &self.summary_fields)
            .field("queryable_fields", &self.queryable_fields)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub enum JsonMeta {
    Enum(Vec<String>),
    Default(String),
    Description(String),
}

fn object_field(
    var_name: String,
    var_type: String,
    meta: Vec<JsonMeta>,
) -> proc_macro2::TokenStream {
    let var_name = syn::LitStr::new(&var_name, proc_macro2::Span::call_site());
    let var_type = match var_type.as_str() {
        "i64" | "i32" | "u64" | "u32" => "Integer",
        "String" => "String",
        "bool" => "Boolean",
        e => {
            let ty = syn::Ident::new(e, proc_macro2::Span::call_site());

            let out = quote! {
                schema_obj
                    .object()
                    .properties
                    .insert(#var_name.to_string(), #ty::json_schema(_gen));
            };

            return out;
        }
    };
    let var_type = syn::Ident::new(var_type, proc_macro2::Span::call_site());
    let mut enum_values = quote! {};
    let mut description = quote! {};
    let mut defaults = quote! {};

    for m in meta {
        match m {
            JsonMeta::Enum(values) => {
                let values = values
                    .iter()
                    .map(|v| syn::LitStr::new(v, proc_macro2::Span::call_site()));

                enum_values = quote! {
                    enum_values: Some(vec![
                        #(serde_json::Value::String(#values.to_string()),)*
                    ]),
                };
            }
            JsonMeta::Description(desc) => {
                let desc = syn::LitStr::new(&desc, proc_macro2::Span::call_site());
                description = quote! {
                    description: Some(#desc.to_string()),
                };
            }
            JsonMeta::Default(v) => {
                // FIXME: This does not work properly
                let v = syn::LitStr::new(&v, proc_macro2::Span::call_site());
                defaults = quote! {
                    default: Some(serde_json::Value::String(#v.to_string())),
                };
            }
        }
    }

    quote! {
        schema_obj.object().properties.insert(
            #var_name.to_string(),
            schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                metadata: Some(Box::new(schemars::schema::Metadata {
                    #description
                    #defaults

                    ..Default::default()
                })),
                instance_type: Some(schemars::schema::InstanceType::#var_type.into()),

                #enum_values

                ..Default::default()
            }),
        );

    }
}

impl ApiModel<'_> {
    pub fn generate_jsonschema_for_param(&self) -> proc_macro2::TokenStream {
        let name = syn::Ident::new(&format!("{}Param", self.name), self.name_id.span());
        let name_str = syn::LitStr::new(&name.to_string(), name.span());
        let title = syn::LitStr::new(&format!("{} Query Parameters", self.name), name.span());
        let query_actions = &self.query_action_names;
        let mut actions = HashSet::new();

        #[derive(Clone, Debug)]
        struct Schema {
            ty: String,
            meta: Vec<JsonMeta>,
            actions: Vec<String>,
        }

        let mut fields: IndexMap<String, Schema> = IndexMap::new();
        fields.insert(
            "size".to_string(),
            Schema {
                ty: "i32".to_string(),
                meta: vec![JsonMeta::Description(
                    "Number of items to return".to_string(),
                )],
                actions: vec![],
            },
        );

        fields.insert(
            "bookmark".to_string(),
            Schema {
                ty: "String".to_string(),
                meta: vec![
                    JsonMeta::Description(
                        "bookmark of page number. Note that you must stringify page number."
                            .to_string(),
                    ),
                    JsonMeta::Default("1".to_string()),
                ],
                actions: vec![],
            },
        );

        for f in self.queryable_fields.iter() {
            let name = f.ident.to_token_stream().to_string();
            let ty = f.ty.to_token_stream().to_string();

            fields.insert(
                name,
                Schema {
                    ty,
                    meta: vec![],
                    actions: vec![],
                },
            );
        }

        for (action, f) in self.query_action_names.iter() {
            actions.insert(action.to_string());
            if let ActionField::Fields(v) = f {
                for field in v.iter() {
                    let name = field.ident.to_token_stream().to_string();
                    let ty = field.ty.to_token_stream().to_string();
                    let meta = vec![];

                    if fields.contains_key(&name) {
                        fields
                            .get_mut(&name)
                            .unwrap()
                            .actions
                            .push(action.to_string());
                    } else {
                        fields.insert(
                            name,
                            Schema {
                                ty,
                                meta,
                                actions: vec![action.to_string()],
                            },
                        );
                    }
                }
            }
            // TODO: Support related fields. Query Jsonschema must be flattened. At this point, we cannot know the whole fields of related struct.
        }

        for (action, f) in self.read_action_names.iter() {
            actions.insert(action.to_string());
            if let ActionField::Fields(v) = f {
                for field in v.iter() {
                    let name = field.ident.to_token_stream().to_string();
                    let ty = field.ty.to_token_stream().to_string();
                    let meta = vec![];

                    if fields.contains_key(&name) {
                        fields
                            .get_mut(&name)
                            .unwrap()
                            .actions
                            .push(action.to_string());
                    } else {
                        fields.insert(
                            name,
                            Schema {
                                ty,
                                meta,
                                actions: vec![action.to_string()],
                            },
                        );
                    }
                }
            }
            // TODO: Support related fields. Query Jsonschema must be flattened. At this point, we cannot know the whole fields of related struct.
        }

        for (action, action_fields) in self.actions.read_actions.iter() {
            actions.insert(action.to_string());
            for field in action_fields.iter() {
                let name = field.name.clone();
                let ty = field.r#type.clone();
                let meta = vec![];

                if fields.contains_key(&name) {
                    fields
                        .get_mut(&name)
                        .unwrap()
                        .actions
                        .push(action.to_string());
                } else {
                    fields.insert(
                        name,
                        Schema {
                            ty,
                            meta,
                            actions: vec![action.to_string()],
                        },
                    );
                }
            }
        }

        let mut objs = vec![];

        if actions.len() > 0 {
            objs.push(object_field(
                "action".to_string(),
                "String".to_string(),
                vec![
                    JsonMeta::Description("request action type".to_string()),
                    JsonMeta::Enum(actions.iter().cloned().collect()),
                ],
            ));
        }

        for (n, v) in fields {
            let mut meta = v.meta.clone();
            if v.actions.len() > 0 {
                meta.push(JsonMeta::Description(format!(
                    "This field is used in the following actions: {}",
                    v.actions.join(", ")
                )));
            }

            objs.push(object_field(n, v.ty, meta));
        }

        quote! {
            #[cfg(feature = "server")]
            impl schemars::JsonSchema for #name {
                fn schema_name() -> String {
                    #name_str.to_string()
                }

                fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                    let mut schema_obj = schemars::schema::SchemaObject::default();
                    schema_obj.metadata = Some(Box::new(schemars::schema::Metadata {
                        title: Some(#title.to_string()),
                        ..Default::default()
                    }));

                    #(#objs)*

                    schema_obj.object().required.insert("size".to_string());

                    schemars::schema::Schema::Object(schema_obj)
                }
            }
        }
    }

    pub fn generate_client_impl(&self) -> proc_macro2::TokenStream {
        let struct_name = self.name_id;
        let base_endpoint = &self.base;
        let parent_ids = &self.parent_ids;
        let iter_type = &self.iter_type;

        let client_name = syn::Ident::new(&format!("{}Client", struct_name), struct_name.span());
        let query_name = syn::Ident::new(&format!("{}Query", struct_name), struct_name.span());
        let summary_name = syn::Ident::new(&format!("{}Summary", struct_name), struct_name.span());
        let param_name = syn::Ident::new(&format!("{}Param", struct_name), struct_name.span());

        let base_endpoint_lit = syn::LitStr::new(base_endpoint, struct_name.span());

        let iter_type_with_summary = format!("{}<{}>", iter_type, summary_name);
        let iter_type_tokens: proc_macro2::TokenStream = iter_type_with_summary.parse().unwrap();
        let parent_params = parent_ids.iter().map(|id| {
            let id = syn::Ident::new(id, struct_name.span());
            quote! { #id: i64, }
        });
        let parent_names = parent_ids.iter().map(|id| {
            let id = syn::Ident::new(id, struct_name.span());
            quote! { #id, }
        });
        let parent_names_for_get = parent_names.clone();
        let parent_params_for_get = parent_params.clone();
        let rt = &self.result_type;

        let mut custom_query = quote! {};

        if let Some(q) = self.custom_query_type() {
            let parent_names = parent_names.clone();
            let parent_params = parent_params.clone();

            custom_query = quote! {
                pub async fn query_by_custom(
                    &self,
                    #(#parent_params)*
                    params: #q,
                ) -> #rt<#iter_type_tokens> {
                    let path = format!(#base_endpoint_lit, #(#parent_names)*);
                    let endpoint = format!("{}{}", self.endpoint, path);
                    let query = format!("{}?{}", endpoint, #param_name::Custom(params));
                    rest_api::get(&query).await
                }
            };
        }

        quote! {
            impl #struct_name {
                pub fn get_client(endpoint: &str) -> #client_name {
                    #client_name { endpoint: endpoint.to_string() }
                }
            }

            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq)]
            pub struct #client_name {
                pub endpoint: String,
            }

            impl #client_name {
                pub async fn query(
                    &self,
                    #(#parent_params)*
                    params: #query_name,
                ) -> #rt<#iter_type_tokens> {
                    let path = format!(#base_endpoint_lit, #(#parent_names)*);
                    let endpoint = format!("{}{}", self.endpoint, path);
                    let query = format!("{}?{}", endpoint, #param_name::Query(params));
                    rest_api::get(&query).await
                }

                #custom_query

                pub async fn get(&self, #(#parent_params_for_get)* id: i64) -> #rt<#struct_name> {
                    let path = format!(#base_endpoint_lit, #(#parent_names_for_get)*);
                    let endpoint = format!("{}{}/{}", self.endpoint, path, id);
                    rest_api::get(&endpoint).await
                }
            }

            impl #struct_name {
                pub fn url() -> String {
                    #base_endpoint_lit.to_string()
                }
            }
        }
    }

    pub fn generate_action_by_id_struct(&self) -> proc_macro2::TokenStream {
        let struct_name = self.name_id;
        let base_endpoint = &self.base;
        let parent_ids = &self.parent_ids;
        let actions = &self.action_by_id_names;
        let has_validator = self.has_validator;
        #[cfg(feature = "server")]
        let repo_update_st = self.repository_update_request_st_name();
        #[cfg(feature = "server")]
        let mut enum_into_arms = vec![];

        if actions.is_empty() {
            return quote! {};
        }
        let action_type = "ByIdAction";
        let action_name = syn::Ident::new(
            &format!("{}{}", struct_name, action_type),
            struct_name.span(),
        );
        let client_name = syn::Ident::new(&format!("{}Client", struct_name), struct_name.span());
        let base_endpoint_lit = syn::LitStr::new(base_endpoint, struct_name.span());
        let parent_params = parent_ids.iter().map(|id| {
            let id = syn::Ident::new(id, struct_name.span());
            quote! { #id: i64, }
        });
        let parent_names = parent_ids.iter().map(|id| {
            let id = syn::Ident::new(id, struct_name.span());
            quote! { #id, }
        });

        let mut action_fields = vec![];
        let mut action_requests = vec![];
        let mut cli_actions = vec![];
        let mut validates = vec![];
        let validator_derive = if has_validator {
            quote! { #[derive(validator::Validate)] }
        } else {
            quote! {}
        };
        let rt = &self.result_type;

        for (k, v) in actions.iter() {
            let act = syn::Ident::new(&k.to_case(Case::Pascal), struct_name.span());
            let cli_act = syn::Ident::new(&k.to_case(Case::Snake), struct_name.span());
            let request_struct_name = match v {
                ActionField::Related(st) => syn::Ident::new(&st, struct_name.span()),
                _ => syn::Ident::new(
                    &format!("{}{}Request", struct_name, act),
                    struct_name.span(),
                ),
            };
            action_fields.push(quote! {
                #act(#request_struct_name),
            });

            #[cfg(feature = "server")]
            enum_into_arms.push(quote! {
                #action_name::#act(req) => req.into(),
            });

            if let ActionField::Fields(v) = v {
                let mut fields = vec![];
                let mut params = vec![];
                let mut field_names = vec![];
                #[cfg(feature = "server")]
                let mut into_fields = vec![];

                for field in v.iter() {
                    let field_name = &field.ident;
                    let field_type = &field.ty;
                    let mut validate_attributes = Vec::new();

                    for attr in &field.attrs {
                        if let Meta::List(meta_list) = attr.meta.clone() {
                            if meta_list.path.is_ident("validate") {
                                validate_attributes.push(attr.clone());
                            }
                        }
                    }

                    fields.push(quote! {
                        #(#validate_attributes)*
                        pub #field_name: #field_type,
                    });
                    params.push(quote! { #field_name: #field_type, });
                    field_names.push(quote! { #field_name, });
                    #[cfg(feature = "server")]
                    {
                        if self
                            .fields
                            .get(
                                &field_name
                                    .to_token_stream()
                                    .to_string()
                                    .to_case(self.rename),
                            )
                            .unwrap()
                            .is_option()
                        {
                            into_fields.push(quote! { #field_name: self.#field_name, });
                        } else {
                            into_fields.push(quote! { #field_name: Some(self.#field_name), });
                        }
                    }
                }

                for field in self.actions.action_by_id.get(k).clone().unwrap_or(&vec![]) {
                    let field_name = syn::Ident::new(&field.name, struct_name.span());
                    let field_type = field.ty();

                    fields.push(quote! {
                        pub #field_name: #field_type,
                    });
                    params.push(quote! { #field_name: #field_type, });
                    field_names.push(quote! { #field_name, });
                }

                #[cfg(feature = "server")]
                action_requests.push(quote! {
                    #validator_derive
                    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq)]
                    #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
                    pub struct #request_struct_name {
                        #(#fields)*
                    }

                    impl Into<#repo_update_st> for #request_struct_name {
                        fn into(self) -> #repo_update_st {
                            #repo_update_st {
                                #(#into_fields)*
                                ..Default::default()
                            }
                        }
                    }
                });

                #[cfg(not(feature = "server"))]
                action_requests.push(quote! {
                    #validator_derive
                    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq)]
                    #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
                    pub struct #request_struct_name {
                        #(#fields)*
                    }
                });
                validates.push(quote! {
                    #action_name::#act(req) => req.validate(),
                });

                let parent_params = parent_params.clone();
                let parent_names = parent_names.clone();

                let res_type = self.get_reponse_type(&k.to_case(Case::Snake));

                cli_actions.push(quote! {
                    pub async fn #cli_act(&self, #(#parent_params)* id: i64, #(#params)*) -> #rt<#res_type> {
                        let path = format!(#base_endpoint_lit, #(#parent_names)*);
                        let endpoint = format!("{}{}/{}", self.endpoint, path, id);
                        let req = #action_name::#act(#request_struct_name {
                            #(#field_names)*
                        });
                        rest_api::post(&endpoint, req).await
                    }

                })
            } else if let ActionField::Related(st) = v {
                let parent_params = parent_params.clone();
                let parent_names = parent_names.clone();
                let req_type = syn::Ident::new(&st, struct_name.span());

                let res_type = self.get_reponse_type(&k.to_case(Case::Snake));

                cli_actions.push(quote! {
                    pub async fn #cli_act(&self, #(#parent_params)* id: i64, request: #req_type) -> #rt<#res_type> {
                        let path = format!(#base_endpoint_lit, #(#parent_names)*);
                        let endpoint = format!("{}{}/{}", self.endpoint, path, id);

                        rest_api::post(&endpoint, request).await
                    }
                })
            }
        }

        let validate_function = if has_validator {
            quote! {
                impl validator::Validate for #action_name {
                    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
                        match self {
                            #(#validates)*
                        }
                    }
                }

            }
        } else {
            quote! {}
        };
        let rt = &self.result_type;

        let output = quote! {
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
            #[serde(rename_all = "snake_case")]
            #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
            pub enum #action_name {
                #(#action_fields)*
            }

            #validate_function

            #(#action_requests)*

            impl #client_name {
                pub async fn act_by_id(&self, #(#parent_params)* id: i64, params: #action_name) -> #rt<#struct_name> {
                    let path = format!(#base_endpoint_lit, #(#parent_names)*);
                    let endpoint = format!("{}{}/{}", self.endpoint, path, id);
                    rest_api::post(&endpoint, params).await
                }

                #(#cli_actions)*
            }
        };

        #[cfg(feature = "server")]
        let output = quote! {
            #output

            impl Into<#repo_update_st> for #action_name {
                fn into(self) -> #repo_update_st {
                    match self {
                        #(#enum_into_arms)*
                    }
                }
            }
        };

        output
    }

    pub fn generate_read_struct(&self) -> proc_macro2::TokenStream {
        let struct_name = self.name_id;
        let base_endpoint = &self.base;
        let parent_ids = &self.parent_ids;
        let read_actions = &self.read_action_names;
        let has_validator = self.has_validator;

        let base_endpoint_lit = syn::LitStr::new(base_endpoint, struct_name.span());
        let read_action_struct_name =
            syn::Ident::new(&format!("{}ReadAction", struct_name), struct_name.span());
        let client_name = syn::Ident::new(&format!("{}Client", struct_name), struct_name.span());
        let param_name = syn::Ident::new(&format!("{}Param", struct_name), struct_name.span());
        let parent_params = parent_ids.iter().map(|id| {
            let id = syn::Ident::new(id, struct_name.span());
            quote! { #id: i64, }
        });
        let parent_names = parent_ids.iter().map(|id| {
            let id = syn::Ident::new(id, struct_name.span());
            quote! { #id, }
        });

        let mut hashed_fields = HashSet::new();
        let mut query_builder_functions = vec![];
        let mut cli_read_action_functions = vec![];

        let mut query_fields = vec![];
        let mut read_action_types = vec![];
        let mut parsers = vec![];

        let rt = &self.result_type;

        for (read_action, fields) in read_actions {
            let mut params = vec![];
            let mut replace_expressions = vec![];

            match fields {
                ActionField::Fields(v) => {
                    for field in v {
                        let field_name = &field.ident;
                        let field_type = &field.ty;

                        replace_expressions.push(quote! {
                            self.#field_name = Some(#field_name);
                        });
                        params.push((field_name.clone(), field_type.clone()));
                        if hashed_fields.contains(field_name) {
                            continue;
                        }
                        hashed_fields.insert(field_name.clone());

                        let mut validate_attributes = Vec::new();
                        for attr in &field.attrs {
                            if let Meta::List(meta_list) = attr.meta.clone() {
                                if meta_list.path.is_ident("validate") {
                                    validate_attributes.push(attr.clone());
                                }
                            }
                        }
                        let convert = match to_string(&field_type).as_str() {
                            "String" => quote! {},
                            _ => {
                                let fname = syn::Ident::new(
                                    &format!(
                                        "parse_{}_of_{}_read",
                                        field_name
                                            .clone()
                                            .unwrap()
                                            .to_string()
                                            .to_case(Case::Snake),
                                        struct_name.to_string().to_case(Case::Snake)
                                    ),
                                    struct_name.span(),
                                );
                                let fname_str = syn::LitStr::new(
                                    &format!(
                                        "parse_{}_of_{}_read",
                                        field_name
                                            .clone()
                                            .unwrap()
                                            .to_string()
                                            .to_case(Case::Snake),
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
                    }
                }
                _ => {
                    panic!("Related field should not be in queryable fields");
                }
            }

            let read_action_name =
                syn::Ident::new(&read_action.to_case(Case::Pascal), struct_name.span());
            read_action_types.push(quote! { #read_action_name, });

            let function_name =
                syn::Ident::new(&read_action.to_case(Case::Snake), struct_name.span());
            let mut function_params = params
                .iter()
                .map(|(field_name, field_type)| quote! { #field_name: #field_type, });

            let read_action_enum_name = syn::Ident::new(
                &format!("{}ReadActionType", struct_name),
                struct_name.span(),
            );

            query_builder_functions.push(quote! {
                pub fn #function_name(mut self, #(#function_params)*) -> Self {
                    #(#replace_expressions)*
                    self.action = Some(#read_action_enum_name::#read_action_name);
                    self
                }
            });
            let function_params = params
                .iter()
                .map(|(field_name, field_type)| quote! { #field_name: #field_type, });
            let field_names = params.iter().map(|(field_name, _)| quote! { #field_name, });

            let parent_params = parent_params.clone();
            let parent_names = parent_names.clone();
            let res_type = self.get_reponse_type(&read_action.to_case(Case::Snake));

            cli_read_action_functions.push(quote! {
                pub async fn #function_name(
                    &self,
                    #(#parent_params)*
                    #(#function_params)*
                ) -> #rt<#res_type> {
                    let path = format!(#base_endpoint_lit, #(#parent_names)*);
                    let endpoint = format!("{}{}", self.endpoint, path);
                    let params = #read_action_struct_name::new()
                        .#function_name(#(#field_names)*);
                    let query = format!("{}?{}", endpoint, #param_name::Read(params));
                    rest_api::get(&query).await
                }
            })
        }

        let validator_derive = if has_validator {
            quote! { #[derive(validator::Validate)] }
        } else {
            quote! {}
        };

        let (read_action_enum, read_action_type_field) = if read_action_types.len() > 0 {
            let read_action_enum_name = syn::Ident::new(
                &format!("{}ReadActionType", struct_name),
                struct_name.span(),
            );
            (
                quote! {
                    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
                    #[serde(rename_all = "kebab-case")]
                    #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
                    pub enum #read_action_enum_name {
                        #(#read_action_types)*
                    }
                },
                quote! {
                    pub action: Option<#read_action_enum_name>,
                },
            )
        } else {
            (quote! {}, quote! {})
        };

        quote! {
            #validator_derive
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq, by_macros::QueryDisplay)]
            #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
            pub struct #read_action_struct_name {
                #read_action_type_field
                #(#query_fields)*
            }

            #(#parsers)*

            impl #read_action_struct_name {
                pub fn new() -> Self {
                    Self::default()
                }

                #(#query_builder_functions)*
            }

            #read_action_enum


            impl #client_name {
                #(#cli_read_action_functions)*
            }
        }
    }

    pub fn generate_summary_struct(&self) -> proc_macro2::TokenStream {
        let struct_name = self.name_id;
        let summary_fields = &self.summary_fields;
        let s_fields = summary_fields
            .iter()
            .map(|field| {
                let id = field.ident.clone();
                quote! { #id }
            })
            .collect::<Vec<_>>();

        let summary_name = syn::Ident::new(&format!("{}Summary", struct_name), struct_name.span());
        let fields = summary_fields.iter().map(|field| {
            let field_name = &field.ident;
            let field_type = &field.ty;
            quote! { pub #field_name: #field_type, }
        });

        quote! {
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq)]
            #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo, sqlx::FromRow))]
            pub struct #summary_name {
                #(#fields)*
            }

            impl From<#struct_name> for #summary_name {
                fn from(item: #struct_name) -> Self {
                    Self {
                        #(#s_fields: item.#s_fields,)*
                    }
                }
            }

            impl Into<#struct_name> for #summary_name {
                fn into(self) -> #struct_name {
                    #struct_name {
                        #(#s_fields: self.#s_fields,)*
                        ..Default::default()
                    }
                }
            }
        }
    }

    pub fn generate_query_struct(&self) -> proc_macro2::TokenStream {
        let struct_name = self.name_id;
        let base_endpoint = &self.base;
        let parent_ids = &self.parent_ids;
        let query_actions = &self.query_action_names;
        let has_validator = self.has_validator;
        let queryable_fields = &self.queryable_fields;

        let summary_name = syn::Ident::new(&format!("{}Summary", struct_name), struct_name.span());
        let client_name = syn::Ident::new(&format!("{}Client", struct_name), struct_name.span());
        let query_name = syn::Ident::new(&format!("{}Query", struct_name), struct_name.span());
        let param_name = syn::Ident::new(&format!("{}Param", struct_name), struct_name.span());
        let base_endpoint_lit = syn::LitStr::new(base_endpoint, struct_name.span());

        let mut hashed_fields = HashSet::new();
        let mut query_fields = vec![];
        let mut query_builder_functions = vec![];
        let mut cli_read_action_functions = vec![];
        let mut parsers = vec![];
        let rt = &self.result_type;
        parse_queryable_fields(
            &queryable_fields,
            &struct_name,
            &mut hashed_fields,
            &mut query_fields,
            &mut query_builder_functions,
            &mut parsers,
        );
        parse_queryable_action_fields(
            &self.actions.queryable,
            &struct_name,
            &mut hashed_fields,
            &mut query_fields,
            &mut query_builder_functions,
            &mut parsers,
        );

        let mut extended_query_fields = vec![];
        let mut read_action_types = vec![];

        for (read_action, fields) in query_actions {
            let mut params = vec![];
            let mut replace_expressions = vec![];

            match fields {
                ActionField::Fields(v) => {
                    for field in v {
                        let field_name = &field.ident;
                        let field_type = &field.ty;

                        replace_expressions.push(quote! {
                            self.#field_name = Some(#field_name);
                        });
                        params.push((field_name.clone(), field_type.clone()));
                        if hashed_fields.contains(field_name) {
                            continue;
                        }

                        hashed_fields.insert(field_name.clone());

                        let convert = match to_string(&field_type).as_str() {
                            "String" => quote! {},
                            _ => {
                                let fname = syn::Ident::new(
                                    &format!(
                                        "parse_{}_of_{}_query",
                                        field_name
                                            .clone()
                                            .unwrap()
                                            .to_string()
                                            .to_case(Case::Snake),
                                        struct_name.to_string().to_case(Case::Snake)
                                    ),
                                    struct_name.span(),
                                );
                                let fname_str = syn::LitStr::new(
                                    &format!(
                                        "parse_{}_of_{}_query",
                                        field_name
                                            .clone()
                                            .unwrap()
                                            .to_string()
                                            .to_case(Case::Snake),
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
                                    tracing::debug!("Parsing field: {}", #fname_str);

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

                        let mut validate_attributes = Vec::new();
                        for attr in &field.attrs {
                            if let Meta::List(meta_list) = attr.meta.clone() {
                                if meta_list.path.is_ident("validate") {
                                    validate_attributes.push(attr.clone());
                                }
                            }
                        }
                        extended_query_fields.push(quote! {
                            #(#validate_attributes)*
                            #convert
                            pub #field_name: Option<#field_type>,
                        });
                    }
                }
                _ => {
                    panic!("Related field should not be in queryable fields");
                }
            }

            let read_action_name =
                syn::Ident::new(&read_action.to_case(Case::Pascal), struct_name.span());
            read_action_types.push(quote! { #read_action_name, });

            let function_name =
                syn::Ident::new(&read_action.to_case(Case::Snake), struct_name.span());
            let function_params = params.iter().map(|(field_name, field_type)| {
                quote! { #field_name: #field_type, }
            });
            let read_action_enum_name = syn::Ident::new(
                &format!("{}QueryActionType", struct_name),
                struct_name.span(),
            );

            query_builder_functions.push(quote! {
                pub fn #function_name(mut self, #(#function_params)*) -> Self {
                    #(#replace_expressions)*
                    self.action = Some(#read_action_enum_name::#read_action_name);
                    self
                }
            });

            let mut function_params = vec![];

            for (field_name, field_type) in params.iter() {
                function_params.push(quote! { #field_name: #field_type, });
            }
            let field_names = params
                .iter()
                .map(|(field_name, _)| quote! { #field_name: Some(#field_name), });
            let parent_params = parent_ids.iter().map(|id| {
                let id = syn::Ident::new(id, struct_name.span());
                quote! { #id: i64, }
            });
            let parent_names = parent_ids.iter().map(|id| {
                let id = syn::Ident::new(id, struct_name.span());
                quote! { #id, }
            });
            let iter_type_tokens = self.get_iter_reponse_type(&read_action.to_case(Case::Snake));

            cli_read_action_functions.push(quote! {
                pub async fn #function_name(
                    &self,
                    size: usize,
                    bookmark: Option<String>,
                    #(#parent_params)*
                    #(#function_params)*
                ) -> #rt<#iter_type_tokens> {
                    let path = format!(#base_endpoint_lit, #(#parent_names)*);
                    let endpoint = format!("{}{}", self.endpoint, path);

                    let params = #param_name::Query(#query_name {
                        size,
                        bookmark,
                        action: Some(#read_action_enum_name::#read_action_name),
                        #(#field_names)*
                        ..#query_name::default()
                    });
                    let query = format!("{}?{}", endpoint, params);
                    rest_api::get(&query).await
                }
            })
        }

        let (read_action_enum, read_action_type_field) = if read_action_types.len() > 0 {
            let read_action_enum_name = syn::Ident::new(
                &format!("{}QueryActionType", struct_name),
                struct_name.span(),
            );
            (
                quote! {
                    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
                    #[serde(rename_all = "kebab-case")]
                    #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
                    pub enum #read_action_enum_name {
                        #(#read_action_types)*
                    }
                },
                quote! {
                    pub action: Option<#read_action_enum_name>,
                },
            )
        } else {
            (quote! {}, quote! {})
        };

        let validator_derive = if has_validator {
            quote! { #[derive(validator::Validate)] }
        } else {
            quote! {}
        };

        let f = format!(
            "parse_size_of_{}_query",
            struct_name.to_string().to_case(Case::Snake)
        );

        let size_fname = syn::Ident::new(&f, struct_name.span());
        let size_fname_str = syn::LitStr::new(&f, struct_name.span());

        quote! {
            #validator_derive
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq, by_macros::QueryDisplay)]
            #[serde(rename_all = "kebab-case")]
            #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
            pub struct #query_name {
                #[serde(deserialize_with = #size_fname_str, default)]
                pub size: usize,
                pub bookmark: Option<String>,
                #read_action_type_field
                #(#query_fields)*
                #(#extended_query_fields)*
            }

            pub fn #size_fname<'de, D>(deserializer: D) -> std::result::Result<usize, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use serde::Deserialize;

                let s: Option<String> = Option::deserialize(deserializer)?;
                s.unwrap_or_else(|| Default::default())
                    .parse::<usize>()
                    .map_err(serde::de::Error::custom)
            }

            impl #query_name {
                pub fn new(size: usize) -> Self {
                    Self {
                        size,
                        ..Self::default()
                    }
                }

                pub fn with_bookmark(mut self, bookmark: String) -> Self {
                    self.bookmark = Some(bookmark);
                    self
                }

                pub fn with_page(mut self, page: usize) -> Self {
                    self.bookmark = Some(page.to_string());
                    self
                }

                /// Returns the size(i32) of the query
                pub fn size(&self) -> i32 {
                    self.size as i32
                }

                pub fn page(&self) -> i32 {
                    self.bookmark
                        .as_ref()
                        .unwrap_or(&"1".to_string())
                        .parse()
                        .unwrap_or(1)
                }

                #(#query_builder_functions)*

            }
            #(#parsers)*

            #read_action_enum

            impl #client_name {
                #(#cli_read_action_functions)*
            }
        }
    }

    pub fn get_iter_reponse_type(&self, action_name: &str) -> proc_macro2::TokenStream {
        if let Some(resp) = self.actions.responses.get(action_name) {
            if resp.len() == 1 {
                return resp[0].name.parse().unwrap();
            } else {
                panic!(
                    "`response` must be formed as `response = [action_name(CustomResponseType)]`"
                );
            }
        }

        self.iter_type_name()
    }

    pub fn get_reponse_type(&self, action_name: &str) -> proc_macro2::TokenStream {
        if let Some(resp) = self.actions.responses.get(action_name) {
            if resp.len() == 1 {
                return resp[0].name.parse().unwrap();
            } else {
                panic!(
                    "`response` must be formed as `response = [action_name(CustomResponseType)]`"
                );
            }
        }
        let resp_name = self.name_id;

        resp_name.to_token_stream()
    }

    pub fn generate_action_struct(&self) -> proc_macro2::TokenStream {
        tracing::trace!("Generating action struct for {}", self.name_id);
        let struct_name = self.name_id;
        let base_endpoint = &self.base;
        let parent_ids = &self.parent_ids;
        let actions = &self.action_names;
        let has_validator = self.has_validator;

        if actions.is_empty() {
            return quote! {};
        }

        let action_type = "Action";
        let action_name = syn::Ident::new(
            &format!("{}{}", struct_name, action_type),
            struct_name.span(),
        );
        let client_name = syn::Ident::new(&format!("{}Client", struct_name), struct_name.span());
        let base_endpoint_lit = syn::LitStr::new(base_endpoint, struct_name.span());
        let parent_params = parent_ids.iter().map(|id| {
            let id = syn::Ident::new(id, struct_name.span());
            quote! { #id: i64, }
        });
        let parent_names = parent_ids.iter().map(|id| {
            let id = syn::Ident::new(id, struct_name.span());
            quote! { #id, }
        });

        let mut action_fields = vec![];
        let mut action_requests = vec![];
        let mut cli_actions = vec![];
        let validator_derive = if has_validator {
            quote! { #[derive(validator::Validate)] }
        } else {
            quote! {}
        };
        let mut validates = vec![];
        let rt = &self.result_type;

        for (k, v) in actions.iter() {
            tracing::trace!("Processing action: {}", k);
            let act = syn::Ident::new(&k.to_case(Case::Pascal), struct_name.span());
            let cli_act = syn::Ident::new(&k.to_case(Case::Snake), struct_name.span());
            let request_struct_name = match v {
                ActionField::Related(st) => syn::Ident::new(&st, struct_name.span()),
                _ => syn::Ident::new(
                    &format!("{}{}Request", struct_name, act),
                    struct_name.span(),
                ),
            };
            action_fields.push(quote! {
                #act(#request_struct_name),
            });

            if let ActionField::Fields(v) = v {
                let mut fields = vec![];
                let mut params = vec![];
                let mut field_names = vec![];

                for field in v.iter() {
                    let field_name = &field.ident;
                    let field_type = &field.ty;
                    let mut validate_attributes = Vec::new();

                    for attr in &field.attrs {
                        if let Meta::List(meta_list) = attr.meta.clone() {
                            if meta_list.path.is_ident("validate") {
                                validate_attributes.push(attr.clone());
                            }
                        }
                    }

                    fields.push(quote! {
                        #(#validate_attributes)*
                        pub #field_name: #field_type,
                    });
                    params.push(quote! { #field_name: #field_type, });
                    field_names.push(quote! { #field_name, });
                }

                for field in self.actions.actions.get(k).clone().unwrap_or(&vec![]) {
                    let field_name = syn::Ident::new(&field.name, struct_name.span());
                    let field_type = field.ty();

                    fields.push(quote! {
                        pub #field_name: #field_type,
                    });
                    params.push(quote! { #field_name: #field_type, });
                    field_names.push(quote! { #field_name, });
                }

                action_requests.push(quote! {
                #validator_derive
                #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq)]
                #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
                pub struct #request_struct_name {
                    #(#fields)*
                }
            });
                let parent_params = parent_params.clone();
                let parent_names = parent_names.clone();
                validates.push(quote! {
                    #action_name::#act(req) => req.validate(),
                });

                let res_type = self.get_reponse_type(&k.to_case(Case::Snake));
                cli_actions.push(quote! {
                    pub async fn #cli_act(&self, #(#parent_params)* #(#params)*) -> #rt<#res_type> {
                        let path = format!(#base_endpoint_lit, #(#parent_names)*);
                        let endpoint = format!("{}{}", self.endpoint, path);

                        let req = #action_name::#act(#request_struct_name {
                            #(#field_names)*
                        });
                        rest_api::post(&endpoint, req).await
                    }

                })
            } else if let ActionField::Related(st) = v {
                let parent_params = parent_params.clone();
                let parent_names = parent_names.clone();
                let req_type = syn::Ident::new(&st, struct_name.span());
                let res_type = self.get_reponse_type(&k.to_case(Case::Snake));

                cli_actions.push(quote! {
                pub async fn #cli_act(&self, #(#parent_params)* request: #req_type) -> #rt<#res_type> {
                    let path = format!(#base_endpoint_lit, #(#parent_names)*);
                    let endpoint = format!("{}{}", self.endpoint, path);

                    rest_api::post(&endpoint, request).await
                }

            })
            }
        }

        tracing::trace!("Finished parsing actions");

        let validate_function = if has_validator {
            quote! {
                impl validator::Validate for #action_name {
                    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
                        match self {
                            #(#validates)*
                        }
                    }
                }

            }
        } else {
            quote! {}
        };

        quote! {

            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
            #[serde(rename_all = "snake_case")]
            #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
            pub enum #action_name {
                #(#action_fields)*
            }

            #validate_function

            #(#action_requests)*

            impl #client_name {
                pub async fn act(&self, #(#parent_params)* params: #action_name) -> #rt<#struct_name> {
                    let path = format!(#base_endpoint_lit, #(#parent_names)*);
                    let endpoint = format!("{}{}", self.endpoint, path);
                    rest_api::post(&endpoint, params).await
                }

                #(#cli_actions)*
            }
        }
    }

    pub fn derives(&self) -> proc_macro2::TokenStream {
        if self.has_validator {
            quote! { #[derive(validator::Validate)] }
        } else {
            quote! {}
        }
    }

    pub fn query_fields(&self) -> Vec<syn::Ident> {
        let mut hashed_fields = HashSet::new();
        let mut fields = vec![];

        for (_, f) in &self.query_action_names {
            match f {
                ActionField::Fields(v) => {
                    for field in v {
                        let field_name = &field.ident;

                        if hashed_fields.contains(field_name) {
                            continue;
                        }
                        hashed_fields.insert(field_name.clone());
                        fields.push(field_name.clone().unwrap());
                    }
                }
                _ => {
                    panic!("Related field should not be in queryable fields");
                }
            }
        }

        for f in self.queryable_fields.iter() {
            let field_name = &f.ident;

            if hashed_fields.contains(field_name) {
                continue;
            }
            hashed_fields.insert(field_name.clone());
            fields.push(field_name.clone().unwrap());
        }

        tracing::trace!("Query fields: {:?}", fields);

        fields
    }

    pub fn read_action_fields(&self) -> Vec<syn::Ident> {
        let mut hashed_fields = HashSet::new();
        let mut fields = vec![];

        for (_, f) in &self.read_action_names {
            match f {
                ActionField::Fields(v) => {
                    for field in v {
                        let field_name = &field.ident;

                        if hashed_fields.contains(field_name) {
                            continue;
                        }
                        hashed_fields.insert(field_name.clone());
                        fields.push(field_name.clone().unwrap());
                    }
                }
                _ => {
                    panic!("Related field should not be in queryable fields");
                }
            }
        }

        tracing::trace!("Read action fields: {:?}", fields);

        fields
    }
    pub fn struct_name(&self) -> syn::Ident {
        syn::Ident::new(&self.name, proc_macro2::Span::call_site())
    }

    pub fn read_action_struct_name(&self) -> syn::Ident {
        syn::Ident::new(
            &format!("{}ReadAction", self.name),
            proc_macro2::Span::call_site(),
        )
    }

    pub fn query_action_struct_name(&self) -> syn::Ident {
        syn::Ident::new(
            &format!("{}Query", self.name),
            proc_macro2::Span::call_site(),
        )
    }

    pub fn action_struct_name(&self) -> syn::Ident {
        syn::Ident::new(
            &format!("{}Action", self.name),
            proc_macro2::Span::call_site(),
        )
    }

    pub fn action_by_id_struct_name(&self) -> syn::Ident {
        syn::Ident::new(
            &format!("{}ActionById", self.name),
            proc_macro2::Span::call_site(),
        )
    }

    pub fn summary_struct_name(&self) -> syn::Ident {
        syn::Ident::new(
            &format!("{}Summary", self.name),
            proc_macro2::Span::call_site(),
        )
    }

    pub fn queryable_struct_name(&self) -> syn::Ident {
        syn::Ident::new(
            &format!("{}Query", self.name),
            proc_macro2::Span::call_site(),
        )
    }

    pub fn repository_struct_name(&self) -> syn::Ident {
        syn::Ident::new(
            &format!("{}Repository", self.name),
            proc_macro2::Span::call_site(),
        )
    }

    pub fn client_struct_name(&self) -> syn::Ident {
        syn::Ident::new(
            &format!("{}Client", self.name),
            proc_macro2::Span::call_site(),
        )
    }

    pub fn param_struct_name(&self) -> syn::Ident {
        syn::Ident::new(
            &format!("{}Param", self.name),
            proc_macro2::Span::call_site(),
        )
    }

    pub fn get_response_struct_name(&self) -> syn::Ident {
        syn::Ident::new(
            &format!("{}GetResponse", self.name),
            proc_macro2::Span::call_site(),
        )
    }

    pub fn custom_query_type(&self) -> Option<proc_macro2::TokenStream> {
        if self.custom_query_type.is_none() {
            return None;
        }

        let custom_query_type = self.custom_query_type.as_ref().unwrap();

        custom_query_type.parse::<proc_macro2::TokenStream>().ok()
    }

    pub fn iter_type_name(&self) -> proc_macro2::TokenStream {
        if self.should_have_summary() {
            format!("{}<{}Summary>", self.iter_type, self.name)
        } else if &self.iter_type == "Vec" {
            format!("(Vec<{}>, i64)", self.name)
        } else {
            format!("{}<{}>", self.iter_type, self.name)
        }
        .parse()
        .unwrap()
    }

    pub fn param_struct(&self) -> proc_macro2::TokenStream {
        let name = self.param_struct_name();
        let query = self.queryable_struct_name();
        let read = self.read_action_struct_name();
        let mut enums = vec![];

        enums.push(quote! {
            Query(#query),
        });

        if self.should_have_read_action() {
            enums.push(quote! {
                Read(#read),
            });
        }

        if let Some(t) = self.custom_query_type() {
            enums.push(quote! {
                Custom(#t),
            });
        }

        let json_schema = self.generate_jsonschema_for_param();

        let output = quote! {
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, by_macros::QueryDisplay)]
            #[cfg_attr(feature = "server", derive(aide::OperationIo))]
            #[serde(tag = "param-type", rename_all = "kebab-case")]
            pub enum #name {
                #(#enums)*
            }

            #json_schema
        };

        output.into()
    }

    pub fn should_have_queryable(&self) -> bool {
        !self.queryable_fields.is_empty() & !self.query_action_names.is_empty()
    }

    pub fn should_have_summary(&self) -> bool {
        !self.summary_fields.is_empty()
    }

    pub fn should_have_action(&self) -> bool {
        !self.action_names.is_empty()
    }

    pub fn should_have_action_by_id(&self) -> bool {
        !self.action_by_id_names.is_empty()
    }

    pub fn should_have_read_action(&self) -> bool {
        !self.read_action_names.is_empty()
    }

    pub fn get_response_struct(&self) -> proc_macro2::TokenStream {
        let name = self.name_id;
        let mut enums = vec![];
        let iter_type = self.iter_type_name();

        enums.push(quote! {
            Query(#iter_type),
        });

        if self.should_have_read_action() {
            enums.push(quote! {
                Read(#name),
            });
        }

        let response = self.get_response_struct_name();

        let output = quote! {
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
            #[serde(tag = "param_type")]
            #[serde(rename_all = "snake_case")]
            #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
            pub enum #response {
                #(#enums)*
            }
        };

        output.into()
    }
}

#[cfg(feature = "server")]
impl ApiModel<'_> {
    pub fn queries(&self) -> proc_macro2::TokenStream {
        let mut create_query_fields = vec![];
        let mut additional_queries = vec![];

        let (ref primary_key_name, ref primary_key_type) = self.primary_key;

        for (sql_field_name, field) in self.fields.iter() {
            match field.create_field_query_line() {
                Some(query) => {
                    create_query_fields.push(query);
                }
                None => {}
            }

            additional_queries
                .extend(field.get_additional_query(&primary_key_name, &primary_key_type));
        }
        let queries: Vec<syn::LitStr> = additional_queries
            .iter()
            .map(|item| syn::LitStr::new(item, proc_macro2::Span::call_site()))
            .collect();

        let q = format!(
            "CREATE TABLE IF NOT EXISTS {} ({});",
            self.table_name,
            create_query_fields.join(","),
        );
        let create_query_output = syn::LitStr::new(&q, proc_macro2::Span::call_site());
        tracing::trace!("create table query: {}", q);

        quote! {
            pub fn queries(&self) -> Vec<&'static str> {
                vec![#create_query_output, #(#queries),*]
            }
            pub async fn create_this_table(&self) -> std::result::Result<(), sqlx::Error> {
                tracing::trace!("Create table: {}", #create_query_output);
                sqlx::query(#create_query_output).execute(&self.pool).await?;

                Ok(())
            }

            pub async fn create_related_tables(&self) -> std::result::Result<(), sqlx::Error> {
                for query in [#(#queries),*] {
                    tracing::trace!("Execute queries: {}", query);
                    sqlx::query(query).execute(&self.pool).await?;
                }

                Ok(())
            }

            pub async fn create_table(&self) -> std::result::Result<(), sqlx::Error> {
                sqlx::query(#create_query_output).execute(&self.pool).await?;

                for query in [#(#queries),*] {
                    tracing::trace!("Execute queries: {}", query);
                    sqlx::query(query).execute(&self.pool).await?;
                }

                Ok(())
            }

        }
    }

    pub fn drop_function(&self) -> proc_macro2::TokenStream {
        let q = format!("DROP TABLE IF EXISTS {};", self.table_name);
        let drop_table_query = syn::LitStr::new(&q, proc_macro2::Span::call_site());

        quote! {
            pub async fn drop_table(&self) -> std::result::Result<(), sqlx::Error> {
                sqlx::query(#drop_table_query)
                    .execute(&self.pool)
                    .await?;

                Ok(())
            }
        }
    }

    pub fn find_function(&self) -> proc_macro2::TokenStream {
        let name = self.iter_type_name();
        let query_struct = self.query_action_struct_name();
        let fields = self.query_fields();

        let q = syn::LitStr::new(
            &format!("SELECT * FROM {}", self.table_name),
            proc_macro2::Span::call_site(),
        );
        let qc = syn::LitStr::new(
            &format!("SELECT COUNT(*) FROM {}", self.table_name),
            proc_macro2::Span::call_site(),
        );

        let tail_q = syn::LitStr::new(
            &format!("LIMIT ${} OFFSET ${}", 1, 2),
            proc_macro2::Span::call_site(),
        );

        let mut binds = vec![];
        let mut where_clause = vec![];
        let fmt_str = syn::LitStr::new(
            &format!("{}Repository::find_one", self.name),
            proc_macro2::Span::call_site(),
        );

        for f in fields.iter() {
            let fname = syn::LitStr::new(&f.to_string(), proc_macro2::Span::call_site());

            let field = self
                .fields
                .get(&f.to_string().to_case(self.rename))
                .expect(&format!("Field not found: {}", f.to_string()));
            if field.skip {
                continue;
            }

            let bind = field.bind();

            binds.push(quote! {
                if let Some(#f) = &param.#f {
                    tracing::trace!("{} binding {} = {}", #fmt_str, #fname, #f);
                    q = q #bind;
                }
            });

            where_clause.push(quote! {
                if let Some(_) = &param.#f {
                    i += 1;
                    where_clause.push(format!("{} = ${}", #fname, i));
                }
            });
        }

        let call_map = self.call_map_iter_type();
        let declare_where_clause = if fields.len() > 0 {
            quote! {
                let mut i = 2;
                let mut where_clause = vec![];
            }
        } else {
            quote! {}
        };
        let compose_query = if fields.len() > 0 {
            quote! {
                let where_clause_str = where_clause.join(" AND ");
                let query = if where_clause.len() > 0 {
                    format!("{} WHERE {} {}", #q, where_clause_str, #tail_q)
                } else {
                    format!("{} {}", #q, #tail_q)
                };

                let count_query = if where_clause.len() > 0 {
                    format!("{} WHERE {}", #qc, where_clause_str)
                } else {
                    format!("{}", #qc)
                };
                let query = format!("WITH data AS ({}) SELECT ({}) AS total_count, data.* FROM data;", query, count_query);
                tracing::trace!("{} query {}", #fmt_str, query);
                let offset: i32 = (param.size as i32) * (param.page() - 1);
                let mut q = sqlx::query(&query).bind(param.size as i32).bind(offset);
            }
        } else {
            quote! {
                let query = format!("WITH data AS ({} {}) SELECT ({}) AS total_count, data.* FROM data;", #q, #tail_q, #qc);
                tracing::trace!("{} query {}", #fmt_str, query);
                let offset: i32 = (param.size as i32) * (param.page() - 1);
                let q = sqlx::query(&query).bind(param.size as i32).bind(offset);
            }
        };
        let rt = &self.result_type;

        let output = quote! {
            // #[deprecated(note = "Use `query_builder()` instead.")]
            pub async fn find(&self, param: &#query_struct) -> #rt<#name> {
                #declare_where_clause
                #(#where_clause)*

                #compose_query

                #(#binds)*
                let mut total: i64 = 0;
                let rows = q
                    .map(|row: sqlx::postgres::PgRow| {
                        use sqlx::Row;

                        total = row.get("total_count");
                        row.into()
                    })
                .fetch_all(&self.pool).await?;

                Ok((rows, total).into())
            }
        };

        output.into()
    }

    pub fn impl_summary_functions(&self) -> proc_macro2::TokenStream {
        let name = self.summary_struct_name();
        let base_sql_function = self.base_sql_with_function_for_summary();

        quote! {
            impl #name {
                #base_sql_function
            }
        }
    }

    pub fn impl_functions(&self) -> proc_macro2::TokenStream {
        let name = self.name_id;
        let base_sql_function = self.base_sql_function();

        quote! {
            impl #name {
                #base_sql_function
            }
        }
    }

    pub fn group_by(&self) -> proc_macro2::TokenStream {
        let name = self.name_id;
        let mut group_by = vec![];

        for (field_name, field) in self.fields.iter() {
            if let Some(q) = field.group_by() {
                if !group_by.contains(&q) {
                    group_by.push(q);
                }
            }
        }

        let group_by = syn::LitStr::new(&group_by.join(" "), proc_macro2::Span::call_site());

        let output = quote! {
            pub fn group_by() -> String {
                #group_by.to_string()
            }
        };

        output.into()
    }

    pub fn has_join_query(&self) -> bool {
        for (field_name, field) in self.fields.iter() {
            if let Some(query) = field.aggregate_query(&field_name) {
                return true;
            }
        }

        false
    }

    pub fn base_sql_with_function_for_summary(&self) -> proc_macro2::TokenStream {
        let mut aggregates = vec![];
        let mut aggregated_fields = vec![];
        let mut aggregate_args: IndexMap<String, proc_macro2::TokenStream> = IndexMap::new();
        let mut arg_names = vec![];
        let mut group_by = vec![];
        let mut summary_fields = vec![];

        for field in self.summary_fields.iter() {
            let n = field
                .clone()
                .ident
                .unwrap()
                .to_string()
                .to_case(self.rename);
            let mut is_agg = false;
            let field = self.fields.get(&n).expect(&format!("Field not found: {n}"));
            let field_name = field.name.clone();

            if let Some(query) = field.aggregate_query(&field_name) {
                is_agg = true;
                aggregates.push(query)
            }

            if let Some(q) = field.aggregate_expose_query(&field_name) {
                is_agg = true;
                aggregated_fields.push(q);
            }

            for ((name, q)) in field.aggregate_arg() {
                is_agg = true;
                aggregate_args.insert(name.clone(), q);
                let arg_name = syn::Ident::new(&name, proc_macro2::Span::call_site());
                arg_names.push(arg_name);
            }

            if let Some(q) = field.group_by() {
                if !group_by.contains(&q) {
                    group_by.push(q);
                }
            }

            if !is_agg {
                summary_fields.push(n.clone());
            }
        }

        let q = if aggregated_fields.len() > 0 {
            syn::LitStr::new(
                &format!(
                    "SELECT  COUNT(*) OVER() as total_count, {}, {} FROM {} p {}",
                    summary_fields
                        .iter()
                        .map(|x| format!("p.{}", x))
                        .collect::<Vec<_>>()
                        .join(", "),
                    aggregated_fields.join(", "),
                    self.table_name,
                    aggregates.join(" "),
                ),
                proc_macro2::Span::call_site(),
            )
        } else {
            syn::LitStr::new(
                &format!(
                    "SELECT COUNT(*) OVER() as total_count, {} FROM {}",
                    summary_fields
                        .iter()
                        .map(|x| format!("{}", x))
                        .collect::<Vec<_>>()
                        .join(", "),
                    self.table_name
                ),
                proc_macro2::Span::call_site(),
            )
        };

        let group_by = syn::LitStr::new(&group_by.join(" "), proc_macro2::Span::call_site());
        // let qc = syn::LitStr::new(
        //     &format!("SELECT COUNT(*) FROM {}", self.table_name),
        //     proc_macro2::Span::call_site(),
        // );

        let query_builder = self.repo_query_struct_id();
        let summary_name = self.summary_struct_name();
        let aggregate_args: Vec<proc_macro2::TokenStream> = aggregate_args.into_values().collect();
        let output = quote! {
            pub fn base_sql_with(#(#aggregate_args,)* where_and_statements: &str) -> String {
                tracing::debug!("{} base_sql_with group: {}", #q, #group_by);
                let query = if where_and_statements.is_empty() {
                    format!("{} {}", format!(#q, #(#arg_names),*), #group_by)
                } else {
                    if where_and_statements.to_lowercase().starts_with("where") {
                        format!("{} {} {}", format!(#q, #(#arg_names),*), where_and_statements, #group_by)
                    } else {
                        format!("{} WHERE {} {}", format!(#q, #(#arg_names),*), where_and_statements, #group_by)
                    }
                };

                query
            }

            pub fn query_builder(#(#aggregate_args),*) -> #query_builder {
                let base_sql = format!(#q, #(#arg_names),*);
                #query_builder::from(&base_sql, #group_by).with_count()
            }
        };

        output.into()
    }

    pub fn base_sql_function(&self) -> proc_macro2::TokenStream {
        let name = self.name_id;

        let mut aggregates = vec![];
        let mut aggregated_fields = vec![];
        let mut aggregate_args: IndexMap<String, proc_macro2::TokenStream> = IndexMap::new();
        let mut arg_names = vec![];

        for (field_name, field) in self.fields.iter() {
            if let Some(query) = field.aggregate_query(&field_name) {
                aggregates.push(query)
            }

            if let Some(q) = field.aggregate_expose_query(&field_name) {
                aggregated_fields.push(q);
            }

            for ((name, q)) in field.aggregate_arg() {
                aggregate_args.insert(name.clone(), q);
                let arg_name = syn::Ident::new(&name, proc_macro2::Span::call_site());
                arg_names.push(arg_name);
            }
        }

        let q = if aggregated_fields.len() > 0 {
            syn::LitStr::new(
                &format!(
                    "SELECT p.*, {} FROM {} p {}",
                    aggregated_fields.join(", "),
                    self.table_name,
                    aggregates.join(" "),
                ),
                proc_macro2::Span::call_site(),
            )
        } else {
            syn::LitStr::new(
                &format!("SELECT * FROM {}", self.table_name),
                proc_macro2::Span::call_site(),
            )
        };

        let group_by = self.group_by();

        let query_builder = self.repo_query_struct_id();
        let aggregate_args: Vec<proc_macro2::TokenStream> = aggregate_args.into_values().collect();
        let output = quote! {
            pub fn base_sql(#(#aggregate_args),*) -> String {
                format!(#q, #(#arg_names),*)
            }

            #group_by

            pub fn query_builder(#(#aggregate_args),*) -> #query_builder {
                let base_sql = format!(#q, #(#arg_names),*);
                let g = #name::group_by();
                #query_builder::from(&base_sql, &g)
            }
        };

        output.into()
    }

    pub fn find_one_function(&self) -> proc_macro2::TokenStream {
        let name = self.name_id;
        let read_action = self.read_action_struct_name();
        let fields = self.read_action_fields();

        let mut binds = vec![];
        let mut where_clause = vec![];
        let fmt_str = syn::LitStr::new(
            &format!("{}Repository::find_one", self.name),
            proc_macro2::Span::call_site(),
        );

        let mut aggregate_args: IndexMap<String, proc_macro2::TokenStream> = IndexMap::new();
        let mut arg_names = vec![];

        for (field_name, field) in self.fields.iter() {
            for ((name, q)) in field.aggregate_arg() {
                if aggregate_args.insert(name.clone(), q).is_none() {
                    let arg_name = syn::Ident::new(&name, proc_macro2::Span::call_site());
                    arg_names.push(quote! { #arg_name })
                }
            }
        }
        let mut parent_variable = syn::LitStr::new("", proc_macro2::Span::call_site());

        if self.has_join_query() {
            parent_variable = syn::LitStr::new("p.", proc_macro2::Span::call_site());
        }

        for f in fields.iter() {
            let fname = syn::LitStr::new(&f.to_string(), proc_macro2::Span::call_site());
            let field = self
                .fields
                .get(&f.to_string().to_case(self.rename))
                .expect(&format!("Field not found: {}", f.to_string()));

            if field.skip {
                continue;
            }

            let bind = field.bind();

            binds.push(quote! {
                if let Some(#f) = &param.#f {
                    tracing::trace!("{} binding {} = {}", #fmt_str, #fname, #f);
                    q = q #bind;
                }
            });

            where_clause.push(quote! {
                if let Some(_) = &param.#f {
                    i += 1;
                    where_clause.push(format!("{}{} = ${}", #parent_variable, #fname, i));
                }
            });
        }

        let call_map = self.call_map();

        let for_where = if where_clause.len() > 0 {
            quote! {
                let mut i = 0;
                let mut where_clause = vec![];
                #(#where_clause)*
                let where_clause_str = where_clause.join(" AND ");
                let mut query = if where_clause.len() > 0 {
                    format!("{} WHERE {}", #name::base_sql(#(#arg_names),*), where_clause_str)
                } else {
                    format!("{}", #name::base_sql(#(#arg_names),*))
                };
            }
        } else {
            quote! {
                let mut query = format!("{}", #name::base_sql(#(#arg_names),*));
            }
        };

        let rt = &self.result_type;
        let aggregate_args: Vec<proc_macro2::TokenStream> = aggregate_args.into_values().collect();
        let output = quote! {
            // #[deprecated(note = "Use `query_builder()` instead.")]
            pub async fn find_one(&self, #(#aggregate_args,)* param: &#read_action) -> #rt<#name> {
                #for_where
                query.push_str(" ");
                query.push_str(#name::group_by().as_str());
                tracing::trace!("{} query {}: {:?}", #fmt_str, query, param);

                let mut q = sqlx::query(&query);

                #(#binds)*
                let row = q
                    #call_map
                .fetch_one(&self.pool).await?;

                Ok(row)
            }
        };

        output.into()
    }

    pub fn call_map_iter_type(&self) -> proc_macro2::TokenStream {
        if self.should_have_summary() {
            self.call_map_summary()
        } else {
            self.call_map_iter()
        }
    }

    pub fn call_map_summary(&self) -> proc_macro2::TokenStream {
        let name = self.summary_struct_name();
        let inner = self.from_pg_row_summary_inner();

        let output = quote! {
            .map(|row: sqlx::postgres::PgRow| {
                #inner
            })
        };

        output.into()
    }

    pub fn from_pg_row_summary_inner(&self) -> proc_macro2::TokenStream {
        let name = self.summary_struct_name();
        let mut return_bounds = vec![];

        for field in self.summary_fields.iter() {
            let n = field
                .clone()
                .ident
                .unwrap()
                .to_string()
                .to_case(self.rename);
            let field = self.fields.get(&n).expect(&format!("Field not found: {n}"));
            let field_name = field.name.clone();

            let sql_field_name = syn::LitStr::new(&field_name, proc_macro2::Span::call_site());
            let n = field.field_name_token();
            let rust_type = &field.rust_type;

            return_bounds.push(field.call_map());
        }

        let out = quote! {
            use sqlx::Row;

            #name {
                #(#return_bounds),*
            }
        };
        tracing::trace!("From pg row inner: {}", out.to_string());
        out.into()
    }

    pub fn from_pg_row_summary_trait(&self) -> proc_macro2::TokenStream {
        let name = self.summary_struct_name();
        let inner = self.from_pg_row_summary_inner();
        let out = quote! {
            impl From<sqlx::postgres::PgRow> for #name {
                fn from(row: sqlx::postgres::PgRow) -> Self {
                    #inner
                }
            }
        };
        tracing::trace!("From<PgRow> trait for Summary: {}", out.to_string());
        out.into()
    }

    pub fn call_map_iter(&self) -> proc_macro2::TokenStream {
        let name = syn::Ident::new(&self.name, proc_macro2::Span::call_site());
        let mut return_bounds = vec![];

        for (field_name, field) in self.fields.iter() {
            let sql_field_name = syn::LitStr::new(&field_name, proc_macro2::Span::call_site());
            let n = field.field_name_token();
            let rust_type = &field.rust_type;

            return_bounds.push(field.call_map());
        }

        let output = quote! {
            .map(|row: sqlx::postgres::PgRow| {
                use sqlx::Row;

                total = row.get("total_count");

                #name {
                    #(#return_bounds),*
                }
            })
        };

        output.into()
    }

    pub fn call_map(&self) -> proc_macro2::TokenStream {
        let inner = self.from_pg_row_inner();
        let output = quote! {
            .map(|row: sqlx::postgres::PgRow| {
                #inner
            })
        };

        output.into()
    }

    pub fn from_pg_row_inner(&self) -> proc_macro2::TokenStream {
        let name = syn::Ident::new(&self.name, proc_macro2::Span::call_site());
        let mut return_bounds = vec![];

        for (field_name, field) in self.fields.iter() {
            return_bounds.push(field.call_map());
        }

        let out = quote! {
            use sqlx::Row;

            #name {
                #(#return_bounds),*
            }
        };
        tracing::trace!("From pg row inner: {}", out.to_string());
        out.into()
    }

    pub fn from_pg_row_trait(&self) -> proc_macro2::TokenStream {
        let name = syn::Ident::new(&self.name, proc_macro2::Span::call_site());
        let inner = self.from_pg_row_inner();
        let out = quote! {
            impl From<sqlx::postgres::PgRow> for #name {
                fn from(row: sqlx::postgres::PgRow) -> Self {
                    #inner
                }
            }
        };
        tracing::trace!("From pg row trait: {}", out.to_string());
        out.into()
    }

    pub fn insert_function_for_many_to_many(&self) -> proc_macro2::TokenStream {
        let name = self.name_id;
        // let mut dep_args = vec![];
        let mut dep_args: IndexMap<String, proc_macro2::TokenStream> = IndexMap::new();
        let mut joined_query = vec![];

        for (field_name, field) in self.fields.iter() {
            if field.aggregator.is_some() {
                continue;
            }

            if let Some(Relation::ManyToMany {
                ref table_name,
                ref foreign_primary_key,
                ref foreign_reference_key,
                ..
            }) = &field.relation
            {
                let foreign_primary_key_str = foreign_primary_key.clone();
                let foreign_primary_key =
                    syn::Ident::new(&foreign_primary_key, proc_macro2::Span::call_site());
                dep_args.insert(
                    foreign_primary_key_str,
                    quote! {
                        #foreign_primary_key: i64
                    },
                );

                let q = syn::LitStr::new(
                    &format!(
                        "INSERT INTO {} ({}, {}) VALUES ($1, $2)",
                        table_name, foreign_primary_key, foreign_reference_key
                    ),
                    proc_macro2::Span::call_site(),
                );

                joined_query.push(quote! {
                    sqlx::query(#q)
                        .bind(#foreign_primary_key)
                        .bind(id)
                        .execute(&mut *tx)
                        .await?;
                });
            }
        }

        if dep_args.len() == 0 {
            return quote! {};
        }

        let mut insert_fields = vec![];
        let mut insert_values = vec![];

        let mut i = 1;

        let mut returning = vec![];
        let mut args = vec![];
        let mut binds = vec![];

        for (field_name, field) in self.fields.iter() {
            returning.push(field_name.clone());
            let n = field.field_name_token();

            if field.should_skip_inserting() {
                continue;
            }

            args.push(field.arg_token());
            binds.push(field.bind());

            insert_fields.push(field_name.clone());
            insert_values.push(format!("${}", i));

            i += 1;
        }

        let q = syn::LitStr::new(
            &format!(
                "INSERT INTO {} ({}) VALUES ({}) RETURNING id",
                self.table_name,
                insert_fields.join(", "),
                insert_values.join(", "),
            ),
            proc_macro2::Span::call_site(),
        );
        let rt = &self.result_type;
        let dep_args: Vec<proc_macro2::TokenStream> = dep_args.into_values().collect();
        quote! {
            pub async fn insert_with_dependency(&self, #(#dep_args),*, #(#args),*) -> #rt<#name> {
                use sqlx::Row;
                use sqlx::postgres::PgRow;
                let mut tx = self.pool.begin().await?;


                let row: Option<#name> = sqlx::query(#q)
                    #(#binds)*
                .map(|row: PgRow| {
                    row.into()
                })
                .fetch_optional(&mut *tx)
                    .await?;

                let row = match row {
                    None => Err(sqlx::Error::RowNotFound),
                    Some(row) => Ok(row)
                }?;

                let id: i64 = row.id;

                #(#joined_query)*

                tx.commit().await?;
                Ok(row)
            }
        }
    }

    pub fn repo_query_struct_id(&self) -> syn::Ident {
        syn::Ident::new(
            &format!("{}RepositoryQueryBuilder", self.name),
            proc_macro2::Span::call_site(),
        )
    }

    pub fn repo_query_request(&self) -> proc_macro2::TokenStream {
        let name = syn::Ident::new(
            &format!("{}RepositoryQueryBuilder", self.name),
            proc_macro2::Span::call_site(),
        );

        let mut builder_fields = vec![];
        let mut functions = vec![];
        let mut bindings = vec![];
        let mut assigns = vec![];
        let mut conditions = vec![];

        for (_, v) in self.fields.iter() {
            if v.query_builder {
                tracing::trace!("building query_builder condition: {}", v.name);
                let ty = if v.rust_type.starts_with("Vec") {
                    v.rust_type
                        .replace(" ", "")
                        .trim_start_matches("Vec<")
                        .trim_end_matches(">")
                        .to_string()
                } else {
                    v.rust_type.clone()
                };

                let ty: proc_macro2::TokenStream =
                    format!("{}RepositoryQueryBuilder", ty).parse().unwrap();
                let pattern = syn::LitStr::new(
                    &format!(r"-- {} start([\s\S]*?)-- {} end", v.name, v.name),
                    proc_macro2::Span::call_site(),
                );

                let bpattern = syn::LitStr::new(
                    &format!(r"-- {} start -- {} end", v.name, v.name),
                    proc_macro2::Span::call_site(),
                );

                let bound_name =
                    syn::LitStr::new(&format!("{}", v.name), proc_macro2::Span::call_site());

                let name = syn::Ident::new(
                    &format!("{}_builder", v.name),
                    proc_macro2::Span::call_site(),
                );
                builder_fields.push(quote! {
                    pub #name: Option<#ty>,
                });

                functions.push(quote! {
                    pub fn #name(mut self, #name: #ty) -> Self {
                        self.#name = Some(#name);
                        self
                    }
                });

                if let Some(Relation::ManyToMany {
                    table_name,
                    foreign_primary_key,
                    foreign_reference_key,
                    reference_key,
                    target_table: TargetTable::Foreign,
                    ..
                }) = &v.relation
                {
                    let table_name = syn::LitStr::new(table_name, proc_macro2::Span::call_site());

                    let foreign_primary_key =
                        syn::LitStr::new(foreign_primary_key, proc_macro2::Span::call_site());
                    let foreign_reference_key =
                        syn::LitStr::new(foreign_reference_key, proc_macro2::Span::call_site());
                    let reference_key =
                        syn::LitStr::new(reference_key, proc_macro2::Span::call_site());

                    // NOTE: now support only Foreign table target
                    bindings.push(quote! {
                        let ret = if let Some(q) = &self.#name {
                            let mut q = q.clone();
                            let table_name = #table_name;
                            let foreign_primary_key = #foreign_primary_key;
                            let foreign_reference_key = #foreign_reference_key;
                            let reference_key = #reference_key;
                            let bound_name = #bound_name;
                            let w = q.build_where_starts_with(i);

                            let sub_query = if w.is_empty() {
                                format!("{}", q.base_sql)
                            } else {
                                format!("{} WHERE {}", q.base_sql, w)
                            };

                            tracing::trace!("sub_query(before): {sub_query}");
                            let sub_query = format!(r#"
        {sub_query}
        JOIN {table_name} j ON {bound_name}.id = j.{foreign_primary_key}
        WHERE j.{foreign_reference_key} = dummy.{reference_key}
        GROUP BY {bound_name}.id
"#);
                            tracing::trace!("new ready sub_query: {sub_query}");
                            let sub_query = sub_query.replace("p.", &format!("{}.",#bound_name)).replace(" p ", &format!(" {} ", #bound_name)).replace(" dummy.", " p.").to_string();
                            let sub_query = format!("\n{}\n", sub_query);
                            tracing::trace!("sub query(after): {}", sub_query);

                            let re = regex::Regex::new(#pattern).unwrap();
                            re.replace_all(&ret, #bpattern).to_string().replace(#bpattern, &sub_query).to_string()
                        } else {
                            ret
                        };

                        tracing::trace!("modified query: \n{}", ret);
                    });
                } else if let Some(Relation::OneToMany {
                    foreign_key,
                    reference_key,
                    ..
                }) = &v.relation
                {
                    let foreign_key = syn::LitStr::new(
                        &format!("{} = dummy.{}", foreign_key, reference_key),
                        proc_macro2::Span::call_site(),
                    );

                    bindings.push(quote! {
                        let ret = if let Some(q) = &self.#name {
                            let mut q = q.clone();
                            q.conditions.push(by_types::Conditions::Custom(#foreign_key.to_string()));
                            let sub_query = q.sql_starts_with(i);
                            let sub_query = sub_query.replace("p.", &format!("{}.",#bound_name)).replace(" p ", &format!(" {} ", #bound_name)).replace(" dummy.", " p.").to_string();
                            let sub_query = format!("\n{}\n", sub_query);
                            tracing::trace!("sub query(after): {}", sub_query);

                            let re = regex::Regex::new(#pattern).unwrap();
                            re.replace_all(&ret, #bpattern).to_string().replace(#bpattern, &sub_query).to_string()
                        } else {
                            ret
                        };

                        tracing::trace!("modified query: {}", ret);
                    });
                }
                assigns.push(quote! {
                    #name: self.#name,
                });
                conditions.push(quote! {
                    if let Some(q) = &self.#name {
                        conditions.extend(q.all_conditions());
                    }
                });
                continue;
            }

            if !v.can_query() {
                continue;
            }

            let name = syn::Ident::new(&v.name, proc_macro2::Span::call_site());
            let ty = v.unwrapped_type_token();
            let field_id_str = syn::LitStr::new(&v.name, proc_macro2::Span::call_site());

            match ty.to_string().as_str() {
                "i32" | "u32" => {
                    let ty = ty.to_string();
                    let f = build_integer_query_functions(&v.name, &ty);
                    let o = build_order_by_functions(&v.name);

                    functions.push(quote! {
                        #f
                        #o
                    });
                }
                "i64" | "u64" => {
                    let ty = ty.to_string();
                    let f = build_bigint_query_functions(&v.name, &ty);
                    let o = build_order_by_functions(&v.name);

                    functions.push(quote! {
                        #f
                        #o
                    });
                }
                "String" => {
                    let ty = ty.to_string();
                    let f = build_string_query_functions(&v.name, &ty);
                    let o = build_order_by_functions(&v.name);

                    functions.push(quote! {
                        #f
                        #o
                    })
                }
                "bool" => {
                    let f = build_boolean_query_functions(&v.name);

                    functions.push(quote! {
                        #f
                    })
                }
                _ => match v.r#type.to_lowercase().as_str() {
                    "integer" => {
                        let ty = ty.to_string();
                        let f = build_integer_query_functions(&v.name, &ty);
                        let o = build_order_by_functions(&v.name);

                        functions.push(quote! {
                            #f
                            #o
                        });
                    }
                    _ => {}
                },
            }
        }

        functions.push(quote! {
            pub fn order_by_random(mut self) -> Self {
                self.order = by_types::Order::Random;
                self
            }
        });
        let ns = syn::LitStr::new(&name.to_string(), proc_macro2::Span::call_site());

        quote! {
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
            pub struct #name {
                pub base_sql: String,
                pub group_by: String,
                pub count: bool,
                pub conditions: Vec<by_types::Conditions>,
                pub order: by_types::Order,
                pub limit: Option<i32>,
                pub page: Option<i32>,
                pub or: Vec<Vec<by_types::Conditions>>,
                #(#builder_fields)*
            }

            impl std::ops::BitOr for #name {
                type Output = Self;

                fn bitor(self, rhs: Self) -> Self::Output {
                    let mut new_or = self.or;
                    new_or.push(rhs.conditions); // Append rhs conditions to or
                    if !rhs.or.is_empty() {
                        new_or.extend(rhs.or);
                    }

                    Self {
                        base_sql: self.base_sql,
                        group_by: self.group_by,
                        count: self.count || rhs.count,
                        conditions: self.conditions,
                        order: self.order,
                        limit: self.limit.or(rhs.limit),
                        page: self.page.or(rhs.page),
                        or: new_or,
                        #(#assigns)*
                    }
                }
            }

            impl std::ops::BitOrAssign for #name {
                fn bitor_assign(&mut self, rhs: Self) {
                    self.or.push(rhs.conditions);
                    if !rhs.or.is_empty() {
                        self.or.extend(rhs.or);
                    }
                    self.count = self.count || rhs.count;
                    self.limit = self.limit.or(rhs.limit);
                    self.page = self.page.or(rhs.page);
                }
            }

            impl #name {
                pub fn from(base_sql: &str, group_by: &str) -> Self {
                    Self {
                        base_sql: base_sql.to_string(),
                        group_by: group_by.to_string(),
                        ..Default::default()
                    }
                }

                pub fn with_count(mut self) -> Self {
                    self.count = true;
                    self
                }

                pub fn new() -> Self {
                    Self::default()
                }

                pub fn limit(mut self, limit: i32) -> Self {
                    self.limit = Some(limit);
                    self
                }

                pub fn page(mut self, page: i32) -> Self {
                    self.page = Some(page);
                    self
                }

                pub fn build_where_starts_with(&self, i:&mut i32) -> String {
                    let mut where_clause = vec![];
                    tracing::debug!("Building where clause for {}", #ns);
                    let prefix = if self.group_by.is_empty() {
                        ""
                    } else {
                        "p."
                    };

                    for condition in self.conditions.iter() {
                        let (q, new_i) = condition.to_binder(*i);
                        *i = new_i;
                        where_clause.push(format!("{}{}", prefix, q));
                    }
                    tracing::debug!("conditions: {:?}", where_clause);

                    let mut ret = vec![where_clause.join(" AND ")];

                    for conditions in self.or.iter() {
                        let mut where_clause = vec![];

                        for condition in conditions.iter() {
                            let (q, new_i) = condition.to_binder(*i);
                            *i = new_i;
                            where_clause.push(format!("{}{}", prefix, q));
                        }
                        ret.push(where_clause.join(" AND "));
                    }

                    if ret.len() == 1 {
                        ret[0].clone()
                    } else {
                        format!("({})", ret.join(") OR ("))
                    }
                }

                pub fn build_where(&self) -> String {
                    let mut _i = 1;
                    self.build_where_starts_with(&mut _i)
                }

                pub fn sql_starts_with(&self, i: &mut i32) -> String {
                    let w = self.build_where_starts_with(i);

                    let mut query = if w.is_empty() {
                        format!("{} {} {}", self.base_sql, self.group_by, self.order)
                    } else {
                        format!("{} WHERE {} {} {}", self.base_sql, w, self.group_by, self.order)
                    };

                    if self.count && !query.starts_with("SELECT COUNT(*) OVER() as total_count") {
                        query = query.replacen("SELECT", "SELECT COUNT(*) OVER() as total_count,", 1);
                    }

                    let ret = if let Some(limit) = self.limit {
                        if let Some(page) = self.page {
                            format!("{} LIMIT {} OFFSET {}", query, limit, (limit * (page - 1)))
                        } else {
                            format!("{} LIMIT {}", query, limit)
                        }
                    } else {
                        query
                    };

                    #(#bindings)*

                    ret
                }

                pub fn sql(&self) -> String {
                    let mut _i = 1;
                    self.sql_starts_with(&mut _i)
                }

                pub fn all_conditions(&self) -> Vec<by_types::Conditions> {
                    let mut conditions = self.conditions.clone();
                    conditions.extend(self.or.iter().flatten().cloned());

                    #(#conditions)*

                    conditions
                }

                pub fn query(
                    &self,
                ) -> sqlx::query::Query<
                    'static,
                sqlx::Postgres,
                <sqlx::Postgres as sqlx::Database>::Arguments<'static>,
                > {
                    let mut i = 1;
                    let mut query = self.sql_starts_with(&mut i);

                    let s: Box<String> = Box::new(query);
                    let query: &'static str = Box::leak(s);

                    let mut q = sqlx::query(query);

                    for condition in self.all_conditions() {
                        q = match condition {
                            by_types::Conditions::BetweenBigint(_, from, to) => {
                                tracing::debug!("Binding BetweenBigint {} and {}", from, to);
                                q.bind(from).bind(to)
                            },
                            by_types::Conditions::EqualsBigint(_, value) => {
                                tracing::debug!("Binding EqualsBigint {}", value);
                                q.bind(value)
                            },
                            by_types::Conditions::NotEqualsBigint(_, value) => {
                                tracing::debug!("Binding NotEqualsBigint {}", value);
                                q.bind(value)
                            },
                            by_types::Conditions::GreaterThanBigint(_, value) => {
                                tracing::debug!("Binding GreaterThanBigint {}", value);
                                q.bind(value)
                            },
                            by_types::Conditions::LessThanBigint(_, value) => {
                                tracing::debug!("Binding LessThanBigint {}", value);
                                q.bind(value)
                            },
                            by_types::Conditions::GreaterThanEqualsBigint(_, value) => {
                                tracing::debug!("Binding GreaterThanEqualsBigint {}", value);
                                q.bind(value)
                            },
                            by_types::Conditions::LessThanEqualsBigint(_, value) => {
                                tracing::debug!("Binding LessThanEqualsBigint {}", value);
                                q.bind(value)
                            },
                            by_types::Conditions::BetweenInteger(_, from, to) =>  {
                                tracing::debug!("Binding BetweenInteger {} and {}", from, to);
                                q.bind(from).bind(to)
                            },
                            by_types::Conditions::EqualsInteger(_, value) => {
                                tracing::debug!("Binding EqualsInteger {}", value);
                                q.bind(value)
                            },
                            by_types::Conditions::NotEqualsInteger(_, value) => {
                                tracing::debug!("Binding NotEqualsInteger {}", value);
                                q.bind(value)
                            },
                            by_types::Conditions::GreaterThanInteger(_, value) => {
                                tracing::debug!("Binding GreaterThanInteger {}", value);
                                q.bind(value)
                            },
                            by_types::Conditions::LessThanInteger(_, value) => {
                                tracing::debug!("Binding LessThanInteger {}", value);
                                q.bind(value)
                            },
                            by_types::Conditions::GreaterThanEqualsInteger(_, value) => {
                                tracing::debug!("Binding GreaterThanEqualsInteger {}", value);
                                q.bind(value)
                            },
                            by_types::Conditions::LessThanEqualsInteger(_, value) => {
                                tracing::debug!("Binding LessThanEqualsInteger {}", value);
                                q.bind(value)
                            },
                            by_types::Conditions::EqualsText(_, value) => {
                                tracing::debug!("Binding EqualsText {}", value);
                                q.bind(value)
                            },
                            by_types::Conditions::NotEqualsText(_, value) => {
                                tracing::debug!("Binding NotEqualsText {}", value);
                                q.bind(value)
                            },
                            by_types::Conditions::ContainsText(_, value) => {
                                let value = format!("%{}%", value);
                                tracing::debug!("Binding ContainsText {}", value);
                                q.bind(value)
                            },
                            by_types::Conditions::NotContainsText(_, value) => {
                                let value = format!("%{}%", value);
                                tracing::debug!("Binding NotContainsText {}", value);
                                q.bind(value)
                            }
                            by_types::Conditions::StartsWithText(_, value) => {
                                let value = format!("{}%", value);
                                tracing::debug!("Binding StartsWithText {}", value);
                                q.bind(value)
                            }
                            by_types::Conditions::NotStartsWithText(_, value) => {
                                let value = format!("{}%", value);
                                tracing::debug!("Binding NotStartsWithText {}", value);
                                q.bind(value)
                            }
                            by_types::Conditions::EndsWithText(_, value) => {
                                let value = format!("%{}", value);
                                tracing::debug!("Binding EndsWithText {}", value);
                                q.bind(value)
                            }
                            by_types::Conditions::NotEndsWithText(_, value) => {
                                let value = format!("%{}", value);
                                tracing::debug!("Binding NotEndsWithText {}", value);
                                q.bind(value)
                            }
                            by_types::Conditions::TrueBoolean(_) => {
                                tracing::debug!("(Not)Binding TrueBoolean");
                                q
                            }
                            by_types::Conditions::FalseBoolean(_) => {
                                tracing::debug!("(Not)Binding FalseBoolean");
                                q
                            }
                            by_types::Conditions::Custom(_) => {
                                tracing::debug!("(Not)Binding FalseBoolean");
                                q
                            }
                        };
                    }
                    q
                }

                #(#functions)*
            }
        }
    }

    pub fn repo_update_request(&self) -> proc_macro2::TokenStream {
        let name = syn::Ident::new(
            &format!("{}RepositoryUpdateRequest", self.name),
            proc_macro2::Span::call_site(),
        );

        let mut fields = vec![];
        let mut functions = vec![];

        for (_, v) in self.fields.iter() {
            if v.should_skip_inserting() {
                continue;
            }
            let name = syn::Ident::new(&v.name, proc_macro2::Span::call_site());
            let ty = v.unwrapped_type_token();

            fields.push(quote! {
                pub #name: Option<#ty>
            });

            let fname =
                syn::Ident::new(&format!("with_{}", v.name), proc_macro2::Span::call_site());

            functions.push(quote! {
                pub fn #fname(mut self, #name: #ty) -> Self {
                    self.#name = Some(#name);
                    self
                }
            });
        }

        quote! {
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
            #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
            pub struct #name {
                #(#fields),*
            }

            impl #name {
                pub fn new() -> Self {
                    Self::default()
                }

                #(#functions)*
            }
        }
    }

    pub fn repository_update_request_st_name(&self) -> syn::Ident {
        syn::Ident::new(
            &format!("{}RepositoryUpdateRequest", self.name),
            proc_macro2::Span::call_site(),
        )
    }

    // TODO: impelment update function like find function with optional params
    // default all insert fields can be updated
    pub fn update_function(&self) -> proc_macro2::TokenStream {
        let update_req_st_name = self.repository_update_request_st_name();

        let st_var_name = syn::Ident::new(
            &format!("{}RepositoryUpdateRequest", self.name).to_case(Case::Snake),
            proc_macro2::Span::call_site(),
        );

        let mut returning = vec![];
        let mut option_condition = vec![];
        let mut option_binds = vec![];

        for (field_name, field) in self.fields.iter() {
            tracing::trace!("Field processing(update): {}", field_name);
            if !field.should_return_in_insert() {
                continue;
            }
            returning.push(field_name.clone());

            if field.should_skip_inserting() {
                continue;
            }

            let n = field.field_name_token();
            let ty = field.unwrapped_type_token();

            let field_name = syn::LitStr::new(field_name, proc_macro2::Span::call_site());
            option_condition.push(quote! {
                if #st_var_name.#n.is_some() {
                    i += 1;
                    update_values.push(format!("{} = ${}", #field_name, i));
                }
            });
            let bind = field.bind();
            option_binds.push(quote! {
                if let Some(#n) = #st_var_name.#n {
                    q = q #bind;
                }
            });
            continue;
        }

        if option_condition.is_empty() {
            return quote! {};
        }

        let name = syn::Ident::new(&self.name, proc_macro2::Span::call_site());
        let call_map = self.call_map();

        let q = syn::LitStr::new(
            &format!(
                "UPDATE {} SET {{}} WHERE id = $1 RETURNING {}",
                self.table_name,
                returning.join(", "),
            ),
            proc_macro2::Span::call_site(),
        );

        let rt = &self.result_type;

        let inner = quote! {
            let mut i = 1;
            let mut update_values = vec![];

            #(#option_condition)*

            let query = format!(
                #q,
                update_values.join(", "),
            );
            tracing::trace!("insert query: {}", query);
            let mut q = sqlx::query(&query)
                .bind(id);
            #(#option_binds)*
            let row = q
                #call_map
        };

        let output = quote! {
            pub async fn update(&self, id: i64, #st_var_name: #update_req_st_name) -> #rt<#name> {
                #inner
                .fetch_one(&self.pool)
                    .await?;

                Ok(row)
            }


            pub async fn update_with_tx<'e, 'c: 'e, E>(&self, tx: E, id: i64, #st_var_name: #update_req_st_name) -> #rt<Option<#name>>
            where
                E: sqlx::Executor<'c, Database = sqlx::postgres::Postgres>,
            {
                #inner
                .fetch_optional(tx)
                        .await?;

                Ok(row)
            }
        };

        output
    }

    pub fn delete_function(&self) -> proc_macro2::TokenStream {
        let repo_name = self.repository_struct_name();
        let name = self.name_id;
        let query = syn::LitStr::new(
            &format!("DELETE FROM {} WHERE id = $1 RETURNING *", self.table_name),
            proc_macro2::Span::call_site(),
        );
        let rt = &self.result_type;

        quote! {
            pub async fn delete(&self, id: i64) -> #rt<#name> {
                let res = sqlx::query(#query)
                    .bind(id)
                    .map(#name::from)
                    .fetch_one(&self.pool)
                    .await?;

                Ok(res)
            }

            pub async fn delete_with_tx<'e, 'c: 'e, E>(&self, tx: E, id: i64) -> #rt<Option<#name>>
            where
                E: sqlx::Executor<'c, Database = sqlx::postgres::Postgres>,
            {
                let res = sqlx::query(#query)
                    .bind(id)
                    .map(#name::from)
                    .fetch_optional(tx)
                    .await?;

                Ok(res)
            }
        }
    }

    pub fn insert_function(&self) -> proc_macro2::TokenStream {
        let mut insert_fields = vec![];
        let mut insert_values = vec![];

        let mut i = 1;

        let mut returning = vec![];
        let mut args = vec![];
        let mut binds = vec![];
        let mut has_option_args = false;
        let mut option_condition = vec![];
        let mut option_binds = vec![];

        for (field_name, field) in self.fields.iter() {
            tracing::trace!("Field processing(insert): {}", field_name);
            if !field.should_return_in_insert() {
                continue;
            }
            returning.push(field_name.clone());

            let n = field.field_name_token();

            if field.should_skip_inserting() {
                continue;
            }

            args.push(field.arg_token());

            if field.is_option() {
                tracing::trace!("Field is option: {}", field_name);
                has_option_args = true;
                let field_name = syn::LitStr::new(field_name, proc_macro2::Span::call_site());
                option_condition.push(quote! {
                    if let Some(#n) = &#n {
                        i += 1;
                        insert_fields.push(#field_name);
                        insert_values.push(format!("${}", i));
                    }
                });

                let bind = field.bind();
                option_binds.push(quote! {
                    if let Some(#n) = &#n {
                        q = q #bind;
                    }
                });
                continue;
            }

            binds.push(field.bind());

            insert_fields.push(field_name.clone());
            insert_values.push(format!("${}", i));

            i += 1;
        }
        tracing::trace!("Insert fields: {:?}", insert_fields);
        let name = syn::Ident::new(&self.name, proc_macro2::Span::call_site());
        let call_map = self.call_map();

        let insert_with_dep = self.insert_function_for_many_to_many();
        tracing::trace!("Insert with dep: {}", insert_with_dep.to_string());
        let start = syn::LitInt::new((i - 1).to_string().as_str(), proc_macro2::Span::call_site());

        let (inner, without_inner) = if has_option_args {
            tracing::trace!("Has option args");
            let insert_fields = insert_fields
                .iter()
                .map(|f| syn::LitStr::new(f, proc_macro2::Span::call_site()));
            let insert_values = insert_values
                .iter()
                .map(|f| syn::LitStr::new(f, proc_macro2::Span::call_site()));
            let insert_fields2 = insert_fields.clone();
            let insert_values2 = insert_values.clone();

            let q = syn::LitStr::new(
                &format!(
                    "INSERT INTO {} ({{}}) VALUES ({{}}) RETURNING {}",
                    self.table_name,
                    returning.join(", "),
                ),
                proc_macro2::Span::call_site(),
            );

            let wq = syn::LitStr::new(
                &format!("INSERT INTO {} ({{}}) VALUES ({{}})", self.table_name,),
                proc_macro2::Span::call_site(),
            );

            (
                quote! {
                    let mut i = #start;
                    let mut insert_fields = vec![#(#insert_fields),*];
                    let mut insert_values = vec![#(#insert_values),*].iter().map(|f| f.to_string()).collect::<Vec<String>>();
                    #(#option_condition)*
                    let query = format!(
                        #q,
                        insert_fields.join(", "),
                        insert_values.join(", "),
                    );
                    tracing::trace!("insert query: {}", query);
                    let mut q = sqlx::query(&query)
                        #(#binds)*;
                    #(#option_binds)*
                    let row = q
                        #call_map
                },
                quote! {
                    let mut i = #start;
                    let mut insert_fields = vec![#(#insert_fields2),*];
                    let mut insert_values = vec![#(#insert_values2),*].iter().map(|f| f.to_string()).collect::<Vec<String>>();
                    #(#option_condition)*
                    let query = format!(
                        #wq,
                        insert_fields.join(", "),
                        insert_values.join(", "),
                    );
                    tracing::trace!("insert query: {}", query);
                    let mut q = sqlx::query(&query)
                        #(#binds)*;
                    #(#option_binds)*
                    q
                },
            )
        } else {
            let q = syn::LitStr::new(
                &format!(
                    "INSERT INTO {} ({}) VALUES ({}) RETURNING {}",
                    self.table_name,
                    insert_fields.join(", "),
                    insert_values.join(", "),
                    returning.join(", "),
                ),
                proc_macro2::Span::call_site(),
            );

            let wq = syn::LitStr::new(
                &format!(
                    "INSERT INTO {} ({}) VALUES ({})",
                    self.table_name,
                    insert_fields.join(", "),
                    insert_values.join(", "),
                ),
                proc_macro2::Span::call_site(),
            );

            (
                quote! {
                    tracing::trace!("insert query: {}", #q);
                    let row = sqlx::query(#q)
                        #(#binds)*
                    #call_map

                },
                quote! {
                    tracing::trace!("insert query: {}", #q);
                    sqlx::query(#wq)
                        #(#binds)*

                },
            )
        };

        let rt = &self.result_type;
        let output = quote! {
            pub async fn insert(&self, #(#args),*) -> #rt<#name> {
                #inner
                .fetch_one(&self.pool)
                        .await?;

                Ok(row)
            }

            pub async fn insert_with_tx<'e, 'c: 'e, E>(&self, tx: E, #(#args),*) -> #rt<Option<#name>>
            where
                E: sqlx::Executor<'c, Database = sqlx::postgres::Postgres>,
            {
                #inner
                .fetch_optional(tx)
                        .await?;

                Ok(row)
            }

            pub async fn insert_without_returning(&self, #(#args),*) -> #rt<()> {
                #without_inner
                .execute(&self.pool)
                        .await?;

                Ok(())
            }

            #insert_with_dep
        };
        tracing::trace!("insert function output: {}", output);

        output.into()
    }

    pub fn select_functions(&self) -> proc_macro2::TokenStream {
        let name = syn::Ident::new(&self.name, proc_macro2::Span::call_site());
        let read_action = self.read_action_struct_name();

        let q = syn::LitStr::new(
            &format!("SELECT * FROM {} LIMIT 1", self.table_name),
            proc_macro2::Span::call_site(),
        );
        let rt = &self.result_type;

        let output = quote! {
        //         pub async fn select_by_id(&self, id: i64) -> #rt<#name> {
        //             let row = sqlx::query_as!(#name, "SELECT * FROM {} WHERE id = $1", id)
        //                 .fetch_one(&self.pool)
        //                 .await?;

        //             Ok(row)
        //         }

                pub async fn find_one(&self, param: #read_action) -> #rt<#name> {
                    let rows = sqlx::query("SELECT * FROM $1 LIMIT 1 OFFSET $3", size, (page - 1) * size)
                        .fetch_all(&self.pool)
                        .await?;
                    let rows = sqlx::query_as!(#name, "SELECT * FROM {}", self.table_name)
                        .fetch_all(&self.pool)
                        .await?;

                    Ok(rows)
                }
            };

        output.into()
    }
}

impl<'a> ApiModel<'a> {
    pub fn new(input: &'a DeriveInput, attr: TokenStream) -> Self {
        #[cfg(feature = "server")]
        let mut api_fields = IndexMap::new();
        #[cfg(feature = "server")]
        let mut database = Some(Database::Postgres);
        let name = input.ident.to_string();

        let mut has_validator = false;
        tracing::trace!("Length of attributes: {}", input.attrs.len());
        for at in &input.attrs {
            tracing::trace!("Meta: {:?}", at);
            if let Meta::List(meta_list) = at.meta.clone() {
                tracing::trace!("Meta list: {}", meta_list.tokens.to_string());
                let validate: Vec<String> = meta_list
                    .tokens
                    .to_string()
                    .split(",")
                    .filter(|f| f.contains("Validate"))
                    .map(|f| f.to_string())
                    .collect();
                if validate.len() > 0 {
                    tracing::trace!("Has validator: true");
                    has_validator = true;
                    break;
                }
            }
        }

        let data = match &input.data {
            syn::Data::Struct(data_struct) => data_struct,
            _ => panic!("api_mode can only be applied to structs"),
        };
        let name_id = &input.ident;

        let mut base = String::new();
        let mut parent_ids = Vec::new();
        let mut iter_type = "by_types::QueryResponse".to_string();
        let mut custom_query_type = None;
        let mut result_type: proc_macro2::TokenStream = "crate::Result".parse().unwrap();
        let mut read_action_names = IndexMap::<String, ActionField>::new();
        let actions = attr
            .to_string()
            .as_str()
            .parse::<Actions>()
            .expect("Parsing failed for actions");

        for arg in attr.to_string().split(',') {
            let parts: Vec<&str> = arg.split('=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let value = parts[1].trim().trim_matches('"');
                match key {
                    "base" => {
                        base = value
                            .split('/')
                            .map(|v| {
                                if v.starts_with(':') {
                                    parent_ids.push(
                                        v.trim_start_matches(':').to_string().to_case(Case::Snake),
                                    );
                                    "{}"
                                } else {
                                    v
                                }
                            })
                            .collect::<Vec<&str>>()
                            .join("/");
                    }
                    "iter_type" => iter_type = value.to_string(),
                    "custom_query_type" => custom_query_type = Some(value.to_string()),
                    "result_type" => {
                        result_type = value.to_string().parse().expect("invalid result type.")
                    }
                    // "read_action" => {
                    //     let value = value
                    //         .trim_matches('[')
                    //         .trim_matches(']')
                    //         .split(",")
                    //         .collect::<Vec<&str>>();
                    //     for v in value {
                    //         tracing::trace!("Read action: {}", v);
                    //         let v = v.trim();
                    //         read_action_names.insert(v.to_string(), ActionField::Fields(vec![]));
                    //     }
                    // }
                    #[cfg(feature = "server")]
                    "database" => {
                        if value.contains("skip") {
                            database = None;
                        }
                    }
                    _ => {}
                }
            }
        }

        #[cfg(feature = "server")]
        let attrs = super::sql_model::parse_sql_model(attr);
        #[cfg(feature = "server")]
        let table_name = match attrs.get(&SqlModelKey::Table) {
            Some(SqlModel::Table(name)) => name.to_string(),
            _ => {
                database = None;
                name.to_case(Case::Snake)
            }
        };

        #[cfg(feature = "server")]
        let rename = match attrs.get(&SqlModelKey::Rename) {
            Some(SqlModel::Rename(rename)) => rename.clone(),
            _ => Case::Snake,
        };

        let mut summary_fields = Vec::new();
        let mut queryable_fields = Vec::new();
        let mut action_names = IndexMap::<String, ActionField>::new();
        let mut action_by_id_names = IndexMap::<String, ActionField>::new();
        let mut query_action_names = IndexMap::<String, ActionField>::new();
        let mut primary_key = (String::new(), String::new());

        for (k, v) in actions.read_actions.iter() {
            read_action_names
                .entry(k.to_string())
                .or_insert_with(|| ActionField::Fields(vec![]));
        }

        for (k, v) in actions.action_by_id.iter() {
            action_by_id_names
                .entry(k.to_string())
                .or_insert_with(|| ActionField::Fields(vec![]));
        }

        for (k, v) in actions.actions.iter() {
            action_names
                .entry(k.to_string())
                .or_insert_with(|| ActionField::Fields(vec![]));
        }

        #[cfg(feature = "server")]
        for f in data.fields.iter() {
            let field_name = f.clone().ident.unwrap().to_string().to_case(rename);
            let f = ApiField::new(f, table_name.to_string(), rename.clone());
            if f.primary_key {
                primary_key = (field_name.clone(), f.r#type.clone());
            }

            api_fields.insert(field_name, f);
        }

        if let syn::Fields::Named(named_fields) = &data.fields {
            for field in &named_fields.named {
                for attr in &field.attrs {
                    let mut actions = vec![];
                    let mut related = None::<String>;

                    if let Meta::List(meta_list) = attr.meta.clone() {
                        if meta_list.path.is_ident("validate") {
                            has_validator = true;
                        }
                    }

                    for t in parse_action_attr(attr) {
                        match t {
                            ActionType::Summary => {
                                summary_fields.push(field.clone());
                            }
                            ActionType::Queryable => {
                                queryable_fields.push(field.clone());
                            }
                            ActionType::Action(action_names) => {
                                actions.push(ActionType::Action(action_names));
                            }
                            ActionType::ActionById(action_names) => {
                                actions.push(ActionType::ActionById(action_names));
                            }
                            ActionType::Related(st) => {
                                related = Some(st);
                            }
                            ActionType::QueryActions(action_names) => {
                                actions.push(ActionType::QueryActions(action_names));
                            }
                            ActionType::ReadActions(action_names) => {
                                actions.push(ActionType::ReadActions(action_names));
                            }
                        }
                    }

                    for action in actions {
                        match (related.clone(), action) {
                            (Some(st), ActionType::Action(actions)) => {
                                for action_name in actions {
                                    action_names
                                        .entry(action_name)
                                        .or_insert_with(|| ActionField::Related(st.clone()));
                                }
                            }
                            (Some(st), ActionType::ActionById(actions)) => {
                                for action_name in actions {
                                    action_by_id_names
                                        .entry(action_name)
                                        .or_insert_with(|| ActionField::Related(st.clone()));
                                }
                            }
                            (None, ActionType::Action(actions)) => {
                                for action_name in actions {
                                    match action_names
                                        .entry(action_name)
                                        .or_insert_with(|| ActionField::Fields(vec![]))
                                    {
                                        ActionField::Fields(v) => {
                                            v.push(field.clone());
                                        }

                                        _ => {
                                            panic!("Action should have fields")
                                        }
                                    };
                                }
                            }
                            (None, ActionType::ActionById(actions)) => {
                                for action_name in actions {
                                    match action_by_id_names
                                        .entry(action_name)
                                        .or_insert_with(|| ActionField::Fields(vec![]))
                                    {
                                        ActionField::Fields(v) => {
                                            v.push(field.clone());
                                        }
                                        _ => {
                                            panic!("ActionById should have fields")
                                        }
                                    };
                                }
                            }
                            (_, ActionType::QueryActions(actions)) => {
                                for action_name in actions {
                                    match query_action_names
                                        .entry(action_name)
                                        .or_insert_with(|| ActionField::Fields(vec![]))
                                    {
                                        ActionField::Fields(v) => {
                                            v.push(field.clone());
                                        }
                                        _ => {
                                            panic!("ActionById should have fields")
                                        }
                                    };
                                }
                            }
                            (_, ActionType::ReadActions(actions)) => {
                                for action_name in actions {
                                    match read_action_names
                                        .entry(action_name)
                                        .or_insert_with(|| ActionField::Fields(vec![]))
                                    {
                                        ActionField::Fields(v) => {
                                            v.push(field.clone());
                                        }
                                        _ => {
                                            panic!("ActionById should have fields")
                                        }
                                    };
                                }
                            }

                            _ => {}
                        }
                    }
                }
            }
        }

        Self {
            #[cfg(feature = "server")]
            fields: api_fields,
            #[cfg(feature = "server")]
            table_name,
            #[cfg(feature = "server")]
            rename,
            #[cfg(feature = "server")]
            database,
            #[cfg(feature = "server")]
            primary_key,

            result_type,

            name,
            name_id,
            iter_type,
            custom_query_type,
            base,
            read_action_names,
            parent_ids,
            actions,

            summary_fields,
            queryable_fields,
            action_names,
            action_by_id_names,
            query_action_names,
            has_validator,
        }
    }
}

#[cfg(feature = "server")]
#[derive(Debug, Clone)]
pub enum Relation {
    ManyToMany {
        // Table name of the join table
        table_name: String,
        // Foreign table name
        foreign_table_name: String,
        // Primary key in the foreign table (default: id)
        foreign_key: String,
        // Type of the primary key in the foreign table (default: BIGINT)
        foreign_key_type: String,
        // Reference key of foreign table in the join table
        foreign_primary_key: String,
        // Reference key of the current table in the join table
        foreign_reference_key: String,
        reference_key: String,
        target_table: TargetTable,
    },
    ManyToOne {
        table_name: String,
        foreign_key: String,
        foreign_key_type: String,
    },
    OneToMany {
        #[allow(dead_code)]
        table_name: String,
        foreign_key: String,
        reference_key: String,
        filter_by: Vec<(String, String)>,
    },
}

#[cfg(feature = "server")]
#[derive(Debug, Clone)]
pub struct ApiField {
    pub name: String, // this is a native field name in rust
    pub primary_key: bool,
    pub query_builder: bool,
    pub relation: Option<Relation>,
    pub r#type: String,
    pub unique: bool,
    pub auto: Vec<AutoOperation>,
    pub nullable: bool,
    // omitted indicates if this field should be included in insert or not.
    pub omitted: bool,
    pub indexed: bool,
    pub rust_type: String,

    pub summary: bool,
    pub queryable: bool,
    pub action_names: Vec<String>,
    pub action_by_id_names: Vec<String>,
    pub query_action_names: Vec<String>,
    pub read_action_names: Vec<String>,
    pub related: Option<String>,
    pub version: Option<String>,

    pub aggregator: Option<Aggregator>,
    // skip indicates if this field should be skipped for sql model or not.
    pub skip: bool,

    // depends on struct derive
    pub rename: Case,
    pub table: String,
}

#[cfg(feature = "server")]
impl ApiField {
    pub fn rust_type_id(&self) -> syn::Ident {
        syn::Ident::new(&self.rust_type, proc_macro2::Span::call_site())
    }

    pub fn sql_field_name(&self) -> String {
        self.name.to_case(self.rename)
    }

    pub fn aggregate_expose_query(&self, bound_name: &str) -> Option<String> {
        match self.aggregator {
            Some(Aggregator::Exist) => match &self.relation {
                Some(Relation::ManyToMany {
                    ref table_name,
                    ref foreign_primary_key,
                    ref foreign_reference_key,
                    ref reference_key,
                    ..
                }) => Some(format!(
                    r#"
CASE
    WHEN EXISTS (
        SELECT 1 FROM {table_name} WHERE {foreign_reference_key} = p.{reference_key} AND {foreign_primary_key} = {{}}
    ) THEN true
    ELSE false
END AS {bound_name}
"#,
                )),

                Some(Relation::OneToMany {
                    ref table_name,
                    ref foreign_key,
                    ref reference_key,
                    ..
                }) => Some(format!(
                    r#"
CASE
    WHEN EXISTS (
        SELECT 1 FROM {table_name} WHERE {foreign_key} = p.{reference_key}
    ) THEN true
    ELSE false
END AS {bound_name}
"#,
                )),
                _ => panic!(
                    "exists aggregator supports only `one_to_many` and `many_to_many` relations"
                ),
            },
            Some(_) => Some(format!(
                "COALESCE(MAX({}.value), 0) AS {}",
                bound_name, bound_name
            )),

            None => match &self.relation {
                Some(Relation::ManyToMany {
                    table_name,
                    foreign_table_name,
                    foreign_primary_key,
                    foreign_reference_key,
                    reference_key,
                    target_table,
                    ..
                }) => {
                    let table = match target_table {
                        TargetTable::Foreign => "f",
                        TargetTable::Join => "j",
                    };
                    if self.rust_type.starts_with("Vec") {
                        Some(format!(
                            r#"
COALESCE(
  (SELECT jsonb_agg(to_jsonb(m))
     FROM (
-- {bound_name} start
       SELECT DISTINCT ON (f.id) {table}.*
         FROM {foreign_table_name} f
              JOIN {table_name} j ON f.id = j.{foreign_primary_key}
        WHERE j.{foreign_reference_key} = p.{reference_key}
-- {bound_name} end
     ) m
  ), '[]'
) AS {bound_name}"#,
                        ))
                    } else {
                        Some(format!(
                            r#"
            (SELECT to_jsonb({table})
               FROM {foreign_table_name} f
                    JOIN {table_name} j ON f.id = j.{foreign_primary_key}
              WHERE j.{foreign_reference_key} = p.{reference_key}
              LIMIT 1
            ) AS {bound_name}"#,
                        ))
                    }
                }

                Some(Relation::OneToMany {
                    table_name,
                    foreign_key,
                    reference_key,
                    filter_by,
                }) => {
                    let filter_conditions = if !filter_by.is_empty() {
                        filter_by
                            .iter()
                            .map(|(filter, _)| format!("AND f.{} = {{}}", filter))
                            .collect::<Vec<_>>()
                            .join(" ")
                    } else {
                        "".to_string()
                    };
                    if self.rust_type.starts_with("Vec") {
                        Some(format!(
                            r#"
COALESCE(
  (SELECT jsonb_agg(to_jsonb(m))
     FROM (
-- {bound_name} start
       SELECT DISTINCT ON (f.id) f.*
         FROM {table_name} f
        WHERE f.{foreign_key} = p.{reference_key} {filter_conditions}
-- {bound_name} end
     ) m
  ), '[]'
) AS {bound_name}"#,
                        ))
                    } else {
                        Some(format!(
                            r#"
            (SELECT to_jsonb(f)
               FROM {table_name} f
              WHERE f.{foreign_key} = p.{reference_key} {filter_conditions}
              LIMIT 1
            ) AS {bound_name}"#,
                        ))
                    }
                }
                _ => None,
            },
        }
    }

    pub fn aggregate_replace(&self, idx: usize) -> Option<proc_macro2::TokenStream> {
        match (&self.aggregator, &self.relation) {
            (
                Some(Aggregator::Exist),
                Some(Relation::ManyToMany {
                    ref foreign_primary_key,
                    ..
                }),
            ) => {
                let arg_name =
                    syn::Ident::new(&foreign_primary_key, proc_macro2::Span::call_site());
                let idx = syn::LitStr::new(&format!("${}", idx), proc_macro2::Span::call_site());

                Some(quote! { .replace(#idx, #arg_name.to_string().as_str()) })
            }
            _ => None,
        }
    }

    pub fn aggregate_bind(&self) -> Option<proc_macro2::TokenStream> {
        match (&self.aggregator, &self.relation) {
            (
                Some(Aggregator::Exist),
                Some(Relation::ManyToMany {
                    ref foreign_primary_key,
                    ..
                }),
            ) => {
                let arg_name =
                    syn::Ident::new(&foreign_primary_key, proc_macro2::Span::call_site());

                Some(quote! { .bind(#arg_name) })
            }
            _ => None,
        }
    }

    pub fn aggregate_arg(&self) -> Vec<(String, proc_macro2::TokenStream)> {
        let mut ret = vec![];
        match (&self.aggregator, &self.relation) {
            (
                Some(Aggregator::Exist),
                Some(Relation::ManyToMany {
                    ref foreign_primary_key,
                    ref foreign_key_type,
                    ..
                }),
            ) => {
                let arg_name =
                    syn::Ident::new(&foreign_primary_key, proc_macro2::Span::call_site());
                let arg_type = syn::Ident::new(
                    match foreign_key_type.as_str() {
                        "BIGINT" => "i64",
                        "INTEGER" => "i32",
                        "BOOLEAN" => "bool",
                        "TEXT" => "String",
                        _ => "i64",
                    },
                    proc_macro2::Span::call_site(),
                );
                let stream = quote! { #arg_name: #arg_type};
                ret.push((foreign_primary_key.clone(), stream))
            }
            (_, Some(Relation::OneToMany { filter_by, .. })) => {
                for filter in filter_by {
                    let arg_name = syn::Ident::new(&filter.0, proc_macro2::Span::call_site());
                    let arg_type = syn::Ident::new(
                        match filter.1.as_str() {
                            "BIGINT" => "i64",
                            "INTEGER" => "i32",
                            "BOOLEAN" => "bool",
                            "TEXT" => "String",
                            _ => "i64",
                        },
                        proc_macro2::Span::call_site(),
                    );
                    let stream = quote! { #arg_name: #arg_type};
                    ret.push((filter.0.clone(), stream));
                }
            }
            // (
            //     None,
            //     Some(Relation::ManyToMany {
            //         ref foreign_reference_key,
            //         ..
            //     }),
            // ) => {
            //     if !self.rust_type.starts_with("Vec") {
            //         return None;
            //     }

            //     let arg_name =
            //         syn::Ident::new(&foreign_reference_key, proc_macro2::Span::call_site());

            //     Some(quote! { #arg_name: i64})
            // }
            _ => {}
        }
        ret
    }

    pub fn group_by(&self) -> Option<String> {
        match self.relation {
            Some(Relation::ManyToMany { .. }) => return Some(format!("GROUP BY p.id")),
            Some(Relation::OneToMany { .. }) => return Some(format!("GROUP BY p.id")),
            _ => None,
        }
    }

    /// It will be bound {bound_name.value}.
    pub fn aggregate_query(&self, bound_name: &str) -> Option<String> {
        let (table_name, foreign_key, reference_key) = match self.relation {
            Some(Relation::OneToMany {
                ref table_name,
                ref foreign_key,
                ref reference_key,
                ..
            }) => (table_name, foreign_key, reference_key),
            Some(Relation::ManyToMany {
                ref table_name,
                ref foreign_table_name,
                ref foreign_primary_key,
                ref foreign_reference_key,
                ref reference_key,
                ..
            }) => (table_name, foreign_reference_key, reference_key),
            _ => return None,
        };

        let mut where_clause = "".to_string();

        let aggregate = match self.aggregator {
            Some(Aggregator::Count) => "COUNT(id)".to_string(),
            Some(Aggregator::Sum(ref field_name)) => format!("SUM({})", field_name),
            Some(Aggregator::Avg(ref field_name)) => format!("AVG({})", field_name),
            Some(Aggregator::Max(ref field_name)) => {
                format!("MAX({})", field_name)
            }
            Some(Aggregator::Min(ref field_name)) => {
                format!("MIN({})", field_name)
            }
            Some(Aggregator::Exist) => return None,
            None => return None,
        };

        // NOTE: currently only support for the first bound field. Usually, we can expect to bind the primary key of this table.
        let query = format!(
            r#"
LEFT JOIN (
    SELECT {foreign_key}, {aggregate} AS value
    FROM {table_name} {where_clause}
    GROUP BY {foreign_key}
) {bound_name} ON p.{reference_key} = {bound_name}.{foreign_key}
"#,
        );

        Some(query)
    }

    pub fn can_query(&self) -> bool {
        if self.skip {
            return false;
        }
        if self.aggregator.is_some() {
            return true;
        }

        match self.relation {
            Some(Relation::ManyToMany { .. }) => false,
            Some(Relation::OneToMany { .. }) => false,
            _ => true,
        }
    }

    pub fn should_return_in_insert(&self) -> bool {
        if self.skip {
            return false;
        }
        match self.relation {
            Some(Relation::ManyToMany { .. }) => false,
            Some(Relation::OneToMany { .. }) => false,
            _ => true,
        }
    }

    pub fn should_skip_inserting(&self) -> bool {
        self.omitted
            || self.skip
            || self.auto.len() > 0
            || match self.relation {
                Some(Relation::OneToMany { .. }) => true,
                _ => false,
            }
    }

    pub fn arg_token(&self) -> proc_macro2::TokenStream {
        let name = syn::Ident::new(&self.name, proc_macro2::Span::call_site());
        let rust_type: proc_macro2::TokenStream = self.rust_type.parse().unwrap();

        let output = quote! {
            #name: #rust_type
        };

        output.into()
    }

    pub fn unwrapped_type_token(&self) -> proc_macro2::TokenStream {
        tracing::trace!(
            "ApiField::unwrapped_type_token {} -> {}",
            self.rust_type,
            self.rust_type
                .replace(" ", "")
                .trim_start_matches("Option<")
                .trim_end_matches(">"),
        );

        if self.rust_type.starts_with("Option") {
            self.rust_type
                .replace(" ", "")
                .trim_start_matches("Option<")
                .trim_end_matches(">")
                .parse()
                .unwrap()
        } else {
            self.rust_type.parse().unwrap()
        }
    }

    pub fn field_name_token(&self) -> proc_macro2::TokenStream {
        let name = syn::Ident::new(&self.name, proc_macro2::Span::call_site());

        let output = quote! {
            #name
        };

        output.into()
    }

    pub fn trigger_query(&self) -> Vec<String> {
        let mut query = vec![];

        if self.auto.len() > 0 {
            let function_name = format!("set_{}", self.name);
            let field_name = self.name.to_case(self.rename);

            //             query.push(format!(
            //                 r#"DO $$
            // BEGIN
            //     IF NOT EXISTS (
            //         SELECT 1
            //         FROM pg_proc
            //         WHERE proname = '{}'
            //         AND pg_catalog.pg_function_is_visible(oid)
            //     ) THEN
            //         CREATE FUNCTION {}()
            //             RETURNS TRIGGER AS $$
            //             BEGIN
            //                 NEW.{} := EXTRACT(EPOCH FROM now()); -- seconds
            //                 RETURN NEW;
            //             END;
            //         $$ LANGUAGE plpgsql;
            //     END IF;
            // END $$"#,
            //                 function_name, function_name, field_name,
            //             ));

            let op = self
                .auto
                .iter()
                .map(|a| match a {
                    AutoOperation::Update => "UPDATE",
                    AutoOperation::Insert => "INSERT",
                })
                .collect::<Vec<&str>>()
                .join(" OR ");

            let trigger_name = format!("trigger_{}_on_{}", self.name, self.table);

            query.push(format!(
                r#"DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_trigger
        WHERE tgname = '{}'
        AND tgrelid = '{}'::regclass
    ) THEN
        CREATE TRIGGER {}
        BEFORE {} ON {}
        FOR EACH ROW
        EXECUTE FUNCTION {}();
    END IF;
END $$"#,
                trigger_name,
                self.table,
                trigger_name,
                op,
                self.table,
                // function name
                function_name,
            ));
        }

        query
    }

    pub fn create_functions(&self) -> Vec<String> {
        let mut query = vec![];
        query.push(
            r#"
        DO $$
        BEGIN
            IF NOT EXISTS (
                SELECT 1
                FROM pg_proc
                WHERE proname = 'set_updated_at'
                AND pg_catalog.pg_function_is_visible(oid)
            ) THEN
                CREATE FUNCTION set_updated_at()
                RETURNS TRIGGER AS $$
                BEGIN
                    NEW.updated_at := EXTRACT(EPOCH FROM now()); -- seconds
                    RETURN NEW;
                END;
                $$ LANGUAGE plpgsql;
            END IF;
        END $$;
    "#
            .to_string(),
        );

        query.push(
            r#"
        DO $$
        BEGIN
            IF NOT EXISTS (
                SELECT 1
                FROM pg_proc
                WHERE proname = 'set_created_at'
                AND pg_catalog.pg_function_is_visible(oid)
            ) THEN
                CREATE FUNCTION set_created_at()
                RETURNS TRIGGER AS $$
                BEGIN
                    NEW.created_at := EXTRACT(EPOCH FROM now()); -- seconds
                    RETURN NEW;
                END;
                $$ LANGUAGE plpgsql;
            END IF;
        END $$;
    "#
            .to_string(),
        );

        query
    }
    pub fn alter_query(&self) -> Vec<String> {
        if self.version.is_none() {
            return vec![];
        }

        let q = format!(
            r#"DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM information_schema.columns
        WHERE table_name = '{}'
          AND column_name = '{}'
          AND data_type = '{}'
    ) THEN
        ALTER TABLE {} ADD COLUMN {} {};
    END IF;
END $$;
"#,
            // SELECT
            self.table,
            self.name.to_case(self.rename),
            self.r#type.to_lowercase(),
            // ALTER
            self.table,
            self.name.to_case(self.rename),
            self.r#type
        );
        vec![q]
    }

    fn create_field_query_line(&self) -> Option<String> {
        if self.skip {
            return None;
        }
        let name = self.name.to_case(self.rename);

        let mut line = match &self.relation {
            Some(Relation::ManyToOne {
                table_name,
                foreign_key,
                foreign_key_type,
            }) => {
                return Some(format!(
                    "{} {} NOT NULL, FOREIGN KEY ({}) REFERENCES {}({}) ON DELETE CASCADE",
                    // Foreign field
                    name,
                    foreign_key_type,
                    // Foreign key
                    name,
                    table_name,
                    foreign_key.to_case(self.rename),
                ));
            }
            Some(Relation::OneToMany { .. }) => return None,
            Some(Relation::ManyToMany { .. }) => return None,
            _ => format!("{} {}", name, self.r#type),
        };

        if self.primary_key && self.r#type == "BIGINT" {
            line = format!("{} PRIMARY KEY GENERATED ALWAYS AS IDENTITY", line);
        };

        if self.nullable {
            line = format!("{} NULL", line);
        } else {
            line = format!("{} NOT NULL", line);
        }

        if self.unique {
            line = format!("{} UNIQUE", line);
        }

        tracing::trace!("field creation query for {}: {}", name, line);

        Some(line)
    }

    pub fn get_sql_field_type(&self) -> Option<String> {
        if self.omitted {
            return None;
        }

        Some(format!(
            "{} {} {}",
            self.name.to_case(self.rename),
            self.r#type,
            if self.nullable { "" } else { "NOT NULL" }
        ))
    }

    pub fn get_additional_query(
        &self,
        this_primary_key_name: &str,
        this_primary_key_type: &str,
    ) -> Vec<String> {
        let this_table_name = &self.table;
        let var_name = self.name.to_case(self.rename);
        let case = self.rename;

        tracing::trace!("additional query for {var_name}");
        let mut query = vec![];

        match &self.relation {
            Some(Relation::ManyToMany {
                table_name,
                foreign_primary_key,
                foreign_reference_key,
                foreign_table_name,
                ..
            }) => {
                if foreign_table_name.is_empty() {
                    panic!(
                        "[{} / {}] describe `foreign_table_name` or `table_name` in many_to_many field",
                        self.name, self.table,
                    );
                }
                tracing::trace!("additional query for many to many relation: {var_name}");
                let q = format!(
                    r#"
CREATE TABLE IF NOT EXISTS {} (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    {} BIGINT NOT NULL,
    {} BIGINT NOT NULL,
    created_at BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW()),

    CONSTRAINT fk_{}_{} FOREIGN KEY ({}) REFERENCES {}(id) ON DELETE CASCADE,
    CONSTRAINT fk_{}_{} FOREIGN KEY ({}) REFERENCES {}(id) ON DELETE CASCADE
);
"#,
                    table_name,
                    foreign_primary_key,
                    foreign_reference_key,
                    // constraint 1
                    this_table_name,
                    foreign_table_name,
                    foreign_reference_key,
                    this_table_name,
                    // constraint 1
                    foreign_table_name,
                    this_table_name,
                    foreign_primary_key,
                    foreign_table_name,
                );
                tracing::trace!("query: {}", q);
                query.push(q);

                if self.unique {
                    let mut keys = [
                        foreign_primary_key.to_string(),
                        foreign_reference_key.to_string(),
                    ];
                    keys.sort();
                    let idx_name = format!("idx_{}_{}", table_name, keys.join("_"));
                    query.push(format!(
                        "CREATE UNIQUE INDEX IF NOT EXISTS {} ON {} ({}, {});",
                        idx_name, table_name, keys[0], keys[1]
                    ));
                }
            }
            Some(Relation::ManyToOne { .. }) => {
                tracing::trace!("additional query for many to one relation: {var_name}");

                // NOTE: Usually foreign key is the primary key of the other table in many-to-one relation.
                let index_name = format!("idx_{}_{}", self.table, self.name);
                query.push(format!(
                    "CREATE INDEX IF NOT EXISTS {} ON {}({});",
                    index_name, self.table, var_name,
                ));
            }
            _ => {}
        }

        if self.indexed {
            let index_name = format!("idx_{}_{}", self.table, self.name);
            query.push(format!(
                "CREATE INDEX IF NOT EXISTS {} ON {}({});",
                index_name, self.table, var_name,
            ));
        }

        query.extend(self.trigger_query());
        query.extend(self.alter_query());

        query
    }
}
pub fn to_string(ty: &syn::Type) -> String {
    match &ty {
        syn::Type::Path(ref type_path) => type_path.path.segments.last().unwrap().ident.to_string(),
        syn::Type::Verbatim(token) => token.to_string(),
        _ => {
            panic!("api_model custom type only support Type::Path and Type::Verbatim");
        }
    }
}

#[cfg(feature = "server")]
impl ApiField {
    pub fn is_option(&self) -> bool {
        self.rust_type.starts_with("Option")
    }

    pub fn bind(&self) -> proc_macro2::TokenStream {
        let n = self.field_name_token();
        let sql_field_name = syn::LitStr::new(
            &self.name.to_case(self.rename),
            proc_macro2::Span::call_site(),
        );

        let rust_type = self.unwrapped_type_token().to_string();

        match (rust_type.as_str(), self.r#type.as_str()) {
            (rust_type, "TEXT") if rust_type != "String" => {
                quote! {
                    .bind(#n.to_string())
                }
            }
            ("String", "BIGINT") => {
                quote! {
                    .bind(#n.parse::<i64>().unwrap())
                }
            }
            ("String", "INTEGER") => {
                quote! {
                    .bind(#n.parse::<i32>().unwrap())
                }
            }
            ("u32", "INTEGER") => {
                quote! {
                    .bind(#n as i32)
                }
            }
            ("u64", "BIGINT") => {
                quote! {
                    .bind(#n as i64)
                }
            }
            (_, "JSONB") => {
                quote! {
                    .bind(serde_json::to_value(&#n).unwrap())
                }
            }
            _ => {
                quote! {
                    .bind(#n)
                }
            }
        }
    }

    pub fn call_map(&self) -> proc_macro2::TokenStream {
        let field_name = self.name.to_case(self.rename);
        let n = self.field_name_token();

        let sql_field_name = syn::LitStr::new(&field_name, proc_macro2::Span::call_site());

        match self.aggregator {
            Some(Aggregator::Max(_)) | Some(Aggregator::Min(_)) => {
                return quote! {
                    #n: row.try_get::<i64, _>(#sql_field_name).unwrap_or_default()
                };
            }
            Some(Aggregator::Sum(_)) | Some(Aggregator::Avg(_)) => {
                let rust_type = self.rust_type_id();

                return quote! {
                    #n: row.try_get::<bigdecimal::BigDecimal, _>(#sql_field_name).unwrap_or_default().to_string().parse::<#rust_type>().unwrap()
                };
            }
            _ => {}
        };

        tracing::trace!("callmap {}: {}", self.name, self.rust_type);

        if &self.rust_type == "String" && &self.r#type != "TEXT" {
            if &self.r#type == "BIGINT" {
                return quote! {
                    #n: row.try_get::<i64, _>(#sql_field_name).unwrap_or_default().to_string()
                };
            } else if &self.r#type == "INTEGER" {
                return quote! {
                    #n: row.try_get::<i32, _>(#sql_field_name).unwrap_or_default().to_string()
                };
            }
        } else if (&self.rust_type == "u64" || &self.rust_type == "u32") {
            let ty = syn::Ident::new(&self.rust_type, proc_macro2::Span::call_site());

            if &self.r#type == "BIGINT" {
                return quote! {
                    #n: row.try_get::<i64, _>(#sql_field_name).unwrap_or_default() as #ty
                };
            } else if &self.r#type == "INTEGER" {
                return quote! {
                    #n: row.try_get::<i32, _>(#sql_field_name).unwrap_or_default() as #ty
                };
            }
        }

        if self.rust_type.starts_with("Vec") || self.r#type == "JSONB" {
            tracing::trace!("vector callmap: {}: {}", self.name, self.rust_type);
            let field_name = syn::LitStr::new(&field_name, proc_macro2::Span::call_site());

            let default: proc_macro2::TokenStream = if self.rust_type.starts_with("Vec") {
                quote! { vec![] }
            } else {
                let ty = syn::Ident::new(&self.rust_type, proc_macro2::Span::call_site());
                quote! { #ty::default() }
            };
            return quote! {
                #n: match row.try_get::<serde_json::Value, _>(#field_name) {
                    Ok(v) => match serde_json::from_value(v) {
                        Ok(v) => v,
                        Err(e) => {
                            tracing::error!("error on {} field: {}", #field_name, e);
                            #default
                        }
                    },
                    e => {
                        tracing::debug!("empty vector for {}: {:?}", #field_name, e);
                        #default
                    }
                }
            };
        }
        quote! {
            #n: row.try_get(#sql_field_name).unwrap_or_default()
        }
    }

    fn new(field: &Field, table: String, rename: Case) -> Self {
        let name = field.clone().ident.unwrap().to_string();
        let rust_type = field.ty.to_token_stream().to_string();

        tracing::trace!("new for {}:{}", name, rust_type);

        let mut summary = false;
        let mut queryable = false;
        let mut action_names = Vec::new();
        let mut action_by_id_names = Vec::new();
        let mut query_action_names = Vec::new();
        let mut read_action_names = Vec::new();
        let mut related = None;

        for attr in &field.attrs {
            for t in crate::api_model::parse_action_attr(attr) {
                match t {
                    ActionType::Summary => {
                        summary = true;
                    }
                    ActionType::Queryable => {
                        queryable = true;
                    }
                    ActionType::Action(an) => {
                        action_names = an;
                    }
                    ActionType::ActionById(action_names) => {
                        action_by_id_names = action_names;
                    }
                    ActionType::Related(st) => {
                        related = Some(st);
                    }
                    ActionType::QueryActions(action_names) => {
                        query_action_names = action_names;
                    }
                    ActionType::ReadActions(action_names) => {
                        read_action_names = action_names;
                    }
                }
            }
        }

        let f = super::sql_model::parse_field_attr(field);
        let skip = f.attrs.contains_key(&SqlAttributeKey::Skip);
        let indexed = f.attrs.contains_key(&SqlAttributeKey::Indexed);

        let query_builder = f.attrs.contains_key(&SqlAttributeKey::Nested);
        let primary_key = f.attrs.contains_key(&SqlAttributeKey::PrimaryKey);
        if primary_key {
            if rust_type.as_str() != "i64" {
                panic!("primary key must be i64 type");
            }
        }

        let version = match f.attrs.get(&SqlAttributeKey::Version) {
            Some(SqlAttribute::Version(v)) => Some(v.to_string()),
            _ => None,
        };

        let relation = match f.attrs.get(&SqlAttributeKey::Relation) {
            Some(SqlAttribute::ManyToMany {
                table_name,
                foreign_table_name,
                foreign_key,
                foreign_key_type,
                foreign_primary_key,
                foreign_reference_key,
                reference_key,
                target_table,
            }) => Some(Relation::ManyToMany {
                table_name: table_name.to_string(),
                foreign_table_name: foreign_table_name.to_string(),
                foreign_key: foreign_key.to_string(),
                foreign_key_type: foreign_key_type.to_string(),
                foreign_primary_key: foreign_primary_key.to_string(),
                foreign_reference_key: foreign_reference_key.to_string(),
                reference_key: reference_key.to_string(),
                target_table: *target_table,
            }),

            Some(SqlAttribute::ManyToOne {
                table_name,
                foreign_key,
                foreign_key_type,
            }) => Some(Relation::ManyToOne {
                table_name: table_name.to_string(),
                foreign_key: foreign_key.to_string(),
                foreign_key_type: foreign_key_type.to_string(),
            }),

            Some(SqlAttribute::OneToMany {
                table_name,
                foreign_key,
                reference_key,
                filter_by,
            }) => Some(Relation::OneToMany {
                table_name: table_name.to_string(),
                foreign_key: foreign_key.to_string(),
                reference_key: reference_key.to_string(),
                filter_by: filter_by.clone(),
            }),

            rel => {
                tracing::trace!("no relation for {:?}", rel);
                None
            }
        };

        tracing::trace!("relation: {:?}", relation);

        let ((mut r#type, mut nullable), mut failed_type_inference) = match to_type(&field.ty) {
            Some(t) => (t, false),
            None => (("TEXT".to_string(), false), true),
        };
        tracing::trace!(
            "inference type: {} {} for {}",
            r#type,
            if nullable { "NULL" } else { "NOT NULL" },
            name
        );

        match relation {
            Some(Relation::ManyToOne {
                ref foreign_key_type,
                ..
            }) => {
                tracing::trace!("many to one realtion: {}", foreign_key_type);
                failed_type_inference = false;
                r#type = foreign_key_type.to_string();
            }
            Some(Relation::ManyToMany {
                ref foreign_key_type,
                ..
            }) => {
                tracing::trace!("many to many realtion: {}", foreign_key_type);
                failed_type_inference = false;
            }
            _ => {}
        }

        match f.attrs.get(&SqlAttributeKey::SqlType) {
            Some(SqlAttribute::SqlType(t)) => {
                tracing::trace!("sql type: {}", t);
                failed_type_inference = false;
                r#type = t.to_string();
            }
            _ => {}
        };

        if f.attrs.contains_key(&SqlAttributeKey::Nullable) {
            tracing::trace!("nullable: true");
            nullable = true;
        };

        if primary_key {
            tracing::trace!("primary key: true");
            r#type = "BIGINT".to_string();
        }

        let auto: Vec<AutoOperation> = match f.attrs.get(&SqlAttributeKey::Auto) {
            Some(SqlAttribute::Auto(ops)) => ops.to_vec(),
            _ => vec![],
        };

        let omitted = failed_type_inference
            || match relation {
                Some(Relation::OneToMany { .. }) => true,
                Some(Relation::ManyToMany { .. }) => true,
                _ => false,
            }
            || primary_key
            || !auto.is_empty();

        tracing::trace!("omitted: {}", omitted);

        let unique = f.attrs.contains_key(&SqlAttributeKey::Unique);
        tracing::trace!("unique: {}", unique);

        tracing::trace!("ended new for {}:{}", name, rust_type);

        let aggregator = match f.attrs.get(&SqlAttributeKey::Aggregator) {
            Some(SqlAttribute::Aggregator(aggregator)) => Some(aggregator.clone()),
            _ => None,
        };

        tracing::trace!("aggregator: {:?}", aggregator);

        let ret = Self {
            name,
            primary_key,
            query_builder,
            relation,
            r#type,
            unique,
            auto,
            nullable,
            omitted,
            rust_type,
            summary,
            queryable,
            action_names,
            action_by_id_names,
            query_action_names,
            read_action_names,
            related,
            version,
            aggregator,
            skip,
            indexed,

            table,
            rename,
        };

        // ret.check_error();
        ret
    }

    //     fn check_error(&self) {
    //         let this_table = &self.table;
    //         if let Some(Relation::ManyToMany {
    //             ref table_name,
    //             ref foreign_table_name,
    //             ref foreign_primary_key,
    //             ref foreign_reference_key,
    //             ..
    //         }) = self.relation
    //         {
    //             if self.summary {
    //                 tracing::error!("Recommend that {} field should not have summary attribute. It may be empty when retrieved by other models", self.name);
    //             }

    //             let exists = Some(Aggregator::Exist) == self.aggregator;

    //             if !exists && !self.rust_type.replace(" ", "").ends_with("Summary>") {
    //                 tracing::error!("type: {}", self.rust_type);
    //                 tracing::error!(
    //                     r#"
    // Recommend to use summary model instead of a full model. if you need to contain full model, you must add `#[serde(default)]` to {foreign_primary_key} if the foreign model have the field.

    // Note that {foreign_reference_key} field will be empty when retrieved by other many_to_many model.

    // For example,

    // ❌ Bad Example
    // #[api_model(table = {foreign_table_name})]
    // pub struct ForeignModel {{
    //     #[api_model(summary, primary_key)]
    //     pub id: i64,
    //     #[api_model(summary, auto = insert)]
    //     pub created_at: i64,
    //     #[api_model(auto = [insert,update])]
    //     pub updated_at: i64,

    //     #[api_model(summary, many_to_many = {table_name}, foreign_table_name = {this_table}, foreign_primary_key = {foreign_reference_key}, foreign_reference_key = {foreign_primary_key})]
    //     #[serde(default)]
    //     pub {foreign_reference_key}: i64,
    // }}

    // ✅ Good Example
    // #[api_model(table = {foreign_table_name})]
    // pub struct ForeignModel {{
    //     #[api_model(summary, primary_key)]
    //     pub id: i64,
    //     #[api_model(summary, auto = insert)]
    //     pub created_at: i64,
    //     #[api_model(auto = [insert,update])]
    //     pub updated_at: i64,

    //     #[api_model(many_to_many = {table_name}, foreign_table_name = {this_table}, foreign_primary_key = {foreign_reference_key}, foreign_reference_key = {foreign_primary_key})]
    //     #[serde(default)]
    //     pub foreign_models: Vec<ForeignModel>,
    // }}

    // "#
    //                 );
    //             }

    //             if &self.rust_type == "i64" && self.aggregator.is_none() {
    //                 tracing::warn!(
    //                     "Recommended to use a type of a foreign model directly, instead of id types"
    //                 )
    //             }
    //         } else if let Some(Relation::OneToMany { .. }) = self.relation {
    //             if self.aggregator.is_none() {
    //                 if self.summary {
    //                     tracing::warn!("Recommend that {} field should not have summary attribute. It may be empty when retrieved by other models ", self.name);
    //                 }

    //                 if &self.rust_type == "i64" {
    //                     tracing::warn!(
    //                     "Recommended to use a type of a foreign model directly, instead of id types"
    //                 )
    //                 }
    //             }
    //         }
    //     }
}

#[cfg(feature = "server")]
fn to_type(var_type: &syn::Type) -> Option<(String, bool)> {
    let mut nullable = false;

    let name = match var_type {
        syn::Type::Path(ref type_path) => {
            let type_ident = type_path.path.segments.last().unwrap().ident.to_string();
            tracing::trace!("field type: {:?}", type_ident.as_str());
            match type_ident.as_str() {
                "u64" | "i64" => "BIGINT".to_string(),
                "String" => "TEXT".to_string(),
                "bool" => "BOOLEAN".to_string(),
                "i32" => "INTEGER".to_string(),
                "f64" => "DOUBLE PRECISION".to_string(),

                "Option" => {
                    nullable = true;
                    tracing::trace!("option field type: {:?}", type_path.path);
                    if let PathArguments::AngleBracketed(ref args) =
                        type_path.path.segments.last().unwrap().arguments
                    {
                        if let Some(syn::GenericArgument::Type(ref ty)) = args.args.first() {
                            if let Some((t, _)) = to_type(ty) {
                                t
                            } else {
                                return None;
                            }
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }

                _ => return None,
            }
        }
        _ => return None,
    };

    Some((name, nullable))
}
