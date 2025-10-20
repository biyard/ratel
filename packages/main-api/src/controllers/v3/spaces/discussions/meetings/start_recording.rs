use crate::controllers::v3::spaces::{SpaceDiscussionPath, SpaceDiscussionPathParam};
use crate::features::common_controller_logic::ensure_current_meeting;
use crate::features::common_controller_logic::get_discussion;
use crate::features::dto::SpaceDiscussionResponse;
use crate::features::models::space_discussion::SpaceDiscussion;

use crate::models::User;
use crate::types::{EntityType, Partition};
use crate::{AppState, Error2};
use aws_sdk_chimesdkmeetings::types::Meeting;
use axum::extract::{Path, State};
use bdk::prelude::aide::NoApi;
use bdk::prelude::axum::Json;
use bdk::prelude::*;

pub async fn start_recording_handler(
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
    let disc_initial = get_discussion(&dynamo, space_pk.clone(), discussion_pk.clone()).await?;
    let disc = SpaceDiscussion::get(
        &dynamo.client,
        space_pk.clone(),
        Some(EntityType::SpaceDiscussion(discussion_id.to_string())),
    )
    .await?;
    let disc = disc.unwrap();

    let meeting_id = ensure_current_meeting(
        dynamo.clone(),
        &client,
        space_pk.clone(),
        discussion_id.clone(),
        &disc,
    )
    .await?;

    let meeting = build_meeting_info(&client, &meeting_id, disc.clone().name).await?;
    let (pipeline_id, pipeline_arn) = client
        .make_pipeline(meeting, disc_initial.name.clone())
        .await
        .map_err(|e| {
            tracing::error!("failed to create pipeline: {:?}", e);
            Error2::AwsChimeError(e.to_string())
        })?;

    SpaceDiscussion::updater(
        &space_pk.clone(),
        EntityType::SpaceDiscussion(discussion_id.clone()),
    )
    .with_meeting_id(meeting_id.clone())
    .with_pipeline_id(pipeline_id)
    .with_media_pipeline_arn(pipeline_arn)
    .execute(&dynamo.client)
    .await?;

    let disc = get_discussion(&dynamo, space_pk.clone(), discussion_pk.clone()).await?;
    Ok(Json(disc))
}

async fn build_meeting_info(
    client: &crate::utils::aws_chime_sdk_meeting::ChimeMeetingService,
    meeting_id: &str,
    discussion_name: String,
) -> Result<Meeting, Error2> {
    let m = client.get_meeting_info(&meeting_id).await;

    let meeting = if m.is_some() {
        m.unwrap()
    } else {
        let v = match client.create_meeting(&discussion_name).await {
            Ok(v) => Ok(v),
            Err(e) => {
                tracing::error!("create meeting failed with error: {:?}", e);
                Err(Error2::AwsChimeError(e.to_string()))
            }
        }?;

        v
    };

    return Ok(meeting);
}
