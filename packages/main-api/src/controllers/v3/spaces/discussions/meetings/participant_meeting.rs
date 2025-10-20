use crate::controllers::v3::spaces::{SpaceDiscussionPath, SpaceDiscussionPathParam};
use crate::features::spaces::discussions::dto::SpaceDiscussionResponse;
use crate::features::spaces::discussions::models::space_discussion::SpaceDiscussion;
use crate::features::spaces::discussions::models::space_discussion_participant::SpaceDiscussionParticipant;
use crate::types::Partition;
use crate::{AppState, Error2, models::user::User};
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;

use aide::NoApi;

pub async fn participant_meeting_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpaceDiscussionPathParam {
        space_pk,
        discussion_pk,
    }): SpaceDiscussionPath,
) -> Result<Json<SpaceDiscussionResponse>, Error2> {
    let client = crate::utils::aws_chime_sdk_meeting::ChimeMeetingService::new().await;
    let user_pk = match user.pk.clone() {
        Partition::User(v) => v,
        _ => String::new(),
    };

    let is_participant =
        SpaceDiscussionParticipant::is_participant(&dynamo.client, &discussion_pk, &user.pk)
            .await?;

    if is_participant {
        let disc =
            SpaceDiscussion::get_discussion(&dynamo.client, &space_pk, &discussion_pk, &user.pk)
                .await?;
        return Ok(Json(disc));
    }

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

    let meeting = client.build_meeting_info(&client, &meeting_id).await?;

    let attendee_res = match client
        .create_attendee(&meeting, user_pk.as_str())
        .await
        .map_err(|e| {
            tracing::error!("create_attendee error: {:?}", e);
            Error2::AwsChimeError(e.to_string())
        }) {
        Ok(r) => r,
        Err(e) => {
            let msg = e.to_string();
            let not_found = msg.contains("NotFound")
                || msg.contains("NotFoundException")
                || msg.to_ascii_lowercase().contains("not found");
            if not_found {
                let recreated_id = client
                    .ensure_current_meeting(
                        dynamo.clone(),
                        &client,
                        space_pk.clone(),
                        discussion_pk.clone(),
                        &disc,
                    )
                    .await?;
                let meeting2 = client.build_meeting_info(&client, &recreated_id).await?;
                client
                    .create_attendee(&meeting2, user_pk.as_str())
                    .await
                    .map_err(|e| {
                        tracing::error!("create_attendee error: {:?}", e);
                        Error2::AwsChimeError(e.to_string())
                    })?
            } else {
                return Err(e);
            }
        }
    };

    let participant = SpaceDiscussionParticipant::new(
        discussion_pk.clone(),
        attendee_res.attendee_id,
        user.clone(),
    );
    participant.create(&dynamo.client).await?;

    let disc = SpaceDiscussion::get_discussion(&dynamo.client, &space_pk, &discussion_pk, &user.pk)
        .await?;
    Ok(Json(disc))
}
