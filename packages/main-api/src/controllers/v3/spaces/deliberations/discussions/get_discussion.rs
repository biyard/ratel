use crate::{
    AppState, Error2,
    controllers::v3::spaces::deliberations::discussions::start_meeting::DeliberationDiscussionByIdPath,
    models::{
        space::{DeliberationDiscussionResponse, DeliberationSpaceDiscussion},
        user::User,
    },
    types::{EntityType, Partition},
};
use bdk::prelude::*;
use bdk::prelude::axum::extract::{Json, Path, State};
use aide::NoApi;

pub async fn get_discussion_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
    Path(DeliberationDiscussionByIdPath {
        space_pk,
        discussion_pk,
    }): Path<DeliberationDiscussionByIdPath>,
) -> Result<Json<DeliberationDiscussionResponse>, Error2> {
    let discussion_id = match discussion_pk {
        Partition::Discussion(v) => v.to_string(),
        _ => "".to_string(),
    };

    let disc = DeliberationSpaceDiscussion::get(
        &dynamo.client,
        &space_pk,
        Some(EntityType::DeliberationSpaceDiscussion(
            discussion_id.to_string(),
        )),
    )
    .await?;

    let disc = disc.unwrap().into();

    Ok(Json(disc))
}
