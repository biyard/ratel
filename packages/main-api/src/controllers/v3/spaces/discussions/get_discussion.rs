use crate::controllers::v3::spaces::SpaceDiscussionPath;
use crate::controllers::v3::spaces::SpaceDiscussionPathParam;
use crate::features::dto::GetDiscussionResponse;
use crate::features::dto::SpaceDiscussionMemberResponse;
use crate::features::dto::SpaceDiscussionParticipantResponse;
use crate::features::dto::SpaceDiscussionResponse;
use crate::features::models::space_discussion::SpaceDiscussion;
use crate::features::models::space_discussion_member::SpaceDiscussionMember;
use crate::features::models::space_discussion_member::SpaceDiscussionMemberQueryOption;
use crate::features::models::space_discussion_participant::SpaceDiscussionParticipant;
use crate::features::models::space_discussion_participant::SpaceDiscussionParticipantQueryOption;
use crate::types::Partition;
use crate::{AppState, Error2, models::user::User, types::EntityType};
use aide::NoApi;
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;

pub async fn get_discussion_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
    Path(SpaceDiscussionPathParam {
        space_pk,
        discussion_pk,
    }): SpaceDiscussionPath,
) -> Result<Json<GetDiscussionResponse>, Error2> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error2::NotFoundSpace);
    }

    let discussion_id = match discussion_pk {
        Partition::Discussion(v) => v.to_string(),
        _ => "".to_string(),
    };

    let discussion = SpaceDiscussion::get(
        &dynamo.client,
        space_pk.clone(),
        Some(EntityType::SpaceDiscussion(discussion_id.to_string())),
    )
    .await?;

    if discussion.is_none() {
        return Err(Error2::NotFoundDiscussion);
    }

    let discussion = discussion.unwrap();

    let mut discussion: SpaceDiscussionResponse = discussion.into();

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

    Ok(Json(GetDiscussionResponse { discussion }))
}
