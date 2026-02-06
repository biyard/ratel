use super::dto::{AdminCancelPaymentRequest, AdminCancelPaymentResponse};
use crate::features::membership::{Membership, MembershipStatus, MembershipTier, UserMembership};
use crate::types::{EntityType, Partition};
use crate::*;

/// Cancel a payment (ServiceAdmin only)
///
/// Cancels the payment via Portone
pub async fn cancel_payment_handler(
    State(AppState { dynamo, portone, .. }): State<AppState>,
    Path(payment_id): Path<String>,
    Json(req): Json<AdminCancelPaymentRequest>,
) -> Result<Json<AdminCancelPaymentResponse>> {
    let cli = &dynamo.client;

    // 1. Cancel payment via Portone API
    let cancel_response = portone
        .cancel_payment(&payment_id, req.reason, req.amount, req.requester)
        .await?;

    // 2. Parse user_pk string to Partition
    let user_pk: Partition = req
        .user_pk
        .parse()
        .map_err(|_| Error::InvalidPartitionKey(req.user_pk.clone()))?;

    // 3. Get Free membership to retrieve credits info
    let free_membership_pk = Partition::Membership(MembershipTier::Free.to_string());
    let free_membership = Membership::get(cli, free_membership_pk.clone(), Some(EntityType::Membership))
        .await?
        .ok_or_else(|| Error::NoMembershipFound)?;

    // 4. Update user membership to Free tier with Active
    UserMembership::updater(&user_pk, &EntityType::UserMembership)
        .with_membership_pk(free_membership.pk.clone().into())
        .with_auto_renew(false)
        .execute(cli)
        .await?;

    debug!(
        "Payment {} cancelled and user {} membership reset to Free with {} credits",
        payment_id, req.user_pk, free_membership.credits
    );

    Ok(Json(cancel_response.cancellation.into()))
}
