use crate::controllers::v3::spaces::deliberations::discussions::start_meeting::DeliberationDiscussionByIdPath;
use crate::models::{
    DeliberationSpaceDiscussion, DeliberationSpaceParticipant,
    DeliberationSpaceParticipantQueryOption, User,
};
use crate::types::attendee_info::AttendeeInfo;
use crate::types::media_placement_info::MediaPlacementInfo;
use crate::types::meeting_info::MeetingInfo;
use crate::types::{EntityType, Partition};
use crate::{AppState, Error2};
use aide::NoApi;
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;

#[derive(
    Debug,
    Clone,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct DiscussionUser {
    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,
}

#[derive(
    Debug,
    Clone,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    aide::OperationIo,
)]
#[serde(rename_all = "PascalCase")]
pub struct MeetingData {
    pub meeting: MeetingInfo,
    pub attendee: AttendeeInfo,
    pub participants: Vec<DiscussionUser>,
    pub record: Option<String>,
}

pub async fn get_meeting_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(DeliberationDiscussionByIdPath {
        space_pk,
        discussion_pk,
    }): Path<DeliberationDiscussionByIdPath>,
) -> Result<Json<MeetingData>, Error2> {
    let user = user.unwrap_or_default();

    let discussion_id = match discussion_pk {
        Partition::Discussion(v) => v.to_string(),
        _ => "".to_string(),
    };

    let user_id = match user.pk.clone() {
        Partition::User(v) => v,
        _ => String::new(),
    };

    let client = crate::utils::aws_chime_sdk_meeting::ChimeMeetingService::new().await;

    let discussion = DeliberationSpaceDiscussion::get(
        &dynamo.client,
        &space_pk,
        Some(EntityType::DeliberationSpaceDiscussion(
            discussion_id.to_string(),
        )),
    )
    .await?
    .unwrap_or_default();

    let meeting_id = discussion.meeting_id.unwrap_or_default();
    let _pipeline_arn = discussion.media_pipeline_arn.unwrap_or_default();
    let _record = discussion.record;

    let opt = DeliberationSpaceParticipantQueryOption::builder();

    let participants = DeliberationSpaceParticipant::find_by_discussion_user_pk(
        &dynamo.client,
        Partition::DiscussionUser(format!("{}#{}", discussion_id, user_id)),
        opt,
    )
    .await?
    .0;

    if participants.is_empty() {
        return Err(Error2::NotFound("Not found user".into()));
    }

    let attendee_id = participants[0].clone().participant_id;
    let user_id = participants[0].clone().user_pk;

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

        DeliberationSpaceDiscussion::updater(
            &space_pk,
            EntityType::DeliberationSpaceDiscussion(discussion_id.clone()),
        )
        .with_meeting_id(meeting.meeting_id().unwrap().to_string())
        .execute(&dynamo.client)
        .await?;

        for p in participants.clone() {
            DeliberationSpaceParticipant::delete(&dynamo.client, p.pk, Some(p.sk)).await?;
        }

        let participant = DeliberationSpaceParticipant::new(
            space_pk,
            Partition::Discussion(discussion_id.to_string()),
            v.clone().attendee_id.unwrap_or_default(),
            user,
        );
        participant.create(&dynamo.client).await?;

        v
    };

    let attendee = AttendeeInfo {
        attendee_id: attendee_id.unwrap_or_default(),
        join_token: attendee.join_token.unwrap_or_default(),
        external_user_id: attendee.external_user_id.unwrap_or_default(),
    };

    let opt = DeliberationSpaceParticipantQueryOption::builder();

    let discussion_participants = DeliberationSpaceParticipant::find_by_discussion_pk(
        &dynamo.client,
        Partition::Discussion(format!("{}", discussion_id)),
        opt,
    )
    .await?
    .0;

    let mut users: Vec<DiscussionUser> = vec![];

    for participant in discussion_participants {
        let user_pk = participant.user_pk;

        let user = User::get(&dynamo.client, user_pk, Some(EntityType::User))
            .await?
            .unwrap_or_default();

        users.push(DiscussionUser {
            user_pk: user.pk,
            author_display_name: user.display_name,
            author_profile_url: user.profile_url,
            author_username: user.username,
        });
    }

    Ok(Json(MeetingData {
        meeting: meeting_info,
        attendee,
        participants: users,
        record: None, //FIXME: fix to get record from chime
    }))
}
