use crate::features::membership::*;
use crate::features::payment::*;
use crate::services::portone::Currency;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct ChangeMembershipRequest {
    #[schemars(description = "memberhsip tier to be paid for")]
    pub membership: MembershipTier,
    pub currency: Currency,
    pub card_info: Option<CardInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema, Default)]
pub struct ChangeMembershipResponse {
    #[schemars(description = "Status of the operation")]
    #[serde(default)]
    pub renewal_date: i64,
    pub receipt: Option<PaymentReceipt>,
    pub membership: Option<MembershipResponse>,
}

pub async fn change_membership_handler(
    State(AppState {
        dynamo, portone, ..
    }): State<AppState>,
    NoApi(user): NoApi<User>,
    Json(req): Json<ChangeMembershipRequest>,
) -> Result<Json<ChangeMembershipResponse>> {
    tracing::debug!("change_membership_handler request: {:?}", req);
    let cli = &dynamo.client;

    // Try to get existing membership, or create a Free one if it doesn't exist
    let (user_membership, current_membership) = match user.get_membership(cli).await {
        Ok(result) => result,
        Err(Error::NoUserMembershipFound) | Err(Error::NoMembershipFound) => {
            // User doesn't have a membership yet, create a Free membership
            let free_membership_pk = Partition::Membership(MembershipTier::Free.to_string());
            let free_membership = Membership::get(
                cli,
                free_membership_pk.clone(),
                Some(EntityType::Membership),
            )
            .await?
            .ok_or_else(|| Error::NoMembershipFound)?;

            let user_membership = UserMembership::new(
                user.pk.clone().into(),
                free_membership.pk.clone().into(),
                free_membership.duration_days,
                free_membership.credits,
            )?;
            user_membership.create(cli).await?;

            (user_membership, free_membership)
        }
        Err(e) => return Err(e),
    };

    if current_membership.tier == req.membership {
        return Err(Error::MembershipAlreadyActive);
    }

    let mut ret = ChangeMembershipResponse {
        renewal_date: now(),
        receipt: None,
        membership: None,
    };

    let user_id: UserPartition = user_membership.pk.clone().into();

    if req.membership < current_membership.tier {
        // Downgrade: Get payment to cancel scheduled payments
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
        // Upgrade
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

    Ok(Json(ret))
}
