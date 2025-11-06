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

    let document = if let Some(document) = document {
        document.document
    } else {
        let document = StoredDidDocument::new(user.pk.clone(), user.username)?;
        document.create(&dynamo.client).await?;

        document.document
    };

    Ok(Json(document))
}
