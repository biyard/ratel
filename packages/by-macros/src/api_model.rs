use convert_case::Casing;
use proc_macro::TokenStream;
use quote::quote;
use syn::*;

use crate::api_model_struct::ApiModel;

#[derive(Debug)]
pub enum ActionType {
    Summary,
    Queryable,
    Action(Vec<String>),
    ActionById(Vec<String>),
    Related(String),
    QueryActions(Vec<String>),
    ReadActions(Vec<String>),
}

pub fn api_model_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    #[cfg(feature = "server")]
    let db_structs: proc_macro2::TokenStream =
        crate::sql_model::sql_model_impl(attr.clone(), item.clone()).into();
    #[cfg(not(feature = "server"))]
    let db_structs: proc_macro2::TokenStream = quote! {};

    tracing::trace!("generated db code: {}", db_structs.to_string());

    let input_cloned = item.clone();
    let input = parse_macro_input!(item as DeriveInput);

    let model = ApiModel::new(&input, attr.clone());
    tracing::trace!("Model: {:#?}", model);
    let param_struct = model.param_struct();
    tracing::trace!("Param struct: {}", param_struct.to_string());
    let get_response = model.get_response_struct();
    tracing::trace!("Get response struct: {}", get_response.to_string());

    let summary_struct = model.generate_summary_struct();
    tracing::trace!("Summary struct: {}", summary_struct.to_string());
    let query_struct = model.generate_query_struct();
    tracing::trace!("Query struct: {}", query_struct.to_string());
    let read_action_struct = model.generate_read_struct();
    tracing::trace!("Read action struct: {}", read_action_struct.to_string());
    let action_struct = model.generate_action_struct();
    tracing::trace!("Action struct: {}", action_struct.to_string());
    let action_by_id_struct = model.generate_action_by_id_struct();
    tracing::trace!("Action by id struct: {}", action_by_id_struct.to_string());

    let client_impl = model.generate_client_impl();
    tracing::trace!("Client impl: {}", client_impl.to_string());
    let input = parse_macro_input!(input_cloned as syn::ItemStruct);
    let stripped_input = strip_struct_attributes(&input);
    let n = &model.name;
    let struct_comment = format!(
        r#"
/// {n} is a generated struct that represents the model
///
/// For making API calls related to this model, use `{n}::get_client(endpoint: &str)`.
/// It will returns {n}Client struct that implements the API calls.
///
/// In server side, you can use `{n}::get_repository()` to interact with the database.
/// Recommend to use `{n}Repository` to insert or update the model.
/// To query the model, use `{n}::query_builder()`.
/// For more detail, refer to the documentation of the query builder.
"#,
    )
    .parse::<proc_macro2::TokenStream>()
    .unwrap();

    let output = quote! {
        #db_structs

        #struct_comment
        #[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default, PartialEq)]
        #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
        #stripped_input

        #action_struct
        #action_by_id_struct
        #summary_struct
        #query_struct
        #client_impl

        #read_action_struct

        #param_struct

        #get_response
    };

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

    let file_path = format!(
        "{}/{}.rs",
        dir_path,
        model.name.to_case(convert_case::Case::Snake)
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
        tracing::info!("generated code {} into {}", model.name, file_path);
    }

    output.into()
}

/// Parse the attribute string and return the action type
/// The attribute should be in the form of #[api_model(action::action_name, action_by_id::action_name)]
pub fn parse_action_attr(attr: &Attribute) -> Vec<ActionType> {
    let mut types = Vec::new();

    if let Meta::List(meta_list) = attr.meta.clone() {
        if meta_list.path.is_ident("api_model") {
            let mut selected_action = ActionType::Summary;
            let mut selected_at = 0;
            let mut i = 0;

            for nested in meta_list.tokens.clone() {
                if let proc_macro2::TokenTree::Ident(iden) = nested {
                    let id = iden.to_string();
                    match id.as_str() {
                        "summary" => {
                            types.push(ActionType::Summary);
                        }
                        "queryable" => {
                            types.push(ActionType::Queryable);
                        }
                        "action" => {
                            selected_at = i;
                            selected_action = ActionType::Action(vec![]);
                        }
                        "action_by_id" => {
                            selected_at = i;
                            selected_action = ActionType::ActionById(vec![]);
                        }
                        "related" => {
                            selected_at = i;
                            selected_action = ActionType::Related("".to_string());
                        }
                        "query_action" => {
                            selected_at = i;
                            selected_action = ActionType::QueryActions(vec![]);
                        }
                        "read_action" => {
                            selected_at = i;
                            selected_action = ActionType::ReadActions(vec![]);
                        }
                        _ => {
                            if selected_at == (i - 2) {
                                match &selected_action {
                                    ActionType::Action(_) => {
                                        types.push(ActionType::Action(vec![id.to_string()]));
                                    }
                                    ActionType::ActionById(_) => {
                                        types.push(ActionType::ActionById(vec![id.to_string()]));
                                    }
                                    ActionType::Related(_) => {
                                        types.push(ActionType::Related(id.to_string()));
                                    }
                                    ActionType::QueryActions(_) => {
                                        types.push(ActionType::QueryActions(vec![id.to_string()]));
                                    }
                                    ActionType::ReadActions(_) => {
                                        types.push(ActionType::ReadActions(vec![id.to_string()]));
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                } else if let proc_macro2::TokenTree::Group(group) = nested {
                    let mut actions = vec![];
                    for nested in group.stream() {
                        if let proc_macro2::TokenTree::Ident(iden) = nested {
                            let id = iden.to_string();
                            actions.push(id);
                        }
                    }

                    match &selected_action {
                        ActionType::Action(_) => {
                            types.push(ActionType::Action(actions));
                        }
                        ActionType::ActionById(_) => {
                            types.push(ActionType::ActionById(actions));
                        }
                        ActionType::QueryActions(_) => {
                            types.push(ActionType::QueryActions(actions));
                        }
                        ActionType::ReadActions(_) => {
                            types.push(ActionType::ReadActions(actions));
                        }
                        ActionType::Related(_) | ActionType::Summary | ActionType::Queryable => {}
                    }
                }

                i += 1;
            }
        }
    }

    types
}

#[derive(Debug)]
pub enum ActionField {
    Fields(Vec<Field>),
    Related(String),
}

fn strip_struct_attributes(input: &syn::ItemStruct) -> syn::ItemStruct {
    let mut stripped_struct = input.clone();
    stripped_struct.fields = strip_api_model_attributes(&input.fields);
    stripped_struct
}

fn strip_api_model_attributes(fields: &syn::Fields) -> syn::Fields {
    match fields {
        syn::Fields::Named(named_fields) => {
            let stripped_fields = named_fields
                .named
                .iter()
                .map(|field| {
                    let mut field = field.clone();
                    field.attrs = field
                        .attrs
                        .iter()
                        .filter(|attr| !attr.path().is_ident("api_model"))
                        .cloned()
                        .collect();
                    field
                })
                .collect();
            syn::Fields::Named(syn::FieldsNamed {
                brace_token: named_fields.brace_token,
                named: stripped_fields,
            })
        }
        syn::Fields::Unnamed(_) | syn::Fields::Unit => fields.clone(),
    }
}
