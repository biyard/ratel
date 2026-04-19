use super::super::*;
use crate::common::services::ClaimSignatureResponse;
use crate::features::posts::models::Team;
use crate::features::social::pages::member::dto::TeamRole;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TeamClaimSignatureRequest {
    pub month: String,
    pub wallet_address: String,
}

#[post("/api/teams/:team_pk/points/claim-signature", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn request_team_claim_signature_handler(
    team_pk: TeamPartition,
    body: TeamClaimSignatureRequest,
) -> Result<ClaimSignatureResponse> {
    let cfg = crate::common::CommonConfig::default();
    let _ = user;
    let _ = team_pk;
    let team_pk = team.pk.clone();

    if !role.is_admin_or_owner() {
        return Err(crate::common::Error::NoPermission);
    }

    cfg.biyard()
        .request_claim_signature(team_pk, body.month, body.wallet_address)
        .await
}
