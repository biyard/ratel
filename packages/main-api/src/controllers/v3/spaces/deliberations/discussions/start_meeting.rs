use crate::{
    AppState, Error2,
    models::space::{
        DeliberationDiscussionResponse, DeliberationSpaceDiscussion, DeliberationSpaceMember,
        DeliberationSpaceMemberQueryOption, DeliberationSpaceParticipant,
        DeliberationSpaceParticipantQueryOption, DiscussionMemberResponse,
        DiscussionParticipantResponse,
    },
    types::{EntityType, Partition},
    utils::aws::DynamoClient,
};
use dto::by_axum::axum::{
    Extension,
    extract::{Json, Path, State},
};

use dto::{aide, schemars};
use tower_sessions::Session;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct DeliberationDiscussionByIdPath {
    pub space_pk: String,
    pub discussion_pk: String,
}

pub async fn start_meeting_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(_session): Extension<Session>,
    Path(DeliberationDiscussionByIdPath {
        space_pk,
        discussion_pk,
    }): Path<DeliberationDiscussionByIdPath>,
) -> Result<Json<DeliberationDiscussionResponse>, Error2> {
    let space_pk = space_pk.replace("%23", "#");
    let discussion_pk = discussion_pk.replace("%23", "#");
    let client = crate::utils::aws_chime_sdk_meeting::ChimeMeetingService::new().await;
    let space_id = space_pk.split("#").last().unwrap_or_default().to_string();
    let discussion_id = discussion_pk
        .split("#")
        .last()
        .unwrap_or_default()
        .to_string();

    let disc = DeliberationSpaceDiscussion::get(
        &dynamo.client,
        &space_pk,
        Some(EntityType::DeliberationSpaceDiscussion(
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
        space_id.clone(),
        discussion_id.clone(),
        &disc,
    )
    .await;

    let disc = DeliberationSpaceDiscussion::get(
        &dynamo.client,
        &space_pk,
        Some(EntityType::DeliberationSpaceDiscussion(
            discussion_id.to_string(),
        )),
    )
    .await?;

    let disc = disc.unwrap();
    let disc_pk = match disc.sk.clone() {
        EntityType::DeliberationSpaceDiscussion(v) => v,
        _ => "".to_string(),
    };

    let opt = DeliberationSpaceMemberQueryOption::builder();
    let members = DeliberationSpaceMember::find_by_discussion_pk(
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
