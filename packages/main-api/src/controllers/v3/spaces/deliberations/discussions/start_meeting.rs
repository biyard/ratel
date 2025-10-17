use crate::aide::NoApi;
use crate::{
    AppState, Error2,
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
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct DeliberationDiscussionByIdPath {
    #[serde(deserialize_with = "crate::types::path_param_string_to_partition")]
    pub space_pk: Partition,
    #[serde(deserialize_with = "crate::types::path_param_string_to_partition")]
    pub discussion_pk: Partition,
}

pub async fn start_meeting_handler(
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

    let disc = DeliberationSpaceDiscussion::get(
        &dynamo.client,
        &space_pk,
        Some(EntityType::DeliberationDiscussion(
            discussion_id.to_string(),
        )),
    )
    .await?;

    if disc.is_none() {
        Err(Error2::NotFound("Discussion not found".to_string()))?;
    }

    let disc = disc.unwrap();

    let _ = ensure_current_meeting(
        dynamo.clone(),
        &client,
        space_pk.clone(),
        discussion_id.clone(),
        &disc,
    )
    .await;

    let disc = DeliberationSpaceDiscussion::get(
        &dynamo.client,
        &space_pk,
        Some(EntityType::DeliberationDiscussion(
            discussion_id.to_string(),
        )),
    )
    .await?;

    let disc = disc.unwrap();
    let disc_pk = match disc.sk.clone() {
        EntityType::DeliberationDiscussion(v) => v,
        _ => "".to_string(),
    };

    let opt = DeliberationDiscussionMemberQueryOption::builder();
    let members = DeliberationDiscussionMember::find_by_discussion_pk(
        &dynamo.client,
        Partition::Discussion(disc_pk.clone()),
        opt,
    )
    .await?
    .0;
    let members_resp: Vec<DiscussionMemberResponse> = members.into_iter().map(Into::into).collect();
    let opt = DeliberationSpaceParticipantQueryOption::builder();
    let participants = DeliberationSpaceParticipant::find_by_discussion_pk(
        &dynamo.client,
        Partition::Discussion(disc_pk.clone()),
        opt,
    )
    .await?
    .0;
    let participants_resp: Vec<DiscussionParticipantResponse> = participants
        .into_iter()
        .map(Into::into)
        .filter(|p: &DiscussionParticipantResponse| !p.clone().participant_id.trim().is_empty())
        .collect();

    let mut res: DeliberationDiscussionResponse = disc.into();
    res.participants = participants_resp.clone();
    res.members = members_resp;

    Ok(Json(res))
}

async fn ensure_current_meeting(
    dynamo: DynamoClient,
    client: &crate::utils::aws_chime_sdk_meeting::ChimeMeetingService,
    space_pk: Partition,
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
        &space_pk.clone(),
        EntityType::DeliberationDiscussion(discussion_id.to_string()),
    )
    .with_meeting_id(new_id.clone())
    .execute(&dynamo.client)
    .await?;

    Ok(new_id)
}
