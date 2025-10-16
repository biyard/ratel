use crate::{
    AppState, Error2,
    controllers::v3::spaces::deliberations::discussions::start_meeting::DeliberationDiscussionByIdPath,
    models::{
        User,
        space::{
            DeliberationDiscussionMember, DeliberationDiscussionMemberQueryOption,
            DeliberationDiscussionResponse, DeliberationSpaceDiscussion,
            DeliberationSpaceParticipant, DeliberationSpaceParticipantQueryOption,
            DiscussionMemberResponse, DiscussionParticipantResponse,
        },
    },
    types::{EntityType, Partition},
    utils::aws::DynamoClient,
};
use bdk::prelude::*;
use bdk::prelude::{
    aide::NoApi,
    axum::extract::{Json, Path, State},
};

pub async fn end_recording_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
    Path(DeliberationDiscussionByIdPath {
        space_pk,
        discussion_pk,
    }): Path<DeliberationDiscussionByIdPath>,
) -> Result<Json<DeliberationDiscussionResponse>, Error2> {
    let client = crate::utils::aws_chime_sdk_meeting::ChimeMeetingService::new().await;
    let discussion_id = match discussion_pk {
        Partition::Discussion(v) => v,
        _ => "".to_string(),
    };

    let (disc, _disc_pk) =
        fetch_discussion_and_pk(&dynamo, space_pk.clone(), &discussion_id).await?;

    let meeting_id = disc
        .meeting_id
        .clone()
        .ok_or_else(|| Error2::NotFound("not found discussion".to_string()))?;

    if disc.pipeline_id == "" {
        return Err(Error2::NotFound("not found pipeline".to_string()));
    }

    client
        .end_pipeline(&disc.pipeline_id, meeting_id.as_str())
        .await
        .map_err(|e| {
            tracing::error!("end_pipeline failed: {:?}", e);
            Error2::AwsChimeError(e.to_string())
        })?;

    DeliberationSpaceDiscussion::updater(
        space_pk.clone(),
        EntityType::DeliberationDiscussion(discussion_id.clone()),
    )
    .with_pipeline_id(String::new())
    .with_media_pipeline_arn(String::new())
    .execute(&dynamo.client)
    .await?;

    let (disc_final, disc_pk) =
        fetch_discussion_and_pk(&dynamo, space_pk.clone(), &discussion_id).await?;
    let members_resp = list_members_resp(&dynamo, &disc_pk).await?;
    let participants_resp = list_participants_resp(&dynamo, &disc_pk).await?;

    let mut res: DeliberationDiscussionResponse = disc_final.into();
    res.members = members_resp;
    res.participants = participants_resp;
    Ok(Json(res))
}

async fn fetch_discussion_and_pk(
    dynamo: &DynamoClient,
    space_pk: Partition,
    discussion_id: &str,
) -> Result<(DeliberationSpaceDiscussion, String), Error2> {
    let disc = DeliberationSpaceDiscussion::get(
        &dynamo.client,
        &space_pk,
        Some(EntityType::DeliberationDiscussion(
            discussion_id.to_string(),
        )),
    )
    .await?
    .ok_or_else(|| Error2::NotFound("Discussion not found".into()))?;
    let disc_pk = match disc.sk.clone() {
        EntityType::DeliberationDiscussion(v) => v,
        _ => String::new(),
    };
    Ok((disc, disc_pk))
}

async fn list_members_resp(
    dynamo: &DynamoClient,
    disc_pk: &str,
) -> Result<Vec<DiscussionMemberResponse>, Error2> {
    let opt = DeliberationDiscussionMemberQueryOption::builder();
    let members = DeliberationDiscussionMember::find_by_discussion_pk(
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
