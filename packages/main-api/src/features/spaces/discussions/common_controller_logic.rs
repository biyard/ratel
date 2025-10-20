use crate::features::spaces::discussions::dto::{
    SpaceDiscussionMemberResponse, SpaceDiscussionParticipantResponse,
};
use crate::features::spaces::discussions::models::space_discussion_member::SpaceDiscussionMember;
use crate::features::spaces::discussions::models::space_discussion_member::SpaceDiscussionMemberQueryOption;
use crate::features::spaces::discussions::models::space_discussion_participant::SpaceDiscussionParticipant;
use crate::features::spaces::discussions::models::space_discussion_participant::SpaceDiscussionParticipantQueryOption;
use crate::types::EntityType;
use crate::types::media_placement_info::MediaPlacementInfo;
use crate::types::meeting_info::MeetingInfo;
use crate::{
    Error2,
    features::spaces::discussions::{
        dto::SpaceDiscussionResponse, models::space_discussion::SpaceDiscussion,
    },
    types::Partition,
    utils::aws::DynamoClient,
};

pub async fn get_discussion(
    dynamo: &DynamoClient,
    space_pk: Partition,
    discussion_pk: Partition,
) -> Result<SpaceDiscussionResponse, Error2> {
    let discussion_id = match discussion_pk {
        Partition::Discussion(v) => v.to_string(),
        _ => "".to_string(),
    };

    let discussion = SpaceDiscussion::get(
        &dynamo.client,
        space_pk.clone(),
        Some(EntityType::SpaceDiscussion(discussion_id.to_string())),
    )
    .await?;

    if discussion.is_none() {
        return Err(Error2::NotFoundDiscussion);
    }

    let discussion = discussion.unwrap();

    let mut discussion: SpaceDiscussionResponse = discussion.into();

    let mut discussion_members: Vec<SpaceDiscussionMemberResponse> = vec![];
    let mut discussion_participants: Vec<SpaceDiscussionParticipantResponse> = vec![];
    let mut bookmark = None::<String>;

    loop {
        let (responses, new_bookmark) = SpaceDiscussionMember::query(
            &dynamo.client,
            discussion.pk.clone(),
            if let Some(b) = &bookmark {
                SpaceDiscussionMemberQueryOption::builder().bookmark(b.clone())
            } else {
                SpaceDiscussionMemberQueryOption::builder()
            },
        )
        .await?;

        for response in responses {
            match response.sk {
                EntityType::SpaceDiscussionMember(_) => {
                    discussion_members.push(response.into());
                }
                EntityType::SpaceDiscussionParticipant(_) => {
                    discussion_members.push(response.into());
                }
                _ => {}
            }
        }

        match new_bookmark {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    discussion.members = discussion_members.clone();
    bookmark = None;

    loop {
        let (responses, new_bookmark) = SpaceDiscussionParticipant::query(
            &dynamo.client,
            discussion.pk.clone(),
            if let Some(b) = &bookmark {
                SpaceDiscussionParticipantQueryOption::builder().bookmark(b.clone())
            } else {
                SpaceDiscussionParticipantQueryOption::builder()
            },
        )
        .await?;

        for response in responses {
            match response.sk {
                EntityType::SpaceDiscussionParticipant(_) => {
                    if response.participant_id.clone().is_some()
                        && !response.participant_id.clone().unwrap().is_empty()
                    {
                        discussion_participants.push(response.into());
                    } else {
                        let response = SpaceDiscussionMember {
                            pk: response.pk,
                            sk: response.sk,
                            user_pk: response.user_pk,
                            author_display_name: response.author_display_name,
                            author_profile_url: response.author_profile_url,
                            author_username: response.author_username,
                        };

                        discussion_members.push(response.into());
                    }
                }
                _ => {}
            }
        }

        match new_bookmark {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    discussion.participants = discussion_participants;

    Ok(discussion)
}

pub async fn ensure_current_meeting(
    dynamo: DynamoClient,
    client: &crate::utils::aws_chime_sdk_meeting::ChimeMeetingService,
    space_pk: Partition,
    discussion_id: String,
    discussion: &SpaceDiscussion,
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

    SpaceDiscussion::updater(
        &space_pk.clone(),
        EntityType::SpaceDiscussion(discussion_id.to_string()),
    )
    .with_meeting_id(new_id.clone())
    .execute(&dynamo.client)
    .await?;

    Ok(new_id)
}

pub async fn build_meeting_info(
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
