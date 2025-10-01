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
use dto::by_axum::axum::{
    Extension,
    extract::{Json, Path, State},
};
use tower_sessions::Session;
use urlencoding::decode;

const SPACE_PREFIX: &str = "DELIBERATION_SPACE#";
const DISCUSSION_PREFIX: &str = "DISCUSSION#";

pub async fn exit_meeting_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<Session>,
    Path(DeliberationDiscussionByIdPath {
        space_pk,
        discussion_pk,
    }): Path<DeliberationDiscussionByIdPath>,
) -> Result<Json<DeliberationDiscussionResponse>, Error2> {
    let space_pk = decode(&space_pk).unwrap_or_default().to_string();
    let discussion_pk = decode(&discussion_pk).unwrap_or_default().to_string();

    let user = extract_user_from_session(&dynamo.client, &session).await?;
    let space_id = space_pk
        .strip_prefix(SPACE_PREFIX)
        .ok_or_else(|| Error2::BadRequest("Invalid space_pk format".into()))?
        .to_string();
    let discussion_id = discussion_pk
        .strip_prefix(DISCUSSION_PREFIX)
        .ok_or_else(|| Error2::BadRequest("Invalid discussion_pk format".into()))?
        .to_string();

    let user_pk = match user.pk {
        Partition::User(v) | Partition::Team(v) => v,
        _ => String::new(),
    };

    let (disc, disc_pk) = fetch_discussion_and_pk(&dynamo, &space_id, &discussion_id).await?;
    if disc.meeting_id.is_none() {
        return Err(Error2::AwsChimeError("Not Found Meeting ID".into()));
    }

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

    let members_resp = list_members_resp(&dynamo, &disc_pk).await?;
    let participants_resp = list_participants_resp(&dynamo, &disc_pk).await?;

    let mut res: DeliberationDiscussionResponse = disc.into();
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
