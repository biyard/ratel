use crate::common::models::auth::{AdminUser, User};
use crate::common::types::MembershipPartition;
use crate::features::admin::*;
use crate::features::admin::types::AdminError;
#[cfg(feature = "server")]
use crate::features::membership::models::{
    ensure_team_membership_monthly_refill, ensure_user_membership_monthly_refill,
};
use crate::features::membership::models::{
    Membership, MembershipTier, TeamMembership, UserMembership,
};
use crate::features::posts::models::Team;

use super::MembershipGrantTargetType;

const ENTERPRISE_NAME: &str = "Enterprise";
const PAGE_SIZE: i32 = 10;
const USER_BOOKMARK_PREFIX: &str = "user:";
const TEAM_BOOKMARK_PREFIX: &str = "team:";
const TEAM_PHASE_BOOKMARK: &str = "team";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnterpriseMembershipGrantListItem {
    pub target_type: MembershipGrantTargetType,
    pub username: String,
    pub remaining_credits: i64,
    pub max_credits_per_space: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
enum EnterpriseMembershipPhase {
    #[default]
    User,
    Team,
}

fn parse_bookmark(bookmark: Option<String>) -> Result<(EnterpriseMembershipPhase, Option<String>)> {
    match bookmark {
        None => Ok((EnterpriseMembershipPhase::User, None)),
        Some(bookmark) if bookmark == TEAM_PHASE_BOOKMARK => {
            Ok((EnterpriseMembershipPhase::Team, None))
        }
        Some(bookmark) if bookmark.starts_with(USER_BOOKMARK_PREFIX) => Ok((
            EnterpriseMembershipPhase::User,
            Some(bookmark[USER_BOOKMARK_PREFIX.len()..].to_string()),
        )),
        Some(bookmark) if bookmark.starts_with(TEAM_BOOKMARK_PREFIX) => Ok((
            EnterpriseMembershipPhase::Team,
            Some(bookmark[TEAM_BOOKMARK_PREFIX.len()..].to_string()),
        )),
        Some(bookmark) => {
            crate::error!("invalid bookmark: {bookmark}");
            Err(AdminError::InvalidBookmark.into())
        }
    }
}

#[get("/api/admin/memberships/enterprise?bookmark", _user: AdminUser)]
pub async fn list_enterprise_memberships(
    bookmark: Option<String>,
) -> Result<ListResponse<EnterpriseMembershipGrantListItem>> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let enterprise_tier = MembershipTier::Enterprise(ENTERPRISE_NAME.to_string());
    let Some(enterprise_membership) = Membership::get(
        cli,
        Partition::Membership(enterprise_tier.to_string()),
        Some(EntityType::Membership),
    )
    .await?
    else {
        return Ok(ListResponse::default());
    };

    let (phase, bookmark) = parse_bookmark(bookmark)?;
    let membership_pk: MembershipPartition = enterprise_membership.pk.clone().into();
    let max_credits_per_space = enterprise_membership.max_credits_per_space;
    let mut items = Vec::with_capacity(PAGE_SIZE as usize);

    if matches!(phase, EnterpriseMembershipPhase::User) {
        let mut user_opt = UserMembership::opt().limit(PAGE_SIZE);
        if let Some(bookmark) = bookmark.clone() {
            user_opt = user_opt.bookmark(bookmark);
        }

        let (user_memberships, next_user_bookmark) =
            UserMembership::find_by_membership(cli, membership_pk.clone(), user_opt).await?;

        for membership in user_memberships {
            if items.len() >= PAGE_SIZE as usize {
                break;
            }
            let membership = ensure_user_membership_monthly_refill(cli, membership).await?;

            let Some(user) = User::get(cli, membership.pk.clone(), Some(EntityType::User)).await?
            else {
                continue;
            };

            items.push(EnterpriseMembershipGrantListItem {
                target_type: MembershipGrantTargetType::User,
                username: user.username,
                remaining_credits: membership.remaining_credits,
                max_credits_per_space,
            });
        }

        if next_user_bookmark.is_some() {
            return Ok(ListResponse {
                items,
                bookmark: next_user_bookmark
                    .map(|bookmark| format!("{USER_BOOKMARK_PREFIX}{bookmark}")),
            });
        }

        if items.len() == PAGE_SIZE as usize {
            let (team_memberships, _) = TeamMembership::find_by_membership(
                cli,
                membership_pk.clone(),
                TeamMembership::opt().limit(1),
            )
            .await?;

            return Ok(ListResponse {
                items,
                bookmark: if team_memberships.is_empty() {
                    None
                } else {
                    Some(TEAM_PHASE_BOOKMARK.to_string())
                },
            });
        }
    }

    let remaining = PAGE_SIZE - items.len() as i32;
    if remaining > 0 {
        let mut team_opt = TeamMembership::opt().limit(remaining);
        if let Some(bookmark) = match phase {
            EnterpriseMembershipPhase::User => None,
            EnterpriseMembershipPhase::Team => bookmark,
        } {
            team_opt = team_opt.bookmark(bookmark);
        }

        let (team_memberships, next_team_bookmark) =
            TeamMembership::find_by_membership(cli, membership_pk, team_opt).await?;

        for membership in team_memberships {
            #[cfg(feature = "server")]
            let membership = ensure_team_membership_monthly_refill(cli, membership).await?;
            #[cfg(not(feature = "server"))]
            let membership = membership;
            let Some(team) = Team::get(cli, membership.pk.clone(), Some(EntityType::Team)).await?
            else {
                continue;
            };

            items.push(EnterpriseMembershipGrantListItem {
                target_type: MembershipGrantTargetType::Team,
                username: team.username,
                remaining_credits: membership.remaining_credits,
                max_credits_per_space,
            });
        }

        return Ok(ListResponse {
            items,
            bookmark: next_team_bookmark
                .map(|bookmark| format!("{TEAM_BOOKMARK_PREFIX}{bookmark}")),
        });
    }

    Ok(ListResponse {
        items,
        bookmark: None,
    })
}
