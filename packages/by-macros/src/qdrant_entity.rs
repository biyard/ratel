use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, LitStr};

/// Proc macro derive for `QdrantEntity`.
///
/// Generates:
/// - `fn collection_name() -> String` — from `#[qdrant(collection_name = "...")]`
/// - `fn point_id(&self) -> String` — from `#[qdrant(id)]` field
/// - `fn payload(&self) -> HashMap<String, qdrant_client::qdrant::Value>` — all serializable fields
/// - `async fn upsert_points(&self, client: &qdrant_client::Qdrant) -> Result<(), qdrant_client::QdrantError>`
/// - `async fn delete_points(client: &qdrant_client::Qdrant, point_id: &str) -> Result<(), qdrant_client::QdrantError>`
///
/// The struct must also implement `Embedding` trait for the `embed()` method.
pub fn qdrant_entity_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Parse container-level #[qdrant(collection_name = "...")]
    let mut collection_name: Option<String> = None;
    for attr in &input.attrs {
        if attr.path().is_ident("qdrant") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("collection_name") {
                    let value: LitStr = meta.value()?.parse()?;
                    collection_name = Some(value.value());
                }
                Ok(())
            });
        }
    }

    let collection_name = collection_name.unwrap_or_else(|| "main".to_string());

    // Parse fields
    let Data::Struct(data_struct) = &input.data else {
        return syn::Error::new_spanned(&input.ident, "QdrantEntity can only be derived for structs")
            .to_compile_error()
            .into();
    };

    let Fields::Named(fields) = &data_struct.fields else {
        return syn::Error::new_spanned(
            &input.ident,
            "QdrantEntity requires named fields",
        )
        .to_compile_error()
        .into();
    };

    let mut id_field: Option<syn::Ident> = None;
    let mut payload_inserts = vec![];

    for field in &fields.named {
        let field_ident = field.ident.as_ref().unwrap();
        let field_name_str = field_ident.to_string();

        // Check for #[qdrant(id)]
        let mut is_id = false;
        for attr in &field.attrs {
            if attr.path().is_ident("qdrant") {
                let _ = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("id") {
                        is_id = true;
                    }
                    Ok(())
                });
            }
        }

        if is_id {
            id_field = Some(field_ident.clone());
        }

        // All fields become payload entries via serde_json
        payload_inserts.push(quote! {
            if let Ok(val) = serde_json::to_value(&self.#field_ident) {
                if !val.is_null() {
                    map.insert(
                        #field_name_str.to_string(),
                        crate::features::rag::qdrant::types::json_to_qdrant_value(val),
                    );
                }
            }
        });
    }

    let id_field = match id_field {
        Some(f) => f,
        None => {
            return syn::Error::new_spanned(
                &input.ident,
                "QdrantEntity requires one field with #[qdrant(id)]",
            )
            .to_compile_error()
            .into();
        }
    };

    let collection_name_lit = collection_name;

    let expanded = quote! {
        impl #name {
            pub fn collection_name() -> String {
                let prefix = option_env!("DYNAMO_TABLE_PREFIX").unwrap_or("ratel-local");
                format!("{}-{}", prefix, #collection_name_lit)
            }

            pub fn point_id(&self) -> String {
                self.#id_field.to_string()
            }

            pub fn payload(&self) -> std::collections::HashMap<String, qdrant_client::qdrant::Value> {
                let mut map = std::collections::HashMap::new();
                #(#payload_inserts)*
                map
            }

            pub async fn upsert_points(
                &self,
                client: &qdrant_client::Qdrant,
            ) -> std::result::Result<(), qdrant_client::QdrantError>
            where
                Self: crate::features::rag::qdrant::types::Embedding,
            {
                use crate::features::rag::qdrant::types::Embedding;

                let collection = Self::collection_name();

                // Ensure collection exists
                if !client.collection_exists(&collection).await? {
                    client
                        .create_collection(
                            qdrant_client::qdrant::CreateCollectionBuilder::new(&collection)
                                .vectors_config(
                                    qdrant_client::qdrant::VectorParamsBuilder::new(
                                        1024,
                                        qdrant_client::qdrant::Distance::Cosine,
                                    ),
                                ),
                        )
                        .await?;
                }

                let vector = self.embed().await.map_err(|e| {
                    qdrant_client::QdrantError::Io(
                        std::io::Error::new(std::io::ErrorKind::Other, format!("embedding failed: {e}"))
                    )
                })?;

                let point = qdrant_client::qdrant::PointStruct::new(
                    self.point_id(),
                    vector,
                    self.payload(),
                );

                client
                    .upsert_points(
                        qdrant_client::qdrant::UpsertPointsBuilder::new(
                            collection,
                            vec![point],
                        ),
                    )
                    .await?;

                Ok(())
            }

            pub async fn delete_points(
                client: &qdrant_client::Qdrant,
                point_id: &str,
            ) -> std::result::Result<(), qdrant_client::QdrantError> {
                let collection = Self::collection_name();

                client
                    .delete_points(
                        qdrant_client::qdrant::DeletePointsBuilder::new(&collection)
                            .points(qdrant_client::qdrant::PointsIdsList {
                                ids: vec![point_id.to_string().into()],
                            }),
                    )
                    .await?;

                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}
