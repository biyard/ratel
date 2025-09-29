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
    utils::aws::DynamoClient,
};
use aws_sdk_chimesdkmeetings::types::Meeting;
use dto::by_axum::{
    auth::Authorization,
    axum::{
        Extension,
        extract::{Json, Path, State},
    },
};

pub async fn start_recording_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(_auth): Extension<Option<Authorization>>,
    Path(DeliberationDiscussionByIdPath {
        deliberation_id,
        id,
    }): Path<DeliberationDiscussionByIdPath>,
) -> Result<Json<DeliberationDiscussionResponse>, Error2> {
    let client = crate::utils::aws_chime_sdk_meeting::ChimeMeetingService::new().await;

    let (disc_initial, _disc_pk_initial) =
        fetch_discussion_and_pk(&dynamo, &deliberation_id, &id).await?;
    let meeting_id = ensure_current_meeting(
        dynamo.clone(),
        &client,
        deliberation_id.clone(),
        id.clone(),
        &disc_initial,
    )
    .await?;

    let meeting = build_meeting_info(&client, &meeting_id, disc_initial.clone().name).await?;
    let (pipeline_id, pipeline_arn) = client
        .make_pipeline(meeting, disc_initial.name.clone())
        .await
        .map_err(|e| {
            tracing::error!("failed to create pipeline: {:?}", e);
            Error2::AwsChimeError(e.to_string())
        })?;

    DeliberationSpaceDiscussion::updater(
        &Partition::DeliberationSpace(deliberation_id.clone()),
        EntityType::DeliberationSpaceDiscussion(id.clone()),
    )
    .with_meeting_id(meeting_id.clone())
    .with_pipeline_id(pipeline_id)
    .with_media_pipeline_arn(pipeline_arn)
    .execute(&dynamo.client)
    .await?;

    let (disc_final, disc_pk) = fetch_discussion_and_pk(&dynamo, &deliberation_id, &id).await?;
    let members_resp = list_members_resp(&dynamo, &disc_pk).await?;
    let participants_resp = list_participants_resp(&dynamo, &disc_pk).await?;

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
