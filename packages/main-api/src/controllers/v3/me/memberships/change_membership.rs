use crate::features::membership::*;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct ChangeMembershipRequest {
    #[schemars(description = "memberhsip tier to be paid for")]
    pub membership: MembershipTier,
    pub card_info: Option<CardInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct CardInfo {
    pub card_number: String,
    pub expiry_year: String,
    pub expiry_month: String,
    pub birth_or_business_registration_number: String,
    pub password_two_digits: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct ChangeMembershipResponse {
    #[schemars(description = "Status of the operation")]
    pub membership: MembershipTier,
    pub renewal_date: i64,
}

pub async fn change_membership_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Json(req): Json<ChangeMembershipRequest>,
) -> Result<Json<ChangeMembershipResponse>> {
    tracing::debug!("Handling request: {:?}", req);
    let cli = &dynamo.client;

    let (user_membership, current_membership) = user.get_membership(cli).await?;

    if current_membership.tier == req.membership {
        return Err(Error::MembershipAlreadyActive);
    }

    let renewal_date = if req.membership < current_membership.tier {
        handle_degrade_membership(cli, &user_membership, req.membership.clone()).await?;
        user_membership.expired_at + 1
    } else {
        handle_upgrade_membership(
            cli,
            &user_membership,
            current_membership,
            req.membership.clone(),
            req.card_info,
        )
        .await?;
        now()
    };

    Ok(Json(ChangeMembershipResponse {
        membership: req.membership,
        renewal_date,
    }))
}

pub async fn handle_degrade_membership(
    cli: &aws_sdk_dynamodb::Client,
    user: &UserMembership,
    new_tier: MembershipTier,
) -> Result<()> {
    // Implement downgrade logic here

    unimplemented!()
}

pub async fn handle_upgrade_membership(
    cli: &aws_sdk_dynamodb::Client,
    user: &UserMembership,
    current_membership: Membership,
    new_tier: MembershipTier,
    card_info: Option<CardInfo>,
) -> Result<()> {
    // Implement upgrade logic here

    unimplemented!()
}
