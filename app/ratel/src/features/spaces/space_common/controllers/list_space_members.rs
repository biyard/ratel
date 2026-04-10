use crate::common::models::space::SpaceParticipant;
use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpaceMemberResponse {
    pub user_id: UserPartition,
    pub display_name: String,
    pub username: String,
    pub profile_url: String,
}

#[cfg(feature = "server")]
impl From<SpaceParticipant> for SpaceMemberResponse {
    fn from(p: SpaceParticipant) -> Self {
        Self {
            user_id: p.user_pk.into(),
            display_name: p.display_name,
            username: p.username,
            profile_url: p.profile_url,
        }
    }
}

#[get("/api/spaces/{space_id}/members?bookmark", role: SpaceUserRole)]
pub async fn list_space_members(
    space_id: SpacePartition,
    bookmark: Option<String>,
) -> Result<ListResponse<SpaceMemberResponse>> {
    let common_config = crate::common::CommonConfig::default();
    let dynamo = common_config.dynamodb();
    let space_pk: Partition = space_id.into();

    let opts = SpaceParticipant::opt_with_bookmark(bookmark).limit(50);
    let (participants, next_bookmark) =
        SpaceParticipant::find_by_space(dynamo, space_pk, opts).await?;

    let members = participants
        .into_iter()
        .map(SpaceMemberResponse::from)
        .collect();

    Ok(ListResponse {
        items: members,
        bookmark: next_bookmark,
    })
}
