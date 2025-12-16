use crate::{
    features::{
        membership::{Membership, MembershipTier, UserMembership},
        payment::{UserPayment, UserPurchase},
    },
    utils::time::after_days_from_now_rfc_3339,
};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct PortoneRequest {
    pub payment_id: String,
    pub status: String,
    pub tx_id: String,
}

pub async fn portone_handler(
    State(AppState {
        dynamo, portone, ..
    }): State<AppState>,
    NoApi(headers): NoApi<HeaderMap>,
    Json(req): Json<PortoneRequest>,
) -> Result<()> {
    debug!(
        "Incomming PortOne hook: {:?} with headers {:?}",
        req, headers
    );
    let cli = &dynamo.client;
    let opt = UserPurchase::opt_one();

    let (user_purchase, _bm) = UserPurchase::find_by_payment_id(cli, &req.payment_id, opt).await?;

    let user_purchase = user_purchase
        .first()
        .ok_or_else(|| Error::NoUserPurchaseFound)?;

    let user_pk = user_purchase.pk.0.clone();
    let user_id: UserPartition = user_pk.clone().into();
    let amount = user_purchase.amount;
    let payment_id = req.payment_id;

    // Schedule the next payment for membership renewal
    let (user_payment, _should_update) =
        UserPayment::get_by_user(cli, &portone, user_id.clone(), None).await?;

    let tier: MembershipTier = user_purchase.tx_type.clone().try_into()?;

    let membership = Membership::get_by_membership_tier(cli, &tier).await?;
    let user_membership = UserMembership::get(cli, &user_pk, Some(EntityType::UserMembership))
        .await?
        .ok_or_else(|| crate::Error::NoUserMembershipFound)?;

    let scheduled_date =
        user_membership
            .renewal_date_rfc_3339()
            .unwrap_or(after_days_from_now_rfc_3339(
                membership.duration_days as i64,
            ));

    let next_purchase = user_payment
        .schedule_next_membership_purchase(
            &portone,
            user_purchase.tx_type.clone(),
            amount,
            user_purchase.currency,
            scheduled_date.clone(),
        )
        .await;

    if next_purchase.is_err() {
        notify!("Failed to schedule next membership purchase for user ID: {user_id}");
        UserMembership::updater(&user_pk, &EntityType::UserMembership)
            .with_auto_renew(false)
            .with_next_membership(MembershipPartition(MembershipTier::Free.to_string()))
            .execute(cli)
            .await?;

        return Ok(());
    }

    let UserPurchase {
        payment_id: next_payment_id,
        ..
    } = next_purchase.unwrap();

    notify!(
        r#"Membership Renewal
  User ID: {user_id}
  Membership: {tier}
  Amount: {amount}
  Payment ID: {payment_id}
  Next Payment ID: {next_payment_id}
  Next Payment Date: {scheduled_date}
  "#
    );

    Ok(())
}
