use crate::controllers::normalize_error;
use crate::models::{
    CardInfo, Currency, Membership, MembershipResponse, MembershipTier, PaymentReceipt,
    UserMembership, UserPayment,
};
#[cfg(feature = "server")]
use crate::models::{handle_downgrade, handle_upgrade};
#[cfg(feature = "server")]
use crate::services::portone::PortOne;
use crate::*;
use ratel_auth::User;
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
        let conf = crate::config::get();
        let cli = conf.common.dynamodb();
        let portone = PortOne::new(conf.portone.api_secret);

        let user_membership =
            UserMembership::get(cli, user.pk.clone(), Some(EntityType::UserMembership)).await?;

        let (user_membership, current_membership) = match user_membership {
            Some(user_membership) => {
                let membership_pk: Partition = user_membership.membership_pk.clone().into();
                let membership = Membership::get(cli, membership_pk, Some(EntityType::Membership))
                    .await?
                    .ok_or_else(|| Error::NotFound("Membership not found".to_string()))?;
                (user_membership, membership)
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

                let user_membership = UserMembership::new(
                    user.pk.clone().into(),
                    free_membership.pk.clone().into(),
                    free_membership.duration_days,
                    free_membership.credits,
                )?;
                user_membership.create(cli).await?;

                (user_membership, free_membership)
            }
        };

        if current_membership.tier == req.membership {
            return Err(Error::BadRequest("Membership already active".to_string()));
        }

        let mut ret = ChangeMembershipResponse {
            renewal_date: common::utils::time::now(),
            receipt: None,
            membership: None,
        };

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
                UserPayment::get_by_user(cli, &portone, user_id.clone(), req.card_info).await?;

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
        };

        Ok(ret)
    }
    .await;

    result.map_err(normalize_error)
}
