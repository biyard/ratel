use crate::dto::{ListItemsResponse, TeamGroupResponse};
use crate::*;

use ratel_post::models::TeamGroup;

#[get("/api/teams/:team_pk/groups?bookmark", user: ratel_auth::OptionalUser)]
pub async fn list_groups_handler(
    team_pk: TeamPartition,
    bookmark: Option<String>,
) -> Result<ListItemsResponse<TeamGroupResponse>> {
    let conf = crate::config::get();
    let cli = conf.common.dynamodb();
    let team_pk: Partition = team_pk.into();

    let mut query_options = TeamGroup::opt()
        .limit(50)
        .sk(EntityType::TeamGroup(String::default()).to_string());

    if let Some(bookmark) = bookmark {
        query_options = query_options.bookmark(bookmark);
    }

    let (groups, next_bookmark) = TeamGroup::query(cli, team_pk, query_options).await?;
    let items = groups.into_iter().map(TeamGroupResponse::from).collect();

    Ok(ListItemsResponse {
        items,
        bookmark: next_bookmark,
    })
}
