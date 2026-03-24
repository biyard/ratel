use crate::features::spaces::pages::apps::apps::general::*;
use crate::spaces::{InvitationStatus, SpaceInvitationMember};

const DEFAULT_LIMIT: i32 = 20;
const MAX_LIMIT: i32 = 100;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpaceInvitationListItem {
    pub user_id: UserPartition,
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
    pub email: String,
    pub status: InvitationStatus,
    pub created_at: i64,
}

impl From<SpaceInvitationMember> for SpaceInvitationListItem {
    fn from(member: SpaceInvitationMember) -> Self {
        Self {
            user_id: member.user_pk.into(),
            display_name: member.display_name,
            profile_url: member.profile_url,
            username: member.username,
            email: member.email,
            status: member.status,
            created_at: member.created_at,
        }
    }
}

#[get(
    "/api/spaces/{space_id}/participants/invitations?bookmark&limit",
    role: SpaceUserRole
)]
pub async fn list_space_invitations(
    space_id: SpacePartition,
    bookmark: Option<String>,
    limit: Option<i32>,
) -> Result<ListResponse<SpaceInvitationListItem>> {
    if role != SpaceUserRole::Creator {
        return Err(Error::NoPermission);
    }

    let common_config = crate::common::CommonConfig::default();
    let dynamo = common_config.dynamodb();
    let space_pk: Partition = space_id.into();

    let mut opt = SpaceInvitationMember::opt()
        .sk(EntityType::SpaceInvitationMember(String::default()).to_string())
        .limit(limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT));

    if let Some(bookmark) = bookmark {
        opt = opt.bookmark(bookmark);
    }

    let (members, bookmark) = SpaceInvitationMember::query(dynamo, &space_pk, opt).await?;
    let items = members
        .into_iter()
        .map(SpaceInvitationListItem::from)
        .collect();

    Ok(ListResponse { items, bookmark })
}
