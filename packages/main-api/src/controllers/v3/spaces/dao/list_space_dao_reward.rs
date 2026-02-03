use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::SpaceDaoRewardUserQueryOption;
use crate::features::spaces::{SpaceDao, SpaceDaoRewardUser};
use crate::types::{EntityType, Permissions, TeamGroupPermission};
use crate::{AppState, Error};
use aide::NoApi;
use axum::Json;
use axum::extract::{Path, Query, State};
use bdk::prelude::*;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct ListSpaceDaoRewardQuery {
    pub bookmark: Option<String>,
    pub limit: Option<i32>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct ListSpaceDaoRewardResponse {
    pub items: Vec<SpaceDaoRewardUser>,
    pub bookmark: Option<String>,
    pub remaining_count: i64,
    pub total_count: i64,
}

pub async fn list_space_dao_reward_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Query(ListSpaceDaoRewardQuery { bookmark, limit }): Query<ListSpaceDaoRewardQuery>,
) -> Result<Json<ListSpaceDaoRewardResponse>, Error> {
    permissions.permitted(TeamGroupPermission::SpaceRead)?;

    let mut opt = if let Some(b) = &bookmark {
        SpaceDaoRewardUserQueryOption::builder()
            .sk("SPACE_DAO_REWARD#".into())
            .bookmark(b.clone())
    } else {
        SpaceDaoRewardUserQueryOption::builder().sk("SPACE_DAO_REWARD#".into())
    };

    if let Some(limit) = limit {
        opt = opt.limit(limit);
    }

    let (items, bookmark) = SpaceDaoRewardUser::query(&dynamo.client, space_pk.clone(), opt)
        .await
        .map_err(|err| {
            tracing::error!(
                "list_space_dao_reward: failed to query reward users: space={} err={:?}",
                space_pk,
                err
            );
            err
        })?;
    let dao = SpaceDao::get(&dynamo.client, space_pk.clone(), Some(EntityType::SpaceDao)).await?;
    let (remaining_count, total_count) = dao
        .map(|item| (item.remaining_count, item.total_count))
        .unwrap_or((0, 0));

    Ok(Json(ListSpaceDaoRewardResponse {
        items,
        bookmark,
        remaining_count,
        total_count,
    }))
}
