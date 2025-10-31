use crate::{AppState, Error};
use aide::NoApi;
use axum::Json;
use axum::extract::{Path, State};
use bdk::prelude::*;

use crate::features::did::{DidDocument, StoredDidDocument};
use crate::models::user::User;

#[derive(Debug, serde::Deserialize, aide::OperationIo, JsonSchema)]
pub struct DidPathParam {
    /// The DID to resolve (URL-encoded)
    pub did: String,
}

pub type DidPath = Path<DidPathParam>;

#[derive(Debug, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema)]
pub struct ResolveDidResponse {
    /// The DID
    pub did: String,

    /// The DID document
    pub document: DidDocument,

    /// Whether the DID is active
    pub is_active: bool,

    /// Creation timestamp
    pub created_at: i64,

    /// Last update timestamp
    pub updated_at: i64,
}

pub async fn resolve_did_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
    Path(DidPathParam { did }): DidPath,
) -> Result<Json<ResolveDidResponse>, Error> {
    let cli = &dynamo.client;

    // Decode the DID from URL encoding
    let did = percent_encoding::percent_decode_str(&did)
        .decode_utf8()
        .map_err(|e| Error::BadRequest(format!("Invalid URL encoding: {}", e)))?
        .into_owned();

    // Get the stored DID document
    let stored_doc = StoredDidDocument::get_by_did(cli, &did)
        .await?
        .ok_or_else(|| Error::NotFound(format!("DID not found: {}", did)))?;

    // Check if DID is active
    if !stored_doc.is_active {
        return Err(Error::BadRequest("DID has been deactivated".to_string()));
    }

    Ok(Json(ResolveDidResponse {
        did: stored_doc.did,
        document: stored_doc.document,
        is_active: stored_doc.is_active,
        created_at: stored_doc.created_at,
        updated_at: stored_doc.updated_at,
    }))
}
