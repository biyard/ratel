use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(DynamoEntity))]
pub struct UserMembershipLocal {
    pub pk: Partition,
    pub sk: EntityType,
    pub created_at: i64,
    pub updated_at: i64,
    pub expired_at: i64,
    pub membership_pk: MembershipPartition,
    pub status: String,
    pub total_credits: i64,
    pub remaining_credits: i64,
    pub auto_renew: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct MembershipResponse {
    pub tier: String,
    pub total_credits: i64,
    pub remaining_credits: i64,
    pub expired_at: i64,
    pub auto_renew: bool,
    pub status: String,
}

#[get("/api/me/membership", user: ratel_auth::User)]
pub async fn get_membership_handler() -> Result<MembershipResponse> {
    let conf = common::config::ServerConfig::default();
    let cli = conf.dynamodb();

    let membership = UserMembershipLocal::get(
        cli,
        user.pk.clone(),
        Some(EntityType::UserMembership),
    )
    .await?;

    match membership {
        Some(m) => Ok(MembershipResponse {
            tier: m.membership_pk.to_string(),
            total_credits: m.total_credits,
            remaining_credits: m.remaining_credits,
            expired_at: m.expired_at,
            auto_renew: m.auto_renew,
            status: m.status,
        }),
        None => Ok(MembershipResponse {
            tier: "Free".to_string(),
            total_credits: 0,
            remaining_credits: 0,
            expired_at: 0,
            auto_renew: false,
            status: "Active".to_string(),
        }),
    }
}
