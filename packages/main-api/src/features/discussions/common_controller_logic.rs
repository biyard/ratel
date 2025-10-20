use crate::features::dto::{SpaceDiscussionMemberResponse, SpaceDiscussionParticipantResponse};
use crate::features::models::space_discussion_member::SpaceDiscussionMember;
use crate::features::models::space_discussion_member::SpaceDiscussionMemberQueryOption;
use crate::features::models::space_discussion_participant::SpaceDiscussionParticipant;
use crate::features::models::space_discussion_participant::SpaceDiscussionParticipantQueryOption;
use crate::types::EntityType;
use crate::{
    Error2,
    features::{dto::SpaceDiscussionResponse, models::space_discussion::SpaceDiscussion},
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
                _ => {}
            }
        }

        match new_bookmark {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    discussion.members = discussion_members;
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
                    discussion_participants.push(response.into());
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
