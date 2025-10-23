use crate::controllers::v3::spaces::{SpaceDiscussionPath, SpaceDiscussionPathParam};
use crate::features::spaces::discussions::dto::MeetingData;
use crate::features::spaces::discussions::models::space_discussion::SpaceDiscussion;
use crate::features::spaces::discussions::models::space_discussion_participant::SpaceDiscussionParticipant;
use crate::types::attendee_info::AttendeeInfo;
use crate::types::media_placement_info::MediaPlacementInfo;
use crate::types::meeting_info::MeetingInfo;
use crate::{AppState, Error2, models::user::User};
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;

use aide::NoApi;

pub async fn get_meeting_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpaceDiscussionPathParam {
        space_pk,
        discussion_pk,
    }): SpaceDiscussionPath,
) -> Result<Json<MeetingData>, Error2> {
    tracing::debug!("1111111: {:?}", discussion_pk);
    let client = crate::utils::aws_chime_sdk_meeting::ChimeMeetingService::new().await;
    let (pk, sk) = SpaceDiscussion::keys(&space_pk, &discussion_pk);

    let discussion = SpaceDiscussion::get(&dynamo.client, pk.clone(), Some(sk.clone())).await?;
    let discussion = discussion.unwrap();

    let meeting_id = discussion.meeting_id.unwrap_or_default();
    let _pipeline_arn = discussion.media_pipeline_arn.unwrap_or_default();
    let _record = discussion.record;

    let (p_pk, p_sk) = SpaceDiscussionParticipant::keys(&discussion_pk, &user.pk);

    let participant =
        SpaceDiscussionParticipant::get(&dynamo.client, p_pk.clone(), Some(p_sk.clone())).await?;

    if participant.is_none() {
        return Err(Error2::AwsChimeError("Not Found Participant".into()));
    }

    let participant = participant.unwrap();

    let attendee_id = participant.clone().participant_id;
    let user_id = participant.user_pk;

    let m = client.get_meeting_info(&meeting_id).await;

    let meeting = if m.is_some() {
        m.unwrap()
    } else {
        let v = match client.create_meeting(&discussion.name).await {
            Ok(v) => Ok(v),
            Err(e) => {
                tracing::error!("create meeting failed with error: {:?}", e);
                Err(Error2::AwsChimeError(e.to_string()))
            }
        }?;

        v
    };

    let meeting_id = meeting.clone().meeting_id.unwrap_or_default();
    let mp = meeting
        .media_placement()
        .ok_or(Error2::AwsChimeError("Missing media_placement".to_string()))?;

    let meeting_info = MeetingInfo {
        meeting_id: meeting_id.clone(),
        media_region: meeting.media_region.clone().unwrap_or_default(),
        media_placement: MediaPlacementInfo {
            audio_host_url: mp.audio_host_url().unwrap_or_default().to_string(),
            audio_fallback_url: mp.audio_fallback_url().unwrap_or_default().to_string(),
            screen_data_url: mp.screen_data_url().unwrap_or_default().to_string(),
            screen_sharing_url: mp.screen_sharing_url().unwrap_or_default().to_string(),
            screen_viewing_url: mp.screen_viewing_url().unwrap_or_default().to_string(),
            signaling_url: mp.signaling_url().unwrap_or_default().to_string(),
            turn_control_url: mp.turn_control_url().unwrap_or_default().to_string(),
        },
    };

    let v = client
        .get_attendee_info(&meeting_id, &attendee_id.clone().unwrap_or_default())
        .await;

    let attendee = if let Some(a) = v {
        a
    } else {
        let created = match client
            .create_attendee(&meeting_info, &user_id.to_string())
            .await
        {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("create attendee failed: {:?}", e);
                return Err(Error2::AwsChimeError(e.to_string()));
            }
        };

        let v = match client
            .get_attendee_info(meeting.meeting_id().unwrap(), &created.attendee_id)
            .await
        {
            Some(a) => a,
            None => {
                return Err(Error2::AwsChimeError(
                    "Failed to fetch created attendee".to_string(),
                ));
            }
        };

        SpaceDiscussion::updater(&pk, &sk)
            .with_meeting_id(meeting.meeting_id().unwrap().to_string())
            .execute(&dynamo.client)
            .await?;

        let _ = SpaceDiscussionParticipant::delete(&dynamo.client, p_pk, Some(p_sk)).await?;

        let participant = SpaceDiscussionParticipant::new(
            discussion_pk.clone(),
            v.clone().attendee_id.unwrap_or_default(),
            user.clone(),
        );
        participant.create(&dynamo.client).await?;

        v
    };

    let attendee = AttendeeInfo {
        attendee_id: attendee_id.unwrap_or_default(),
        join_token: attendee.join_token.unwrap_or_default(),
        external_user_id: attendee.external_user_id.unwrap_or_default(),
    };

    // let discussion =
    //     SpaceDiscussion::get_discussion(&dynamo.client, &space_pk, &discussion_pk, &user.pk)
    //         .await?;

    // let mut users: Vec<DiscussionUser> = vec![];
    // let discussion_participants = discussion.participants;

    // for participant in discussion_participants {
    //     let user_pk = participant.user_pk;

    //     let user = User::get(&dynamo.client, user_pk, Some(EntityType::User))
    //         .await?
    //         .unwrap_or_default();

    //     users.push(DiscussionUser {
    //         user_pk: user.pk,
    //         author_display_name: user.display_name,
    //         author_profile_url: user.profile_url,
    //         author_username: user.username,
    //     });
    // }

    Ok(Json(MeetingData {
        meeting: meeting_info,
        attendee,
        participants: vec![], //FIXME: fix to get participants
        record: None,         //FIXME: fix to get record from chime
    }))
}
