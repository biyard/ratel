use super::super::*;
use crate::common::services::ClaimSignatureResponse;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClaimSignatureRequest {
    pub month: String,
    pub wallet_address: String,
}

#[post("/api/me/points/claim-signature", user: crate::features::auth::User)]
pub async fn request_claim_signature_handler(
    body: ClaimSignatureRequest,
) -> Result<ClaimSignatureResponse> {
    let cfg = crate::common::CommonConfig::default();
    let biyard = cfg.biyard();

    biyard
        .request_claim_signature(user.pk.clone(), body.month, body.wallet_address)
        .await
}
