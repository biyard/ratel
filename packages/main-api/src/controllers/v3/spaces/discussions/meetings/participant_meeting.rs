use crate::controllers::v3::spaces::{SpaceDiscussionPath, SpaceDiscussionPathParam};
use crate::features::common_controller_logic::{
    build_meeting_info, ensure_current_meeting, get_discussion,
};
use crate::features::dto::SpaceDiscussionResponse;
use crate::features::models::space_discussion::SpaceDiscussion;
use crate::features::models::space_discussion_participant::{
    SpaceDiscussionParticipant, SpaceDiscussionParticipantQueryOption,
};
use crate::types::{EntityType, Partition};
use crate::{AppState, Error2, models::user::User};
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;

use aide::NoApi;

pub async fn participant_meeting_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
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

    let user = user.unwrap_or_default();
    let user_pk = match user.pk.clone() {
        Partition::User(v) => v,
        _ => String::new(),
    };

    let disc = get_discussion(&dynamo, space_pk.clone(), discussion_pk.clone()).await?;
    let participants_resp = disc.clone().participants;

    if participants_resp.iter().any(|p| p.user_pk == user.pk) {
        return Ok(Json(disc));
    }

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

    let meeting = build_meeting_info(&client, &meeting_id).await?;
    let create_attendee_once = |_mid: &str| async {
        client
            .create_attendee(&meeting, user_pk.as_str())
            .await
            .map_err(|e| {
                tracing::error!("create_attendee error: {:?}", e);
                Error2::AwsChimeError(e.to_string())
            })
    };

    let attendee_res = match create_attendee_once(&meeting_id).await {
        Ok(r) => r,
        Err(e) => {
            let msg = e.to_string();
            let not_found = msg.contains("NotFound")
                || msg.contains("NotFoundException")
                || msg.to_ascii_lowercase().contains("not found");
            if not_found {
                let recreated_id = ensure_current_meeting(
                    dynamo.clone(),
                    &client,
                    space_pk.clone(),
                    discussion_id.clone(),
                    &disc,
                )
                .await?;
                let meeting2 = build_meeting_info(&client, &recreated_id).await?;
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

    let olds = SpaceDiscussionParticipant::find_by_user_pk(
        &dynamo.client,
        user.pk.clone(),
        SpaceDiscussionParticipantQueryOption::builder(),
    )
    .await?
    .0;

    let mut tx = vec![];

    for p in olds {
        let d = SpaceDiscussionParticipant::updater(p.pk, p.sk)
            .with_participant_id("".to_string())
            .transact_write_item();

        tx.push(d);

        if tx.len() == 10 {
            dynamo
                .client
                .transact_write_items()
                .set_transact_items(Some(tx.clone()))
                .send()
                .await
                .map_err(|e| {
                    tracing::error!("Failed to update discussion participants: {:?}", e);
                    Error2::InternalServerError("Failed to update discussion participants".into())
                })?;

            tx.clear();
        }
    }

    if !tx.is_empty() {
        dynamo
            .client
            .transact_write_items()
            .set_transact_items(Some(tx.clone()))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to update discussion participants: {:?}", e);
                Error2::InternalServerError("Failed to update discussion participants".into())
            })?;

        tx.clear();
    }

    let participant =
        SpaceDiscussionParticipant::new(discussion_pk.clone(), attendee_res.attendee_id, user);
    participant.create(&dynamo.client).await?;

    let disc = get_discussion(&dynamo, space_pk.clone(), discussion_pk.clone()).await?;
    Ok(Json(disc))
}
