use crate::{services::portone::IdentifyResponse, *};

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct ReconcilIdentityVerificationRequest {
    #[schemars(description = "Identity Verifycation ID")]
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct ReconcilIdentityVerificationResponse {
    #[schemars(description = "Status of the operation")]
    pub result: IdentifyResponse,
}

pub async fn reconcil_identity_verification_handler(
    State(AppState { portone, .. }): State<AppState>,
    NoApi(_user): NoApi<User>,
    Json(req): Json<ReconcilIdentityVerificationRequest>,
) -> Result<Json<ReconcilIdentityVerificationResponse>> {
    tracing::debug!("Handling request: {:?}", req);
    let result = portone.identify(&req.id).await?;

    // TODO: reconcil proof containing the validated info
    // gender, age

    Ok(Json(ReconcilIdentityVerificationResponse { result }))
}
