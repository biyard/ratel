use super::super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct MembershipResponse {
    pub tier: String,
    pub total_credits: i64,
    pub remaining_credits: i64,
    pub expired_at: i64,
    pub next_membership: Option<String>,
}

#[get("/api/me/membership", user: ratel_auth::User)]
pub async fn get_membership_handler() -> Result<MembershipResponse> {
    use super::super::models::UserMembershipLocal;

    let conf = common::config::ServerConfig::default();
    let cli = conf.dynamodb();

    let membership =
        UserMembershipLocal::get(cli, user.pk.clone(), Some(EntityType::UserMembership)).await?;

    match membership {
        Some(m) => Ok(MembershipResponse {
            tier: m.membership_pk.to_string(),
            total_credits: m.total_credits,
            remaining_credits: m.remaining_credits,
            expired_at: m.expired_at,
            next_membership: m.next_membership.map(|next| next.to_string()),
        }),
        None => Ok(MembershipResponse {
            tier: "Free".to_string(),
            total_credits: 0,
            remaining_credits: 0,
            expired_at: i64::MAX,
            next_membership: None,
        }),
    }
}
