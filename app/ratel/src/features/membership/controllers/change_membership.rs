use super::*;
use crate::features::auth::User;
use crate::features::membership::controllers::normalize_error;
use crate::features::membership::models::{
    CardInfo, Currency, Membership, MembershipResponse, MembershipTier, PaymentReceipt,
    TransactionType, UserMembership, UserPayment,
};
#[cfg(feature = "server")]
use crate::features::membership::models::{handle_downgrade, handle_upgrade};
#[cfg(feature = "server")]
use crate::features::membership::services::portone::PortOne;
use crate::features::membership::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "camelCase")]
pub struct ChangeMembershipRequest {
    pub membership: MembershipTier,
    pub currency: Currency,
    pub card_info: Option<CardInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "camelCase")]
pub struct ChangeMembershipResponse {
    pub renewal_date: i64,
    pub receipt: Option<PaymentReceipt>,
    pub membership: Option<MembershipResponse>,
}

#[post("/v3/me/memberships", user: User)]
pub async fn change_membership_handler(
    req: ChangeMembershipRequest,
) -> Result<ChangeMembershipResponse> {
    let result = async {
        let conf = crate::features::membership::config::get();
        let cli = conf.common.dynamodb();
        let portone = PortOne::new(conf.portone.api_secret);

        let mut ret = ChangeMembershipResponse {
            renewal_date: crate::common::utils::time::now(),
            receipt: None,
            membership: None,
        };

        let user_membership =
            UserMembership::get(cli, user.pk.clone(), Some(EntityType::UserMembership)).await?;

        match user_membership {
            Some(user_membership) => {
                let membership_pk: Partition = user_membership.membership_pk.clone().into();
                let current_membership =
                    Membership::get(cli, membership_pk, Some(EntityType::Membership))
                        .await?
                        .ok_or_else(|| Error::NotFound("Membership not found".to_string()))?;

                if current_membership.tier == req.membership {
                    return Err(Error::MembershipAlreadyActive);
                }

                let user_id: UserPartition = user_membership.pk.clone().into();

                if req.membership < current_membership.tier {
                    let (user_payment, _) =
                        UserPayment::get_by_user(cli, &portone, user_id.clone(), None).await?;

                    let new_membership = handle_downgrade(
                        cli,
                        &portone,
                        &user_membership,
                        Some(&user_payment),
                        req.membership.clone(),
                        "user",
                    )
                    .await?;

                    ret.renewal_date = user_membership.expired_at + 1;
                    ret.membership = Some(new_membership.into());
                } else {
                    let (user_payment, should_update) =
                        UserPayment::get_by_user(cli, &portone, user_id.clone(), req.card_info)
                            .await?;

                    let (user_purchase, new_membership) = handle_upgrade(
                        cli,
                        &portone,
                        &user_membership,
                        current_membership,
                        req.membership.clone(),
                        req.currency,
                        user_payment,
                        should_update,
                        |new_membership, purchase_pk| {
                            let new_user_membership = UserMembership::new(
                                user_id.clone(),
                                new_membership.pk.clone().into(),
                                new_membership.duration_days,
                                new_membership.credits,
                            )?
                            .with_purchase_id(purchase_pk.clone())
                            .with_auto_renew(true);
                            Ok(new_user_membership.upsert_transact_write_item())
                        },
                    )
                    .await?;

                    ret.receipt = Some(user_purchase.into());
                    ret.membership = Some(new_membership.into());
                }
            }
            None => {
                if req.membership == MembershipTier::Free {
                    return Err(Error::MembershipAlreadyActive);
                }

                let new_membership =
                    Membership::get_by_membership_tier(cli, &req.membership).await?;
                let user_id: UserPartition = user.pk.clone().into();
                let (user_payment, should_update) =
                    UserPayment::get_by_user(cli, &portone, user_id.clone(), req.card_info).await?;

                let tx_type = TransactionType::PurchaseMembership(req.membership.clone());
                let amount = match req.currency {
                    Currency::Usd => new_membership.price_dollars,
                    Currency::Krw => new_membership.price_won,
                };

                let user_purchase = user_payment
                    .purchase(&portone, tx_type, amount, req.currency)
                    .await?;

                let new_user_membership = UserMembership::new(
                    user_id,
                    new_membership.pk.clone().into(),
                    new_membership.duration_days,
                    new_membership.credits,
                )?
                .with_purchase_id(user_purchase.pk.clone())
                .with_auto_renew(true);

                let mut txs = vec![
                    user_purchase.create_transact_write_item(),
                    new_user_membership.upsert_transact_write_item(),
                ];

                if should_update {
                    txs.push(user_payment.upsert_transact_write_item());
                }

                crate::transact_write_all_items_with_failover!(cli, txs);

                ret.receipt = Some(user_purchase.into());
                ret.membership = Some(new_membership.into());
            }
        }

        Ok(ret)
    }
    .await;

    result.map_err(normalize_error)
}
