use crate::controllers::v3::spaces::SpacePath;
use crate::controllers::v3::spaces::SpacePathParam;
use crate::features::dto::ListDiscussionResponse;
use crate::features::dto::SpaceDiscussionMemberResponse;
use crate::features::dto::SpaceDiscussionParticipantResponse;
use crate::features::dto::SpaceDiscussionResponse;
use crate::features::models::space_discussion::SpaceDiscussion;
use crate::features::models::space_discussion::SpaceDiscussionQueryOption;
use crate::features::models::space_discussion_member::SpaceDiscussionMember;
use crate::features::models::space_discussion_member::SpaceDiscussionMemberQueryOption;
use crate::features::models::space_discussion_participant::SpaceDiscussionParticipant;
use crate::features::models::space_discussion_participant::SpaceDiscussionParticipantQueryOption;
use crate::types::Partition;
use crate::{AppState, Error2, models::user::User};
use aide::NoApi;
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;

pub async fn list_discussions_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
    Path(SpacePathParam { space_pk }): SpacePath,
) -> Result<Json<ListDiscussionResponse>, Error2> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error2::NotFoundDeliberationSpace);
    }

    let responses = SpaceDiscussion::query(
        &dynamo.client,
        space_pk.clone(),
        SpaceDiscussionQueryOption::builder(),
    )
    .await?
    .0;

    let mut discussions = vec![];

    for response in responses {
        let mut discussion: SpaceDiscussionResponse = response.into();

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
                discussion_members.push(response.into());
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
                discussion_participants.push(response.into());
            }

            match new_bookmark {
                Some(b) => bookmark = Some(b),
                None => break,
            }
        }

        discussion.participants = discussion_participants;

        discussions.push(discussion);
    }

    Ok(Json(ListDiscussionResponse { discussions }))
}
