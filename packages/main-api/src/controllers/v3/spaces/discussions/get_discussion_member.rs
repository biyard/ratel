use crate::controllers::v3::spaces::SpaceDiscussionPath;
use crate::controllers::v3::spaces::SpaceDiscussionPathParam;
use crate::features::spaces::discussions::dto::SpaceDiscussionMemberResponse;
use crate::features::spaces::discussions::models::space_discussion_member::SpaceDiscussionMember;
use crate::features::spaces::discussions::models::space_discussion_member::SpaceDiscussionMemberQueryOption;
use crate::types::Partition;
use crate::{AppState, Error2, models::user::User};
use aide::NoApi;
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;

pub async fn get_discussion_members_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
    Path(SpaceDiscussionPathParam {
        space_pk,
        discussion_pk,
    }): SpaceDiscussionPath,
) -> Result<Json<Vec<SpaceDiscussionMemberResponse>>, Error2> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error2::NotFoundSpace);
    }

    let mut members: Vec<SpaceDiscussionMemberResponse> = vec![];
    let mut bookmark = None::<String>;

    loop {
        let (responses, new_bookmark) = SpaceDiscussionMember::query(
            &dynamo.client,
            discussion_pk.clone(),
            if let Some(b) = &bookmark {
                SpaceDiscussionMemberQueryOption::builder()
                    .sk("SPACE_DISCUSSION_MEMBER#".into())
                    .bookmark(b.clone())
            } else {
                SpaceDiscussionMemberQueryOption::builder().sk("SPACE_DISCUSSION_MEMBER#".into())
            },
        )
        .await?;

        for response in responses {
            members.push(response.into());
        }

        match new_bookmark {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    Ok(Json(members))
}
