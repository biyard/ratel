use crate::{
    AppState, Error2,
    controllers::v3::spaces::deliberations::discussions::start_meeting::DeliberationDiscussionByIdPath,
    models::{
        DeliberationDiscussionMember, DeliberationDiscussionMemberQueryOption,
        DeliberationSpaceParticipant, DeliberationSpaceParticipantQueryOption,
        DiscussionMemberResponse, DiscussionParticipantResponse,
        space::{DeliberationDiscussionResponse, DeliberationSpaceDiscussion},
        user::User,
    },
    types::{EntityType, Partition},
    utils::aws::DynamoClient,
};
use aide::NoApi;
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;

pub async fn get_discussion_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
    Path(DeliberationDiscussionByIdPath {
        space_pk,
        discussion_pk,
    }): Path<DeliberationDiscussionByIdPath>,
) -> Result<Json<DeliberationDiscussionResponse>, Error2> {
    let discussion_id = match discussion_pk {
        Partition::Discussion(v) => v.to_string(),
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

    let mut disc: DeliberationDiscussionResponse = disc.unwrap().into();

    let members_resp = list_members_resp(&dynamo, disc.pk.clone()).await?;
    let participants_resp = list_participants_resp(&dynamo, disc.pk.clone()).await?;
    disc.members = members_resp;
    disc.participants = participants_resp;

    Ok(Json(disc))
}

async fn list_members_resp(
    dynamo: &DynamoClient,
    disc_pk: Partition,
) -> Result<Vec<DiscussionMemberResponse>, Error2> {
    let opt = DeliberationDiscussionMemberQueryOption::builder();
    let members = DeliberationDiscussionMember::find_by_discussion_pk(&dynamo.client, disc_pk, opt)
        .await?
        .0;
    Ok(members.into_iter().map(Into::into).collect())
}

async fn list_participants_resp(
    dynamo: &DynamoClient,
    disc_pk: Partition,
) -> Result<Vec<DiscussionParticipantResponse>, Error2> {
    let opt = DeliberationSpaceParticipantQueryOption::builder();
    let participants =
        DeliberationSpaceParticipant::find_by_discussion_pk(&dynamo.client, disc_pk, opt)
            .await?
            .0;
    Ok(participants
        .into_iter()
        .map(Into::into)
        .filter(|p: &DiscussionParticipantResponse| !p.participant_id.trim().is_empty())
        .collect())
}
