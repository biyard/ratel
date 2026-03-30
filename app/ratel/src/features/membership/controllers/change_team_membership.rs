use super::*;
use crate::features::membership::*;
use crate::features::membership::controllers::normalize_error;
#[cfg(feature = "server")]
use crate::features::membership::models::{handle_downgrade, handle_upgrade};
use crate::features::membership::models::{
    CardInfo, Currency, Membership, MembershipResponse, MembershipTier, PaymentReceipt,
    TeamMembership, TeamPayment,
};
use crate::features::auth::User;
use crate::features::posts::models::Team;
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use crate::features::membership::services::portone::PortOne;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "camelCase")]
pub struct ChangeTeamMembershipRequest {
    pub membership: MembershipTier,
    pub currency: Currency,
    pub card_info: Option<CardInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "camelCase")]
pub struct ChangeTeamMembershipResponse {
    pub renewal_date: i64,
    pub receipt: Option<PaymentReceipt>,
    pub membership: Option<MembershipResponse>,
}

#[post("/v3/teams/:username/memberships", user: User, team: Team, permissions: TeamGroupPermissions)]
pub async fn change_team_membership_handler(
    username: String,
    req: ChangeTeamMembershipRequest,
) -> Result<ChangeTeamMembershipResponse> {
    let result = async {
        if !permissions.contains(TeamGroupPermission::TeamAdmin) {
            return Err(Error::NotFound("Permission denied".to_string()));
        }

        let conf = crate::features::membership::config::get();
        let cli = conf.common.dynamodb();
        let portone = PortOne::new(conf.portone.api_secret);

        let team_membership =
            TeamMembership::get(cli, team.pk.clone(), Some(EntityType::TeamMembership)).await?;

        let (team_membership, current_membership) = match team_membership {
            Some(team_membership) => {
                let membership_pk: Partition = team_membership.membership_pk.clone().into();
                let membership = Membership::get(cli, membership_pk, Some(EntityType::Membership))
                    .await?
                    .ok_or_else(|| Error::NotFound("Membership not found".to_string()))?;
                (team_membership, membership)
            }
            None => {
                let free_membership_pk = Partition::Membership(MembershipTier::Free.to_string());
                let free_membership = Membership::get(
                    cli,
                    free_membership_pk.clone(),
                    Some(EntityType::Membership),
                )
                .await?
                .ok_or_else(|| Error::NotFound("Membership not found".to_string()))?;

                let team_membership = TeamMembership::new(
                    team.pk.clone().into(),
                    free_membership.pk.clone().into(),
                    free_membership.duration_days,
                    free_membership.credits,
                )?;
                team_membership.create(cli).await?;

                (team_membership, free_membership)
            }
        };

        if current_membership.tier == req.membership {
            return Err(Error::BadRequest("Membership already active".to_string()));
        }

        let mut ret = ChangeTeamMembershipResponse {
            renewal_date: crate::common::utils::time::now(),
            receipt: None,
            membership: None,
        };

        let team_id: TeamPartition = team_membership.pk.clone().into();

        if req.membership < current_membership.tier {
            let (team_payment, _) =
                TeamPayment::get_by_team(cli, &portone, team_id.clone(), None).await?;

            let new_membership = handle_downgrade(
                cli,
                &portone,
                &team_membership,
                Some(&team_payment),
                req.membership.clone(),
                "team",
            )
            .await?;

            ret.renewal_date = team_membership.expired_at + 1;
            ret.membership = Some(new_membership.into());
        } else {
            let (team_payment, should_update) =
                TeamPayment::get_by_team(cli, &portone, team_id.clone(), req.card_info).await?;

            let (team_purchase, new_membership) = handle_upgrade(
                cli,
                &portone,
                &team_membership,
                current_membership,
                req.membership.clone(),
                req.currency,
                team_payment,
                should_update,
                |new_membership, purchase_pk| {
                    let new_team_membership = TeamMembership::new(
                        team_id.clone(),
                        new_membership.pk.clone().into(),
                        new_membership.duration_days,
                        new_membership.credits,
                    )?
                    .with_purchase_id(purchase_pk.clone())
                    .with_auto_renew(true);
                    Ok(new_team_membership.upsert_transact_write_item())
                },
            )
            .await?;

            ret.receipt = Some(team_purchase.into());
            ret.membership = Some(new_membership.into());
        };

        Ok(ret)
    }
    .await;

    result.map_err(normalize_error)
}
