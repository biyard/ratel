use crate::controllers::v3::spaces::SpacePath;
use crate::controllers::v3::spaces::SpacePathParam;
use crate::features::spaces::discussions::dto::ListDiscussionQueryParams;
use crate::features::spaces::discussions::dto::ListDiscussionResponse;
use crate::features::spaces::discussions::dto::SpaceDiscussionResponse;
use crate::features::spaces::discussions::models::space_discussion::SpaceDiscussion;
use crate::features::spaces::discussions::models::space_discussion::SpaceDiscussionQueryOption;
use crate::features::spaces::discussions::models::space_discussion_member::SpaceDiscussionMember;
use crate::types::Partition;
use crate::{AppState, Error2, models::user::User};
use aide::NoApi;
use bdk::prelude::axum::extract::{Json, Path, Query, State};
use bdk::prelude::*;

pub async fn list_discussions_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Query(ListDiscussionQueryParams { bookmark }): Query<ListDiscussionQueryParams>,
) -> Result<Json<ListDiscussionResponse>, Error2> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error2::NotFoundSpace);
    }

    let mut query_options = SpaceDiscussionQueryOption::builder()
        .sk("SPACE_DISCUSSION#".into())
        .limit(10);

    if let Some(bookmark) = bookmark {
        query_options = query_options.bookmark(bookmark);
    }

    let (responses, bookmark) =
        SpaceDiscussion::query(&dynamo.client, space_pk.clone(), query_options).await?;

    let mut discussions = vec![];

    for response in responses {
        let mut discussion: SpaceDiscussionResponse = response.clone().into();

        let is_member =
            SpaceDiscussionMember::is_member(&dynamo.client, &discussion.pk, &user.pk).await?;

        discussion.is_member = is_member;
        discussions.push(discussion);
    }

    Ok(Json(ListDiscussionResponse {
        discussions,
        bookmark,
    }))
}
