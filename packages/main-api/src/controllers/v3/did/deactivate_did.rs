use crate::{AppState, Error, models::user::User};
use aide::NoApi;
use axum::Json;
use axum::extract::{Path, State};
use bdk::prelude::*;

use crate::features::did::StoredDidDocument;

use super::resolve_did::{DidPath, DidPathParam};

#[derive(Debug, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema)]
pub struct DeactivateDidResponse {
    /// The deactivated DID
    pub did: String,

    /// Success message
    pub message: String,

    /// Deactivation timestamp
    pub deactivated_at: i64,
}

pub async fn deactivate_did_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(DidPathParam { did }): DidPath,
) -> Result<Json<DeactivateDidResponse>, Error> {
    let cli = &dynamo.client;

    // Decode the DID from URL encoding
    let did = percent_encoding::percent_decode_str(&did)
        .decode_utf8()
        .map_err(|e| Error::BadRequest(format!("Invalid URL encoding: {}", e)))?
        .into_owned();

    // Get the stored DID document
    let mut stored_doc = StoredDidDocument::get_by_did(cli, &did)
        .await?
        .ok_or_else(|| Error::NotFound(format!("DID not found: {}", did)))?;

    // Check if user owns this DID
    if !stored_doc.is_owned_by(&user.pk) {
        return Err(Error::Unauthorized(
            "You do not have permission to deactivate this DID".to_string(),
        ));
    }

    // Check if already deactivated
    if !stored_doc.is_active {
        return Err(Error::BadRequest("DID is already deactivated".to_string()));
    }

    // Deactivate the DID
    stored_doc.deactivate(cli).await?;

    Ok(Json(DeactivateDidResponse {
        did: stored_doc.did,
        message: "DID successfully deactivated".to_string(),
        deactivated_at: stored_doc.updated_at,
    }))
}
