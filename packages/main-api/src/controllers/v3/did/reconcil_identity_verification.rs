use crate::features::did::{Service, ServiceEndpoint, ServiceType, StoredDidDocument};
use crate::services::portone::{IdentifyResponse, VerifiedGender};
use crate::{AppState, Error, models::user::User, utils};
use aide::{NoApi, OperationIo};
use axum::extract::{Json, State};
use bdk::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReconcilIdentityVerificationRequest {
    #[schemars(description = "Identity Verifycation ID")]
    pub id: String,

    #[schemars(description = "The DID to associate verified attributes with")]
    pub did: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReconcilIdentityVerificationResponse {
    #[schemars(description = "Status of the operation")]
    pub result: IdentifyResponse,

    #[schemars(description = "Updated DID document with verified attributes")]
    pub did_document: StoredDidDocument,
}

/// Helper to calculate age from birth date string (format: YYYY-MM-DD)
fn calculate_age(birth_date: &str) -> Result<u32, Error> {
    use chrono::Datelike;

    let parts: Vec<&str> = birth_date.split('-').collect();
    if parts.len() != 3 {
        return Err(Error::BadRequest(
            "Invalid birth date format. Expected YYYY-MM-DD".to_string(),
        ));
    }

    let birth_year: i32 = parts[0]
        .parse()
        .map_err(|_| Error::BadRequest("Invalid year in birth date".to_string()))?;
    let birth_month: u32 = parts[1]
        .parse()
        .map_err(|_| Error::BadRequest("Invalid month in birth date".to_string()))?;
    let birth_day: u32 = parts[2]
        .parse()
        .map_err(|_| Error::BadRequest("Invalid day in birth date".to_string()))?;

    // Get current date
    let now = chrono::Utc::now();
    let current_year = now.year();
    let current_month = now.month();
    let current_day = now.day();

    // Calculate age
    let mut age = (current_year - birth_year) as u32;

    // Adjust if birthday hasn't occurred this year
    if current_month < birth_month || (current_month == birth_month && current_day < birth_day) {
        age -= 1;
    }

    Ok(age)
}

pub async fn reconcil_identity_verification_handler(
    State(AppState {
        portone, dynamo, ..
    }): State<AppState>,
    NoApi(user): NoApi<User>,
    Json(req): Json<ReconcilIdentityVerificationRequest>,
) -> Result<Json<ReconcilIdentityVerificationResponse>, Error> {
    tracing::debug!("Handling KYC request: {:?}", req);
    let cli = &dynamo.client;

    // Get verification data from PortOne
    let result = portone.identify(&req.id).await?;

    // Verify the DID exists and user owns it
    let mut stored_did = StoredDidDocument::get_by_did(cli, &req.did)
        .await?
        .ok_or_else(|| Error::NotFound(format!("DID not found: {}", req.did)))?;

    if !stored_did.is_owned_by(&user.pk) {
        return Err(Error::Unauthorized(
            "You do not have permission to add verified attributes to this DID".to_string(),
        ));
    }

    let verified_customer = &result.verified_customer;

    let age = calculate_age(&verified_customer.birth_date)?;

    // Create verified attributes as structured data
    // Note: gender will be serialized using DynamoEnum's Display impl (UPPER_SNAKE case)
    let verified_attrs = serde_json::json!({
        "name": verified_customer.name,
        "birthDate": verified_customer.birth_date,
        "age": age,
        "gender": verified_customer.gender.to_string(),
        "phoneNumber": verified_customer.phone_number,
        "isForeigner": verified_customer.is_foreigner,
        "verifiedAt": utils::time::get_now_timestamp(),
        "verifiedBy": "portone",
        "verificationId": req.id
    });

    // Add or update the verified attributes service endpoint
    let service_id = format!("{}#verified-attributes", req.did);
    let service = Service {
        id: service_id.clone(),
        service_type: ServiceType::Single("VerifiedAttributes".to_string()),
        service_endpoint: ServiceEndpoint::Object(verified_attrs),
    };

    // Update or add the service to the DID document
    let mut document = stored_did.document.clone();
    match &mut document.service {
        Some(services) => {
            // Check if verified attributes service already exists
            if let Some(existing) = services.iter_mut().find(|s| s.id == service_id) {
                // Update existing service
                *existing = service;
            } else {
                // Add new service
                services.push(service);
            }
        }
        None => {
            // Create new services array
            document.service = Some(vec![service]);
        }
    }

    // Validate the updated document
    document
        .validate()
        .map_err(|e| Error::BadRequest(format!("Invalid DID document after KYC: {}", e)))?;

    // Update the stored DID document
    stored_did.update_document(cli, document).await?;

    tracing::info!(
        "Added verified attributes to DID: {} (age: {}, gender: {})",
        req.did,
        age,
        verified_customer.gender
    );

    Ok(Json(ReconcilIdentityVerificationResponse {
        result,
        did_document: stored_did,
    }))
}
