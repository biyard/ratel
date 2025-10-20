use crate::controllers::v3::spaces::{SpaceDiscussionPath, SpaceDiscussionPathParam};
use crate::features::common_controller_logic::get_discussion;
use crate::features::dto::SpaceDiscussionResponse;
use crate::features::models::space_discussion::SpaceDiscussion;
use crate::features::models::space_discussion_participant::{
    SpaceDiscussionParticipant, SpaceDiscussionParticipantQueryOption,
};
use crate::types::Partition;
use crate::{AppState, Error2, models::user::User, types::EntityType};
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;

use aide::NoApi;

pub async fn exit_meeting_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(SpaceDiscussionPathParam {
        space_pk,
        discussion_pk,
    }): SpaceDiscussionPath,
) -> Result<Json<SpaceDiscussionResponse>, Error2> {
    let user = user.unwrap_or_default();
    let discussion_id = match discussion_pk.clone() {
        Partition::Discussion(v) => v,
        _ => "".to_string(),
    };

    let disc = SpaceDiscussion::get(
        &dynamo.client,
        space_pk.clone(),
        Some(EntityType::SpaceDiscussion(discussion_id.to_string())),
    )
    .await?;

    if disc.is_none() {
        return Err(Error2::NotFoundDiscussion);
    }

    let disc = disc.unwrap();
    if disc.meeting_id.is_none() {
        return Err(Error2::AwsChimeError("Not Found Meeting ID".into()));
    }

    let olds = SpaceDiscussionParticipant::find_by_user_pk(
        &dynamo.client,
        user.pk.clone(),
        SpaceDiscussionParticipantQueryOption::builder(),
    )
    .await?
    .0;

    let mut tx = vec![];

    for p in olds {
        let d = SpaceDiscussionParticipant::updater(p.pk, p.sk)
            .with_participant_id("".to_string())
            .transact_write_item();

        tx.push(d);

        if tx.len() == 10 {
            dynamo
                .client
                .transact_write_items()
                .set_transact_items(Some(tx.clone()))
                .send()
                .await
                .map_err(|e| {
                    tracing::error!("Failed to update discussion participants: {:?}", e);
                    Error2::InternalServerError("Failed to update discussion participants".into())
                })?;

            tx.clear();
        }
    }

    if !tx.is_empty() {
        dynamo
            .client
            .transact_write_items()
            .set_transact_items(Some(tx.clone()))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to update discussion participants: {:?}", e);
                Error2::InternalServerError("Failed to update discussion participants".into())
            })?;

        tx.clear();
    }

    let discussion = get_discussion(&dynamo, space_pk, discussion_pk).await?;

    Ok(Json(discussion))
}
