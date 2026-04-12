use super::*;
use crate::features::membership::models::{
    Currency, Membership, PurchaseStatus, TeamMembership, TeamPayment, TeamPurchase,
    TransactionType, UserMembership, UserPayment, UserPurchase,
};
#[cfg(feature = "server")]
use crate::features::membership::services::portone::PortOne;
use crate::features::membership::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct PortoneRequest {
    pub payment_id: String,
    pub status: String,
    pub tx_id: String,
}

#[cfg(feature = "server")]
fn should_schedule_renewal(membership: &Membership, currency: Currency) -> bool {
    membership.duration_days > 0 && membership.price_in_currency(currency) > 0
}

#[cfg(feature = "server")]
async fn has_user_scheduled_purchase(
    cli: &aws_sdk_dynamodb::Client,
    user_pk: &Partition,
) -> Result<bool> {
    let opt = UserPurchase::opt_one().sk(PurchaseStatus::Scheduled.to_string());
    let pk = CompositePartition::user_purchase_pk(user_pk.clone());
    let (purchases, _) = UserPurchase::find_by_status(cli, &pk, opt).await?;
    Ok(!purchases.is_empty())
}

#[cfg(feature = "server")]
async fn has_team_scheduled_purchase(
    cli: &aws_sdk_dynamodb::Client,
    team_pk: &Partition,
) -> Result<bool> {
    let opt = TeamPurchase::opt_one().sk(PurchaseStatus::Scheduled.to_string());
    let pk = CompositePartition::team_purchase_pk(team_pk.clone());
    let (purchases, _) = TeamPurchase::find_by_status(cli, &pk, opt).await?;
    Ok(!purchases.is_empty())
}

#[cfg(feature = "server")]
async fn schedule_user_renewal(
    cli: &aws_sdk_dynamodb::Client,
    portone: &PortOne,
    membership: &Membership,
    user_membership: &UserMembership,
    user_payment: &UserPayment,
    currency: Currency,
) -> Result<()> {
    if !user_membership.auto_renew || !should_schedule_renewal(membership, currency) {
        return Ok(());
    }

    if has_user_scheduled_purchase(cli, &user_membership.pk).await? {
        return Ok(());
    }

    let Some(scheduled_at) = user_membership.renewal_date_rfc_3339() else {
        return Ok(());
    };

    let scheduled_purchase = user_payment
        .schedule_next_membership_purchase(
            portone,
            TransactionType::PurchaseMembership(membership.tier.clone()),
            membership.price_in_currency(currency),
            currency,
            scheduled_at,
        )
        .await?;

    scheduled_purchase.create(cli).await?;
    Ok(())
}

#[cfg(feature = "server")]
async fn schedule_team_renewal(
    cli: &aws_sdk_dynamodb::Client,
    portone: &PortOne,
    membership: &Membership,
    team_membership: &TeamMembership,
    team_payment: &TeamPayment,
    currency: Currency,
) -> Result<()> {
    if !team_membership.auto_renew || !should_schedule_renewal(membership, currency) {
        return Ok(());
    }

    if has_team_scheduled_purchase(cli, &team_membership.pk).await? {
        return Ok(());
    }

    let Some(scheduled_at) = team_membership.renewal_date_rfc_3339() else {
        return Ok(());
    };

    let scheduled_purchase = team_payment
        .schedule_next_membership_purchase(
            portone,
            TransactionType::PurchaseMembership(membership.tier.clone()),
            membership.price_in_currency(currency),
            currency,
            scheduled_at,
        )
        .await?;

    scheduled_purchase.create(cli).await?;
    Ok(())
}

#[cfg(feature = "server")]
async fn disable_user_auto_renew(
    cli: &aws_sdk_dynamodb::Client,
    membership: &UserMembership,
) -> Result<()> {
    let mut updated = membership.clone();
    updated.auto_renew = false;
    updated.next_membership = None;
    updated.upsert(cli).await?;
    Ok(())
}

#[cfg(feature = "server")]
async fn disable_team_auto_renew(
    cli: &aws_sdk_dynamodb::Client,
    membership: &TeamMembership,
) -> Result<()> {
    let mut updated = membership.clone();
    updated.auto_renew = false;
    updated.next_membership = None;
    updated.upsert(cli).await?;
    Ok(())
}

#[cfg(feature = "server")]
async fn handle_user_purchase(
    cli: &aws_sdk_dynamodb::Client,
    portone: &PortOne,
    req: &PortoneRequest,
    user_purchase: UserPurchase,
) -> Result<bool> {
    let user_pk = user_purchase.pk.0.clone();
    let user_id: UserPartition = user_pk.clone().into();
    let (user_payment, _) = UserPayment::get_by_user(cli, portone, user_id.clone(), None).await?;

    let TransactionType::PurchaseMembership(tier) = user_purchase.tx_type.clone() else {
        return Ok(true);
    };
    let membership = Membership::get_by_membership_tier(cli, &tier).await?;

    match user_purchase.status {
        PurchaseStatus::Scheduled => {
            let renewed_membership = UserMembership::new(
                user_id,
                membership.pk.clone().into(),
                membership.duration_days,
                membership.credits,
            )?
            .with_auto_renew(true);

            let txs = vec![
                UserPurchase::updater(&user_purchase.pk, &user_purchase.sk)
                    .with_status(PurchaseStatus::Success)
                    .with_tx_id(req.tx_id.clone())
                    .transact_write_item(),
                renewed_membership.upsert_transact_write_item(),
            ];

            crate::transact_write_all_items_with_failover!(cli, txs);

            if let Err(err) = schedule_user_renewal(
                cli,
                portone,
                &membership,
                &renewed_membership,
                &user_payment,
                user_purchase.currency,
            )
            .await
            {
                tracing::error!("failed to schedule next user membership renewal: {:?}", err);
                disable_user_auto_renew(cli, &renewed_membership).await?;
            }
        }
        PurchaseStatus::Success => {
            let user_membership =
                UserMembership::get(cli, user_pk.clone(), Some(EntityType::UserMembership))
                    .await?
                    .ok_or_else(|| Error::NotFound("User membership not found".to_string()))?;

            if let Err(err) = schedule_user_renewal(
                cli,
                portone,
                &membership,
                &user_membership,
                &user_payment,
                user_purchase.currency,
            )
            .await
            {
                tracing::error!("failed to schedule next user membership renewal: {:?}", err);
                disable_user_auto_renew(cli, &user_membership).await?;
            }
        }
        PurchaseStatus::Canceled => {}
    }

    Ok(true)
}

#[cfg(feature = "server")]
async fn handle_team_purchase(
    cli: &aws_sdk_dynamodb::Client,
    portone: &PortOne,
    req: &PortoneRequest,
    team_purchase: TeamPurchase,
) -> Result<bool> {
    let team_pk = team_purchase.pk.0.clone();
    let team_id: TeamPartition = team_pk.clone().into();
    let (team_payment, _) = TeamPayment::get_by_team(cli, portone, team_id.clone(), None).await?;

    let TransactionType::PurchaseMembership(tier) = team_purchase.tx_type.clone() else {
        return Ok(true);
    };
    let membership = Membership::get_by_membership_tier(cli, &tier).await?;

    match team_purchase.status {
        PurchaseStatus::Scheduled => {
            let renewed_membership = TeamMembership::new(
                team_id,
                membership.pk.clone().into(),
                membership.duration_days,
                membership.credits,
            )?
            .with_auto_renew(true);

            let txs = vec![
                TeamPurchase::updater(&team_purchase.pk, &team_purchase.sk)
                    .with_status(PurchaseStatus::Success)
                    .with_tx_id(req.tx_id.clone())
                    .transact_write_item(),
                renewed_membership.upsert_transact_write_item(),
            ];

            crate::transact_write_all_items_with_failover!(cli, txs);

            if let Err(err) = schedule_team_renewal(
                cli,
                portone,
                &membership,
                &renewed_membership,
                &team_payment,
                team_purchase.currency,
            )
            .await
            {
                tracing::error!("failed to schedule next team membership renewal: {:?}", err);
                disable_team_auto_renew(cli, &renewed_membership).await?;
            }
        }
        PurchaseStatus::Success => {
            let team_membership =
                TeamMembership::get(cli, team_pk.clone(), Some(EntityType::TeamMembership))
                    .await?
                    .ok_or_else(|| Error::NotFound("Team membership not found".to_string()))?;

            if let Err(err) = schedule_team_renewal(
                cli,
                portone,
                &membership,
                &team_membership,
                &team_payment,
                team_purchase.currency,
            )
            .await
            {
                tracing::error!("failed to schedule next team membership renewal: {:?}", err);
                disable_team_auto_renew(cli, &team_membership).await?;
            }
        }
        PurchaseStatus::Canceled => {}
    }

    Ok(true)
}

#[cfg(feature = "server")]
pub async fn handle_portone_webhook(req: PortoneRequest) -> Result<()> {
    let result = async {
        if !req.status.eq_ignore_ascii_case("paid") {
            return Ok(());
        }

        let conf = crate::features::membership::config::get();
        let cli = conf.common.dynamodb();
        let portone = PortOne::new(conf.portone.api_secret);

        let user_opt = UserPurchase::opt_one();
        let (user_purchases, _) =
            UserPurchase::find_by_payment_id(cli, &req.payment_id, user_opt).await?;

        if let Some(user_purchase) = user_purchases.into_iter().next() {
            handle_user_purchase(cli, &portone, &req, user_purchase).await?;
            return Ok(());
        }

        let team_opt = TeamPurchase::opt_one();
        let (team_purchases, _) =
            TeamPurchase::find_by_payment_id(cli, &req.payment_id, team_opt).await?;

        if let Some(team_purchase) = team_purchases.into_iter().next() {
            handle_team_purchase(cli, &portone, &req, team_purchase).await?;
            return Ok(());
        }

        Err(Error::NotFound("Purchase not found".to_string()))
    }
    .await;

    result.map_err(normalize_error)
}

#[cfg(not(feature = "server"))]
pub async fn handle_portone_webhook(_req: PortoneRequest) -> Result<()> {
    Err(crate::features::membership::types::MembershipPaymentError::WebhookProcessingFailed.into())
}
