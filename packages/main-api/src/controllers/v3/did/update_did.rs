use crate::{AppState, Error, models::user::User};
use aide::NoApi;
use axum::extract::{Json, Path, State};
use bdk::prelude::*;

use crate::features::did::{DidDocument, StoredDidDocument};

use super::resolve_did::{DidPath, DidPathParam};

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct UpdateDidRequest {
    /// The updated DID document content
    pub document: DidDocument,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema)]
pub struct UpdateDidResponse {
    /// The DID
    pub did: String,

    /// The updated DID document
    pub document: DidDocument,

    /// Last update timestamp
    pub updated_at: i64,
}

pub async fn update_did_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(DidPathParam { did }): DidPath,
    Json(req): Json<UpdateDidRequest>,
) -> Result<Json<UpdateDidResponse>, Error> {
    let cli = &dynamo.client;

    // Decode the DID from URL encoding
    let did = percent_encoding::percent_decode_str(&did)
        .decode_utf8()
        .map_err(|e| Error::BadRequest(format!("Invalid URL encoding: {}", e)))?
        .into_owned();

    // Validate the DID document
    req.document
        .validate()
        .map_err(|e| Error::BadRequest(format!("Invalid DID document: {}", e)))?;

    // Check if DID document ID matches
    if req.document.id != did {
        return Err(Error::BadRequest(
            "DID document ID must match the DID being updated".to_string(),
        ));
    }

    // Get the stored DID document
    let mut stored_doc = StoredDidDocument::get_by_did(cli, &did)
        .await?
        .ok_or_else(|| Error::NotFound(format!("DID not found: {}", did)))?;

    // Check if user owns this DID
    if !stored_doc.is_owned_by(&user.pk) {
        return Err(Error::Unauthorized(
            "You do not have permission to update this DID".to_string(),
        ));
    }

    // Check if DID is active
    if !stored_doc.is_active {
        return Err(Error::BadRequest(
            "Cannot update a deactivated DID".to_string(),
        ));
    }

    // Update the document
    stored_doc
        .update_document(cli, req.document.clone())
        .await?;

    Ok(Json(UpdateDidResponse {
        did: stored_doc.did,
        document: req.document,
        updated_at: stored_doc.updated_at,
    }))
}
