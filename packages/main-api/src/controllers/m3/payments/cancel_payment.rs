use super::dto::{AdminCancelPaymentRequest, AdminCancelPaymentResponse};
use crate::features::membership::{MembershipTier, UserMembership};
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

    // 1. Get payment info from PortOne
    let payment = portone.get_payment(&payment_id).await?;
    
    // 2. Extract user_pk
    let user_pk = payment
        .user_partition()
        .ok_or_else(|| Error::InvalidPartitionKey(payment.customer.id.clone()))?;

    // 3. Cancel payment via Portone API
    let cancel_response = portone
        .cancel_payment(
            &payment_id,
            req.reason,
            None, // Full refund
            Some(crate::services::portone::PortoneCancelRequester::Admin),
        )
        .await?;

    // 4. Downgrade user membership to Free tier
    let free_membership_pk = Partition::Membership(MembershipTier::Free.to_string());
    UserMembership::updater(&user_pk, &EntityType::UserMembership)
        .with_membership_pk(free_membership_pk.into())
        .with_auto_renew(false)
        .execute(cli)
        .await?;

    tracing::debug!(
        "Payment {} cancelled and user {} membership downgraded to Free",
        payment_id, user_pk
    );

    Ok(Json(cancel_response.cancellation.into()))
}
