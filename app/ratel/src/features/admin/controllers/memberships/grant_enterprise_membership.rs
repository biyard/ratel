use crate::common::models::auth::{AdminUser, User};
use crate::features::admin::*;
use crate::features::admin::types::AdminError;
use crate::features::membership::models::{
    ENTERPRISE_MAX_CREDITS_PER_SPACE, ENTERPRISE_MONTHLY_REFILL_CREDITS, Membership,
    MembershipResponse, MembershipTier, TeamMembership, UserMembership,
};
use crate::features::posts::models::Team;

const ENTERPRISE_DISPLAY_ORDER: i32 = 999;
const ENTERPRISE_NAME: &str = "Enterprise";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum MembershipGrantTargetType {
    #[default]
    User,
    Team,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GrantEnterpriseMembershipRequest {
    pub username: String,
    pub target_type: MembershipGrantTargetType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantEnterpriseMembershipResponse {
    pub username: String,
    pub target_type: MembershipGrantTargetType,
    pub membership: MembershipResponse,
    pub remaining_credits: i64,
    pub max_credits_per_space: i64,
}

fn build_enterprise_membership(existing: Option<Membership>) -> Membership {
    let now = crate::common::utils::time::now();
    let tier = MembershipTier::Enterprise(ENTERPRISE_NAME.to_string());
    let pk = Partition::Membership(tier.to_string());
    let created_at = existing
        .as_ref()
        .map_or(now, |membership| membership.created_at);

    Membership {
        pk,
        sk: EntityType::Membership,
        created_at,
        updated_at: now,
        credits: ENTERPRISE_MONTHLY_REFILL_CREDITS,
        tier,
        price_dollars: 0,
        price_won: 0,
        display_order: ENTERPRISE_DISPLAY_ORDER,
        duration_days: -1,
        max_credits_per_space: ENTERPRISE_MAX_CREDITS_PER_SPACE,
        is_active: true,
        display_order_indexed: ENTERPRISE_DISPLAY_ORDER,
    }
}

#[post("/api/admin/memberships/enterprise", _user: AdminUser)]
pub async fn grant_enterprise_membership(
    req: GrantEnterpriseMembershipRequest,
) -> Result<GrantEnterpriseMembershipResponse> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let username = req.username.trim().to_string();
    if username.is_empty() {
        return Err(AdminError::UsernameRequired.into());
    }

    let enterprise_tier = MembershipTier::Enterprise(ENTERPRISE_NAME.to_string());
    let existing_membership = Membership::get(
        cli,
        Partition::Membership(enterprise_tier.to_string()),
        Some(EntityType::Membership),
    )
    .await?;
    let enterprise_membership = build_enterprise_membership(existing_membership);
    enterprise_membership.upsert(cli).await?;

    let membership_pk = enterprise_membership.pk.clone().into();

    let remaining_credits = match req.target_type {
        MembershipGrantTargetType::User => {
            let (users, _) = User::find_by_username(cli, &username, User::opt().limit(1)).await?;
            let user = users
                .into_iter()
                .find(|user| user.username == username)
                .ok_or_else(|| Error::NotFound("User not found".to_string()))?;

            let mut user_membership = UserMembership::new(
                user.pk.into(),
                membership_pk,
                enterprise_membership.duration_days,
                enterprise_membership.credits,
            )?;
            user_membership.auto_renew = false;
            user_membership.next_membership = None;
            user_membership.monthly_refill_credits = ENTERPRISE_MONTHLY_REFILL_CREDITS;
            user_membership.next_refill_at =
                crate::common::utils::time::add_one_month(crate::common::utils::time::now())
                    .unwrap_or(crate::common::utils::time::now() + 30 * 24 * 60 * 60 * 1_000);
            user_membership.upsert(cli).await?;
            user_membership.remaining_credits
        }
        MembershipGrantTargetType::Team => {
            let team_query_option = Team::opt()
                .sk(Team::compose_gsi2_sk(String::default()))
                .limit(5);
            let (teams, _) =
                Team::find_by_username_prefix(cli, username.clone(), team_query_option).await?;
            let team = teams
                .into_iter()
                .find(|team| team.username == username)
                .ok_or_else(|| Error::NotFound("Team not found".to_string()))?;

            let mut team_membership = TeamMembership::new(
                team.pk.into(),
                membership_pk,
                enterprise_membership.duration_days,
                enterprise_membership.credits,
            )?;
            team_membership.auto_renew = false;
            team_membership.next_membership = None;
            team_membership.monthly_refill_credits = ENTERPRISE_MONTHLY_REFILL_CREDITS;
            team_membership.next_refill_at =
                crate::common::utils::time::add_one_month(crate::common::utils::time::now())
                    .unwrap_or(crate::common::utils::time::now() + 30 * 24 * 60 * 60 * 1_000);
            team_membership.upsert(cli).await?;
            team_membership.remaining_credits
        }
    };

    Ok(GrantEnterpriseMembershipResponse {
        username,
        target_type: req.target_type,
        membership: enterprise_membership.into(),
        remaining_credits,
        max_credits_per_space: ENTERPRISE_MAX_CREDITS_PER_SPACE,
    })
}
