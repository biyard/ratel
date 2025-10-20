use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::discussions::dto::SpaceDiscussionRequest;
use crate::features::spaces::discussions::dto::{
    CreateDiscussionResponse, SpaceDiscussionResponse,
};
use crate::features::spaces::discussions::models::space_discussion::SpaceDiscussion;
use crate::features::spaces::discussions::models::space_discussion_member::SpaceDiscussionMember;
use crate::models::SpaceCommon;
use crate::types::{Partition, TeamGroupPermission};
use crate::{AppState, Error2, models::user::User, types::EntityType};
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;

use aide::NoApi;

pub async fn create_discussion_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<SpaceDiscussionRequest>,
) -> Result<Json<CreateDiscussionResponse>, Error2> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error2::NotFoundSpace);
    }

    let (_, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        Some(&user.pk),
        TeamGroupPermission::SpaceEdit,
    )
    .await?;
    if !has_perm {
        return Err(Error2::NoPermission);
    }

    let disc = SpaceDiscussion::new(
        space_pk.clone(),
        req.name,
        req.description,
        req.started_at,
        req.ended_at,
        None,
        "".to_string(),
        None,
        None,
    );

    disc.create(&dynamo.client).await?;

    let disc_id = match disc.clone().sk {
        EntityType::SpaceDiscussion(v) => v,
        _ => "".to_string(),
    };

    let discussion_pk = Partition::Discussion(disc_id.clone());

    let mut tx = vec![];

    for member in req.user_ids.clone() {
        let user = User::get(&dynamo.client, member, Some(EntityType::User))
            .await?
            .ok_or(Error2::NotFound("User not found".into()))?;

        let m =
            SpaceDiscussionMember::new(discussion_pk.clone(), user).create_transact_write_item();

        tx.push(m);

        if tx.len() == 10 {
            dynamo
                .client
                .transact_write_items()
                .set_transact_items(Some(tx.clone()))
                .send()
                .await
                .map_err(|e| {
                    tracing::error!("Failed to create discussion: {:?}", e);
                    Error2::InternalServerError("Failed to create discussion".into())
                })?;

            tx.clear();
        }
    }

    if !tx.is_empty() {
        dynamo
            .client
            .transact_write_items()
            .set_transact_items(Some(tx))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to create discussion: {:?}", e);
                Error2::InternalServerError("Failed to create discussion".into())
            })?;
    }

    let mut discussion: SpaceDiscussionResponse = disc.into();
    let is_member =
        SpaceDiscussionMember::is_member(&dynamo.client, &discussion_pk, &user.pk).await?;

    discussion.is_member = is_member;

    Ok(Json(CreateDiscussionResponse { discussion }))
}
