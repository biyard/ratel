use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::rewards::{SpaceReward, SpaceRewardResponse, UserReward};
use crate::models::space::SpaceCommon;
use crate::types::{EntityType, ListItemsQuery, ListItemsResponse, Pagination, SpacePublishState};
use crate::{
    AppState, Error, Permissions, Result,
    models::user::User,
    types::{Partition, TeamGroupPermission},
};

use bdk::prelude::*;
use by_axum::axum::{
    Json,
    extract::{Extension, Path, Query, State},
};

use aide::NoApi;

#[derive(
    serde::Deserialize,
    serde::Serialize,
    Debug,
    Clone,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]

pub struct ListRewardQuery {
    #[schemars(description = "Entity type to filter by e.g)POLL")]
    pub feature: Option<EntityType>,
    #[schemars(description = "Bookmark to start from")]
    pub bookmark: Option<String>,
}
pub async fn list_rewards_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    NoApi(user): NoApi<Option<User>>,

    Path(SpacePathParam { space_pk }): SpacePath,
    Query(query): Query<ListRewardQuery>,
) -> Result<Json<ListItemsResponse<SpaceRewardResponse>>> {
    permissions.permitted(TeamGroupPermission::SpaceRead)?;

    let (space_rewards, bookmark) = SpaceReward::list_by_feature(
        &dynamo.client,
        space_pk.clone().into(),
        query.feature,
        query.bookmark,
    )
    .await?;

    let user_rewards = if let Some(user) = user {
        let user_reward_keys = space_rewards
            .iter()
            .map(|reward| {
                UserReward::keys(
                    user.pk.clone().into(),
                    space_pk.clone().into(),
                    reward.sk.clone(),
                )
            })
            .collect::<Vec<_>>();
        UserReward::batch_get(&dynamo.client, user_reward_keys).await?
    } else {
        vec![]
    };

    Ok(Json(ListItemsResponse {
        items: space_rewards
            .into_iter()
            .map(|reward| {
                if let Some(user_reward) = user_rewards.iter().find(|ur| ur.sk == reward.sk) {
                    SpaceRewardResponse::from((reward, user_reward.clone()))
                } else {
                    SpaceRewardResponse::from(reward)
                }
            })
            .collect(),
        bookmark,
    }))
}
