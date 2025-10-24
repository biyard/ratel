use crate::controllers::v3::spaces::SpaceDiscussionPath;
use crate::controllers::v3::spaces::SpaceDiscussionPathParam;
use crate::features::spaces::discussions::dto::SpaceDiscussionParticipantResponse;
use crate::features::spaces::discussions::models::space_discussion_participant::SpaceDiscussionParticipant;
use crate::features::spaces::discussions::models::space_discussion_participant::SpaceDiscussionParticipantQueryOption;
use crate::types::Partition;
use crate::{AppState, Error2, models::user::User};
use aide::NoApi;
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;

pub async fn get_discussion_participants_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
    Path(SpaceDiscussionPathParam {
        space_pk,
        discussion_pk,
    }): SpaceDiscussionPath,
) -> Result<Json<Vec<SpaceDiscussionParticipantResponse>>, Error2> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error2::NotFoundSpace);
    }

    let mut participants: Vec<SpaceDiscussionParticipantResponse> = vec![];
    let mut bookmark = None::<String>;

    loop {
        let (responses, new_bookmark) = SpaceDiscussionParticipant::query(
            &dynamo.client,
            discussion_pk.clone(),
            if let Some(b) = &bookmark {
                SpaceDiscussionParticipantQueryOption::builder()
                    .sk("SPACE_DISCUSSION_PARTICIPANT#".into())
                    .bookmark(b.clone())
            } else {
                SpaceDiscussionParticipantQueryOption::builder()
                    .sk("SPACE_DISCUSSION_PARTICIPANT#".into())
            },
        )
        .await?;

        for response in responses {
            participants.push(response.into());
        }

        match new_bookmark {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    Ok(Json(participants))
}
