use crate::controllers::v3::spaces::{SpaceDiscussionPath, SpaceDiscussionPathParam};
use crate::features::spaces::discussions::dto::{SpaceDiscussionRequest, UpdateDiscussionResponse};
use crate::features::spaces::discussions::models::space_discussion::SpaceDiscussion;
use crate::features::spaces::discussions::models::space_discussion_member::{
    SpaceDiscussionMember, SpaceDiscussionMemberQueryOption,
};
use crate::features::spaces::discussions::models::space_discussion_participant::{
    SpaceDiscussionParticipant, SpaceDiscussionParticipantQueryOption,
};
use crate::models::User;
use crate::types::{EntityType, Partition, TeamGroupPermission};
use crate::{AppState, Error, Permissions};
use axum::extract::{Json, Path, State};
use bdk::prelude::aide::NoApi;
use bdk::prelude::*;

pub async fn update_discussion_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    NoApi(user): NoApi<User>,
    Path(SpaceDiscussionPathParam {
        space_pk,
        discussion_pk,
    }): SpaceDiscussionPath,
    Json(req): Json<SpaceDiscussionRequest>,
) -> Result<Json<UpdateDiscussionResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceEdit) {
        return Err(Error::NoPermission);
    }

    let (pk, sk) = SpaceDiscussion::keys(&space_pk, &discussion_pk);

    let _discussion =
        SpaceDiscussion::get_discussion(&dynamo.client, &space_pk, &discussion_pk, &user.pk)
            .await?;

    let mut bookmark = None::<String>;

    // DELETE EXIST MEMBERS
    loop {
        let (responses, new_bookmark) = SpaceDiscussionMember::query(
            &dynamo.client,
            discussion_pk.clone(),
            if let Some(b) = &bookmark {
                SpaceDiscussionMemberQueryOption::builder().bookmark(b.clone())
            } else {
                SpaceDiscussionMemberQueryOption::builder()
            },
        )
        .await?;

        for response in responses {
            SpaceDiscussionMember::delete(&dynamo.client, response.pk, Some(response.sk)).await?;
        }

        match new_bookmark {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    bookmark = None;

    loop {
        let (responses, new_bookmark) = SpaceDiscussionParticipant::query(
            &dynamo.client,
            discussion_pk.clone(),
            if let Some(b) = &bookmark {
                SpaceDiscussionParticipantQueryOption::builder().bookmark(b.clone())
            } else {
                SpaceDiscussionParticipantQueryOption::builder()
            },
        )
        .await?;

        for response in responses {
            SpaceDiscussionParticipant::delete(&dynamo.client, response.pk, Some(response.sk))
                .await?;
        }

        match new_bookmark {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    // UPDATE DISCUSSION
    let mut tx = vec![];

    let d = SpaceDiscussion::updater(&pk, sk)
        .with_name(req.name)
        .with_description(req.description)
        .with_started_at(req.started_at)
        .with_ended_at(req.ended_at)
        .transact_write_item();

    tx.push(d);

    for member in req.user_ids {
        let user = User::get(&dynamo.client, member, Some(EntityType::User))
            .await?
            .ok_or(Error::NotFound("User not found".into()))?;

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
                    tracing::error!("Failed to update discussion: {:?}", e);
                    Error::InternalServerError("Failed to update discussion".into())
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
                tracing::error!("Failed to update discussion: {:?}", e);
                Error::InternalServerError("Failed to update discussion".into())
            })?;
    }

    // QUERY DISCUSSION
    let discussion =
        SpaceDiscussion::get_discussion(&dynamo.client, &space_pk, &discussion_pk, &user.pk)
            .await?;

    Ok(Json(UpdateDiscussionResponse { discussion }))
}
