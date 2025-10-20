use crate::controllers::v3::spaces::SpacePath;
use crate::controllers::v3::spaces::SpacePathParam;
use crate::features::spaces::discussions::dto::ListDiscussionResponse;
use crate::features::spaces::discussions::dto::SpaceDiscussionMemberResponse;
use crate::features::spaces::discussions::dto::SpaceDiscussionParticipantResponse;
use crate::features::spaces::discussions::dto::SpaceDiscussionResponse;
use crate::features::spaces::discussions::models::space_discussion::SpaceDiscussion;
use crate::features::spaces::discussions::models::space_discussion::SpaceDiscussionQueryOption;
use crate::features::spaces::discussions::models::space_discussion_member::SpaceDiscussionMember;
use crate::features::spaces::discussions::models::space_discussion_member::SpaceDiscussionMemberQueryOption;
use crate::features::spaces::discussions::models::space_discussion_participant::SpaceDiscussionParticipant;
use crate::features::spaces::discussions::models::space_discussion_participant::SpaceDiscussionParticipantQueryOption;
use crate::types::EntityType;
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
        return Err(Error2::NotFoundSpace);
    }

    let responses = SpaceDiscussion::query(
        &dynamo.client,
        space_pk.clone(),
        SpaceDiscussionQueryOption::builder().sk("SPACE_DISCUSSION#".into()),
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
                match response.sk {
                    EntityType::SpaceDiscussionMember(_) => {
                        discussion_members.push(response.into());
                    }
                    EntityType::SpaceDiscussionParticipant(_) => {
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

        discussion.members = discussion_members.clone();
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
                        if response.participant_id.clone().is_some()
                            && !response.participant_id.clone().unwrap().is_empty()
                        {
                            discussion_participants.push(response.into());
                        } else {
                            let response = SpaceDiscussionMember {
                                pk: response.pk,
                                sk: response.sk,
                                user_pk: response.user_pk,
                                author_display_name: response.author_display_name,
                                author_profile_url: response.author_profile_url,
                                author_username: response.author_username,
                            };

                            discussion_members.push(response.into());
                        }
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

        discussions.push(discussion);
    }

    Ok(Json(ListDiscussionResponse { discussions }))
}
