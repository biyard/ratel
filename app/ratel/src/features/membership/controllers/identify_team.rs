use super::*;
use crate::features::auth::User;
use crate::features::membership::controllers::normalize_error;
use crate::features::membership::models::TeamPayment;
#[cfg(feature = "server")]
use crate::features::membership::services::portone::PortOne;
use crate::features::membership::services::portone::VerifiedCustomer;
use crate::features::membership::*;
use crate::features::posts::models::Team;
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "camelCase")]
pub struct TeamIdentificationRequest {
    pub id: String,
}

#[post("/v3/teams/:username/payments/identify", user: User, team: Team, permissions: TeamGroupPermissions)]
pub async fn identify_team_handler(
    username: String,
    req: TeamIdentificationRequest,
) -> Result<VerifiedCustomer> {
    let result = async {
        if !permissions.contains(TeamGroupPermission::TeamAdmin) {
            return Err(Error::NotFound("Permission denied".to_string()));
        }

        let conf = crate::features::membership::config::get();
        let cli = conf.common.dynamodb();
        let portone = PortOne::new(conf.portone.api_secret);

        let result = portone.identify(&req.id).await?;
        let verified = result.verified_customer.clone();

        TeamPayment::new(
            team.pk,
            verified.id.clone(),
            verified.name.clone(),
            verified.birth_date.clone(),
        )
        .create(cli)
        .await?;

        Ok(verified)
    }
    .await;

    result.map_err(normalize_error)
}
