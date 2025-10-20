use crate::controllers::v3::spaces::{SpaceDiscussionPath, SpaceDiscussionPathParam};
use crate::features::spaces::discussions::common_controller_logic::get_discussion;
use crate::features::spaces::discussions::dto::space_discussion_response::SpaceDiscussionResponse;
use crate::features::spaces::discussions::models::space_discussion::SpaceDiscussion;

use crate::models::User;
use crate::types::{EntityType, Partition};
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
    let discussion_id = match discussion_pk.clone() {
        Partition::Discussion(v) => v,
        _ => "".to_string(),
    };

    let disc = SpaceDiscussion::get(
        &dynamo.client,
        space_pk.clone(),
        Some(EntityType::SpaceDiscussion(discussion_id.to_string())),
    )
    .await?;
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

    let disc = get_discussion(&dynamo, space_pk.clone(), discussion_pk.clone()).await?;

    Ok(Json(disc))
}
