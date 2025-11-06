use crate::features::did::*;

use super::*;

pub async fn get_or_create_did_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
) -> Result<Json<Document>> {
    let document = StoredDidDocument::get(
        &dynamo.client,
        CompositePartition(user.pk.clone(), Partition::Did),
        None::<String>,
    )
    .await?;

    let document = if let Some(stored) = document {
        stored.document.ok_or_else(|| {
            Error::InternalServerError("DID document not found in storage".to_string())
        })?
    } else {
        let stored = StoredDidDocument::new(user.pk.clone(), user.username)?;
        stored.create(&dynamo.client).await?;

        stored.document.ok_or_else(|| {
            Error::InternalServerError("Failed to create DID document".to_string())
        })?
    };

    Ok(Json(document))
}
