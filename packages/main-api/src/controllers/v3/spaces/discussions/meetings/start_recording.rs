use crate::controllers::v3::spaces::{SpaceDiscussionPath, SpaceDiscussionPathParam};
use crate::features::spaces::discussions::dto::SpaceDiscussionResponse;
use crate::features::spaces::discussions::models::space_discussion::SpaceDiscussion;

use crate::models::User;
use crate::{AppState, Error2};
use aws_sdk_chimesdkmeetings::types::Meeting;
use axum::extract::{Path, State};
use bdk::prelude::aide::NoApi;
use bdk::prelude::axum::Json;
use bdk::prelude::*;

pub async fn start_recording_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpaceDiscussionPathParam {
        space_pk,
        discussion_pk,
    }): SpaceDiscussionPath,
) -> Result<Json<SpaceDiscussionResponse>, Error2> {
    let client = crate::utils::aws_chime_sdk_meeting::ChimeMeetingService::new().await;

    let (pk, sk) = SpaceDiscussion::keys(&space_pk, &discussion_pk);

    let disc = SpaceDiscussion::get(&dynamo.client, pk.clone(), Some(sk.clone())).await?;
    let disc = disc.unwrap();

    let meeting_id = client
        .ensure_current_meeting(
            dynamo.clone(),
            &client,
            space_pk.clone(),
            discussion_pk.clone(),
            &disc,
        )
        .await?;

    let meeting = build_meeting(&client, &meeting_id, disc.clone().name).await?;
    let (pipeline_id, pipeline_arn) = client
        .make_pipeline(meeting, disc.name.clone())
        .await
        .map_err(|e| {
            tracing::error!("failed to create pipeline: {:?}", e);
            Error2::AwsChimeError(e.to_string())
        })?;

    SpaceDiscussion::updater(pk.clone(), sk.clone())
        .with_meeting_id(meeting_id.clone())
        .with_pipeline_id(pipeline_id)
        .with_media_pipeline_arn(pipeline_arn)
        .execute(&dynamo.client)
        .await?;

    let discussion =
        SpaceDiscussion::get_discussion(&dynamo.client, &space_pk, &discussion_pk, &user.pk)
            .await?;
    Ok(Json(discussion))
}

async fn build_meeting(
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
