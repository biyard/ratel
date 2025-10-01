use crate::{
    AppState, Error2,
    controllers::v3::spaces::deliberations::discussions::start_meeting::DeliberationDiscussionByIdPath,
    models::space::{
        DeliberationDiscussionResponse, DeliberationSpaceDiscussion, DeliberationSpaceMember,
        DeliberationSpaceMemberQueryOption, DeliberationSpaceParticipant,
        DeliberationSpaceParticipantQueryOption, DiscussionMemberResponse,
        DiscussionParticipantResponse,
    },
    types::{EntityType, Partition},
    utils::{aws::DynamoClient, dynamo_extractor::extract_user_from_session},
};
use bdk::prelude::axum::{
    Extension,
    extract::{Json, Path, State},
};
use bdk::prelude::*;
use dto::{MediaPlacementInfo, MeetingInfo};
use tower_sessions::Session;

pub async fn participant_meeting_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<Session>,
    Path(DeliberationDiscussionByIdPath {
        space_pk,
        discussion_pk,
    }): Path<DeliberationDiscussionByIdPath>,
) -> Result<Json<DeliberationDiscussionResponse>, Error2> {
    let client = crate::utils::aws_chime_sdk_meeting::ChimeMeetingService::new().await;
    let space_id = match space_pk.clone() {
        Partition::DeliberationSpace(v) => v,
        _ => "".to_string(),
    };
    let discussion_id = match discussion_pk {
        Partition::Discussion(v) => v,
        _ => "".to_string(),
    };

    let user = extract_user_from_session(&dynamo.client, &session).await?;
    let user_pk = match user.pk.clone() {
        Partition::User(v) => v,
        _ => String::new(),
    };

    let (disc_initial, disc_pk) =
        fetch_discussion_and_pk(&dynamo, &space_id, &discussion_id).await?;
    let members_resp = list_members_resp(&dynamo, &disc_pk).await?;
    let mut participants_resp = list_participants_resp(&dynamo, &disc_pk).await?;

    if participants_resp.iter().any(|p| p.user_pk == user.pk) {
        let mut res: DeliberationDiscussionResponse = disc_initial.into();
        res.members = members_resp;
        res.participants = participants_resp;
        return Ok(Json(res));
    }

    let meeting_id = ensure_current_meeting(
        dynamo.clone(),
        &client,
        space_id.clone(),
        discussion_id.clone(),
        &disc_initial,
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
                    space_id.clone(),
                    discussion_id.clone(),
                    &disc_initial,
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

    let opt = DeliberationSpaceParticipantQueryOption::builder();
    let olds = DeliberationSpaceParticipant::find_by_discussion_user_pk(
        &dynamo.client,
        Partition::DiscussionUser(format!("{}#{}", discussion_id, user_pk)),
        opt,
    )
    .await?
    .0;
    for p in olds {
        DeliberationSpaceParticipant::delete(&dynamo.client, p.pk, Some(p.sk)).await?;
    }

    let participant = DeliberationSpaceParticipant::new(
        Partition::DeliberationSpace(space_id.to_string()),
        Partition::Discussion(discussion_id.to_string()),
        attendee_res.attendee_id,
        user,
    );
    participant.create(&dynamo.client).await?;

    let (disc_final, disc_pk) = fetch_discussion_and_pk(&dynamo, &space_id, &discussion_id).await?;
    let members_resp = list_members_resp(&dynamo, &disc_pk).await?;
    participants_resp = list_participants_resp(&dynamo, &disc_pk).await?;

    let mut res: DeliberationDiscussionResponse = disc_final.into();
    res.members = members_resp;
    res.participants = participants_resp;
    Ok(Json(res))
}

async fn fetch_discussion_and_pk(
    dynamo: &DynamoClient,
    deliberation_id: &str,
    discussion_id: &str,
) -> Result<(DeliberationSpaceDiscussion, String), Error2> {
    let disc = DeliberationSpaceDiscussion::get(
        &dynamo.client,
        &Partition::DeliberationSpace(deliberation_id.to_string()),
        Some(EntityType::DeliberationSpaceDiscussion(
            discussion_id.to_string(),
        )),
    )
    .await?
    .ok_or_else(|| Error2::NotFound("Discussion not found".into()))?;
    let disc_pk = match disc.sk.clone() {
        EntityType::DeliberationSpaceDiscussion(v) => v,
        _ => String::new(),
    };
    Ok((disc, disc_pk))
}

async fn list_members_resp(
    dynamo: &DynamoClient,
    disc_pk: &str,
) -> Result<Vec<DiscussionMemberResponse>, Error2> {
    let opt = DeliberationSpaceMemberQueryOption::builder();
    let members = DeliberationSpaceMember::find_by_discussion_pk(
        &dynamo.client,
        Partition::Discussion(disc_pk.to_string()),
        opt,
    )
    .await?
    .0;
    Ok(members.into_iter().map(Into::into).collect())
}

async fn list_participants_resp(
    dynamo: &DynamoClient,
    disc_pk: &str,
) -> Result<Vec<DiscussionParticipantResponse>, Error2> {
    let opt = DeliberationSpaceParticipantQueryOption::builder();
    let participants = DeliberationSpaceParticipant::find_by_discussion_pk(
        &dynamo.client,
        Partition::Discussion(disc_pk.to_string()),
        opt,
    )
    .await?
    .0;
    Ok(participants
        .into_iter()
        .map(Into::into)
        .filter(|p: &DiscussionParticipantResponse| !p.participant_id.trim().is_empty())
        .collect())
}

async fn ensure_current_meeting(
    dynamo: DynamoClient,
    client: &crate::utils::aws_chime_sdk_meeting::ChimeMeetingService,
    deliberation_id: String,
    discussion_id: String,
    discussion: &DeliberationSpaceDiscussion,
) -> Result<String, Error2> {
    if let Some(ref mid) = discussion.meeting_id {
        if client.get_meeting_info(mid).await.is_some() {
            return Ok(mid.clone());
        }
    }
    let created = client.create_meeting(&discussion.name).await.map_err(|e| {
        tracing::error!("create_meeting failed: {:?}", e);
        Error2::AwsChimeError(e.to_string())
    })?;
    let new_id = created.meeting_id().unwrap_or_default().to_string();
    DeliberationSpaceDiscussion::updater(
        &Partition::DeliberationSpace(deliberation_id.to_string()),
        EntityType::DeliberationSpaceDiscussion(discussion_id.to_string()),
    )
    .with_meeting_id(new_id.clone())
    .execute(&dynamo.client)
    .await?;
    Ok(new_id)
}

async fn build_meeting_info(
    client: &crate::utils::aws_chime_sdk_meeting::ChimeMeetingService,
    meeting_id: &str,
) -> Result<MeetingInfo, Error2> {
    let m = client
        .get_meeting_info(meeting_id)
        .await
        .ok_or_else(|| Error2::AwsChimeError("Missing meeting from Chime".into()))?;
    let mp = m
        .media_placement()
        .ok_or_else(|| Error2::AwsChimeError("Missing media_placement".into()))?;
    Ok(MeetingInfo {
        meeting_id: meeting_id.to_string(),
        media_region: m.media_region.clone().unwrap_or_default(),
        media_placement: MediaPlacementInfo {
            audio_host_url: mp.audio_host_url().unwrap_or_default().to_string(),
            audio_fallback_url: mp.audio_fallback_url().unwrap_or_default().to_string(),
            screen_data_url: mp.screen_data_url().unwrap_or_default().to_string(),
            screen_sharing_url: mp.screen_sharing_url().unwrap_or_default().to_string(),
            screen_viewing_url: mp.screen_viewing_url().unwrap_or_default().to_string(),
            signaling_url: mp.signaling_url().unwrap_or_default().to_string(),
            turn_control_url: mp.turn_control_url().unwrap_or_default().to_string(),
        },
    })
}
