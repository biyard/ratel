use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::polls::*;
use crate::models::{space::SpaceCommon, user::User};
use crate::types::SpacePublishState;

use crate::utils::time::get_now_timestamp_millis;
use crate::*;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct ListPollQueryParams {
    pub bookmark: Option<String>,
}

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct ListPollsResponse {
    pub polls: Vec<PollResponse>,
    pub bookmark: Option<String>,
}

pub async fn list_polls_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Query(ListPollQueryParams { bookmark }): Query<ListPollQueryParams>,
) -> Result<Json<ListPollsResponse>> {
    // Request Validation
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundPoll);
    }

    permissions.permitted(TeamGroupPermission::SpaceRead)?;

    let mut query_options = PollQueryOption::builder()
        .sk("SPACE_POLL#".into())
        .limit(10);

    if let Some(bookmark) = bookmark {
        query_options = query_options.bookmark(bookmark);
    }

    let (responses, bookmark) =
        Poll::query(&dynamo.client, space_pk.clone(), query_options).await?;

    let mut polls = vec![];

    for response in responses {
        let poll: PollResponse = response.clone().into();

        polls.push(poll);
    }

    Ok(Json(ListPollsResponse { polls, bookmark }))
}
