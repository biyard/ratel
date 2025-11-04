use crate::{AppState, Error, models::user::User};
use aide::NoApi;
use axum::extract::{Json, State};
use bdk::prelude::*;

use crate::features::did::{DidDocument, DidIdentifier, DidMethod, StoredDidDocument};

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct CreateDidRequest {
    /// The DID to create (e.g., "did:web:example.com")
    pub did: String,

    /// The DID document content
    pub document: DidDocument,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema)]
pub struct CreateDidResponse {
    /// The created DID
    pub did: String,

    /// The stored DID document
    pub document: StoredDidDocument,
}

pub async fn create_did_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Json(req): Json<CreateDidRequest>,
) -> Result<Json<CreateDidResponse>, Error> {
    let cli = &dynamo.client;

    // Parse and validate the DID
    let did_identifier = DidIdentifier::parse(&req.did)
        .map_err(|e| Error::BadRequest(format!("Invalid DID: {}", e)))?;

    // Validate the DID document
    req.document
        .validate()
        .map_err(|e| Error::BadRequest(format!("Invalid DID document: {}", e)))?;

    // Check if DID document ID matches the requested DID
    if req.document.id != req.did {
        return Err(Error::BadRequest(
            "DID document ID must match the requested DID".to_string(),
        ));
    }

    // Check if DID already exists
    if let Some(_existing) = StoredDidDocument::get_by_did(cli, &req.did).await? {
        return Err(Error::BadRequest("DID already exists".to_string()));
    }

    // Create the stored DID document
    let stored_doc = StoredDidDocument::new(
        req.did.clone(),
        did_identifier.method,
        req.document,
        user.pk.clone(),
    );

    // Save to database
    stored_doc.create(cli).await?;

    Ok(Json(CreateDidResponse {
        did: req.did,
        document: stored_doc,
    }))
}
