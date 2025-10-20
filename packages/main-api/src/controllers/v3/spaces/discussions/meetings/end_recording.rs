use crate::controllers::v3::spaces::{SpaceDiscussionPath, SpaceDiscussionPathParam};
use crate::features::spaces::discussions::dto::space_discussion_response::SpaceDiscussionResponse;
use crate::features::spaces::discussions::models::space_discussion::SpaceDiscussion;

use crate::models::User;
use crate::{AppState, Error2};
use axum::extract::{Path, State};
use bdk::prelude::aide::NoApi;
use bdk::prelude::axum::Json;
use bdk::prelude::*;

pub async fn end_recording_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
    Path(SpaceDiscussionPathParam {
        space_pk,
        discussion_pk,
    }): SpaceDiscussionPath,
) -> Result<Json<SpaceDiscussionResponse>, Error2> {
    let client = crate::utils::aws_chime_sdk_meeting::ChimeMeetingService::new().await;

    let (pk, sk) = SpaceDiscussion::keys(&space_pk, &discussion_pk);

    let disc = SpaceDiscussion::get(&dynamo.client, pk.clone(), Some(sk.clone())).await?;
    let disc = disc.unwrap();

    let meeting_id = disc
        .meeting_id
        .clone()
        .ok_or_else(|| Error2::NotFound("not found discussion".to_string()))?;

    if disc.pipeline_id == "" {
        return Err(Error2::NotFound("not found pipeline".to_string()));
    }

    client
        .end_pipeline(&disc.pipeline_id, meeting_id.as_str())
        .await
        .map_err(|e| {
            tracing::error!("end_pipeline failed: {:?}", e);
            Error2::AwsChimeError(e.to_string())
        })?;

    let disc = disc.into();

    Ok(Json(disc))
}
