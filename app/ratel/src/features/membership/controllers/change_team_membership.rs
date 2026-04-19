use super::*;
use crate::features::auth::User;
use crate::features::membership::controllers::normalize_error;
use crate::features::membership::models::{
    CardInfo, Currency, Membership, MembershipResponse, MembershipTier, PaymentReceipt,
    TeamMembership, TeamPayment, TransactionType,
};
#[cfg(feature = "server")]
use crate::features::membership::models::{handle_downgrade, handle_upgrade};
#[cfg(feature = "server")]
use crate::features::membership::services::portone::PortOne;
use crate::features::membership::*;
use crate::features::posts::models::Team;
use crate::features::social::pages::member::dto::TeamRole;
use serde::{Deserialize, Serialize};

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

#[post("/v3/teams/:username/memberships", user: User, team: Team, role: TeamRole)]
pub async fn change_team_membership_handler(
    username: String,
    req: ChangeTeamMembershipRequest,
) -> Result<ChangeTeamMembershipResponse> {
    let result = async {
        if !role.is_admin_or_owner() {
            return Err(Error::NotFound("Permission denied".to_string()));
        }

        let conf = crate::features::membership::config::get();
        let cli = conf.common.dynamodb();
        let portone = PortOne::new(conf.portone.api_secret);

        let mut ret = ChangeTeamMembershipResponse {
            renewal_date: crate::common::utils::time::now(),
            receipt: None,
            membership: None,
        };

        let team_membership =
            TeamMembership::get(cli, team.pk.clone(), Some(EntityType::TeamMembership)).await?;

        match team_membership {
            Some(team_membership) => {
                let membership_pk: Partition = team_membership.membership_pk.clone().into();
                let current_membership =
                    Membership::get(cli, membership_pk, Some(EntityType::Membership))
                        .await?
                        .ok_or_else(|| Error::NotFound("Membership not found".to_string()))?;

                if current_membership.tier == req.membership {
                    return Err(Error::MembershipAlreadyActive);
                }

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
                        TeamPayment::get_by_team(cli, &portone, team_id.clone(), req.card_info)
                            .await?;

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
                }
            }
            None => {
                if req.membership == MembershipTier::Free {
                    return Err(Error::MembershipAlreadyActive);
                }

                let new_membership =
                    Membership::get_by_membership_tier(cli, &req.membership).await?;
                let team_id: TeamPartition = team.pk.clone().into();
                let (team_payment, should_update) =
                    TeamPayment::get_by_team(cli, &portone, team_id.clone(), req.card_info).await?;

                let tx_type = TransactionType::PurchaseMembership(req.membership.clone());
                let amount = match req.currency {
                    Currency::Usd => new_membership.price_dollars,
                    Currency::Krw => new_membership.price_won,
                };

                let team_purchase = team_payment
                    .purchase(&portone, tx_type, amount, req.currency)
                    .await?;

                let new_team_membership = TeamMembership::new(
                    team_id,
                    new_membership.pk.clone().into(),
                    new_membership.duration_days,
                    new_membership.credits,
                )?
                .with_purchase_id(team_purchase.pk.clone())
                .with_auto_renew(true);

                let mut txs = vec![
                    team_purchase.create_transact_write_item(),
                    new_team_membership.upsert_transact_write_item(),
                ];

                if should_update {
                    txs.push(team_payment.upsert_transact_write_item());
                }

                crate::transact_write_all_items_with_failover!(cli, txs);

                ret.receipt = Some(team_purchase.into());
                ret.membership = Some(new_membership.into());
            }
        }

        Ok(ret)
    }
    .await;

    result.map_err(normalize_error)
}
