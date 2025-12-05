use crate::features::payment::UserPayment;
use crate::models::user::{UserDetailResponse, UserMetadata};
use crate::*;

pub type GetInfoResponse = UserDetailResponse;

pub async fn get_info_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<tower_sessions::Session>,
) -> Result<Json<GetInfoResponse>> {
    let user_pk: Partition = session
        .get(SESSION_KEY_USER_ID)
        .await?
        .ok_or(Error::Unauthorized("no session".to_string()))?;
    tracing::debug!("get_info_handler: user_pk = {}", user_pk);
    let payment = UserPayment::get(
        &dynamo.client,
        CompositePartition::user_payment_pk(user_pk.clone()),
        None::<String>,
    );
    let user = UserMetadata::query(&dynamo.client, user_pk);

    let (user, payment) = tokio::try_join!(user, payment)?;
    let mut user: UserDetailResponse = user.into();

    user.is_identified = payment.is_some();
    user.has_billing_key = payment
        .as_ref()
        .and_then(|p| p.billing_key.as_ref())
        .is_some();

    Ok(Json(user))
}
