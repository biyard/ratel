use crate::controllers::v3::spaces::SpaceDiscussionPath;
use crate::controllers::v3::spaces::SpaceDiscussionPathParam;
use crate::features::spaces::discussions::common_controller_logic::get_discussion;
use crate::features::spaces::discussions::dto::GetDiscussionResponse;
use crate::types::Partition;
use crate::{AppState, Error2, models::user::User};
use aide::NoApi;
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;

pub async fn get_discussion_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
    Path(SpaceDiscussionPathParam {
        space_pk,
        discussion_pk,
    }): SpaceDiscussionPath,
) -> Result<Json<GetDiscussionResponse>, Error2> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error2::NotFoundSpace);
    }

    let discussion = get_discussion(&dynamo, space_pk, discussion_pk).await?;

    Ok(Json(GetDiscussionResponse { discussion }))
}
