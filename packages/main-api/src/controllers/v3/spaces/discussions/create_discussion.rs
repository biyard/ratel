use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::discussions::dto::SpaceDiscussionRequest;
use crate::features::spaces::discussions::dto::{
    CreateDiscussionResponse, SpaceDiscussionResponse,
};
use crate::features::spaces::discussions::dto::{
    SpaceDiscussionMemberResponse, SpaceDiscussionParticipantResponse,
};
use crate::features::spaces::discussions::models::space_discussion::SpaceDiscussion;
use crate::features::spaces::discussions::models::space_discussion_member::{
    SpaceDiscussionMember, SpaceDiscussionMemberQueryOption,
};
use crate::features::spaces::discussions::models::space_discussion_participant::{
    SpaceDiscussionParticipant, SpaceDiscussionParticipantQueryOption,
};
use crate::models::SpaceCommon;
use crate::types::{Partition, TeamGroupPermission};
use crate::{AppState, Error2, models::user::User, types::EntityType};
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;

use aide::NoApi;

pub async fn create_discussion_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<SpaceDiscussionRequest>,
) -> Result<Json<CreateDiscussionResponse>, Error2> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error2::NotFoundSpace);
    }

    let (_, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        Some(&user.unwrap_or_default().pk),
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

    let mut tx = vec![];

    for member in req.user_ids.clone() {
        let user = User::get(&dynamo.client, member, Some(EntityType::User))
            .await?
            .ok_or(Error2::NotFound("User not found".into()))?;

        let m = SpaceDiscussionMember::new(Partition::Discussion(disc_id.clone()), user)
            .create_transact_write_item();

        tx.push(m);

        if tx.len() == 100 {
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

    Ok(Json(CreateDiscussionResponse { discussion }))
}
