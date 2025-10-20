use crate::controllers::v3::spaces::SpaceDiscussionPath;
use crate::controllers::v3::spaces::SpaceDiscussionPathParam;
use crate::features::spaces::discussions::dto::ListDiscussionMemberResponse;
use crate::features::spaces::discussions::dto::ListDiscussionQueryParams;
use crate::features::spaces::discussions::models::space_discussion_member::SpaceDiscussionMember;
use crate::features::spaces::discussions::models::space_discussion_member::SpaceDiscussionMemberQueryOption;
use crate::types::Partition;
use crate::{AppState, Error2, models::user::User};
use aide::NoApi;
use bdk::prelude::axum::extract::{Json, Path, Query, State};
use bdk::prelude::*;

pub async fn get_discussion_members_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
    Path(SpaceDiscussionPathParam {
        space_pk,
        discussion_pk,
    }): SpaceDiscussionPath,
    Query(ListDiscussionQueryParams { bookmark }): Query<ListDiscussionQueryParams>,
) -> Result<Json<ListDiscussionMemberResponse>, Error2> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error2::NotFoundSpace);
    }

    let mut query_options = SpaceDiscussionMemberQueryOption::builder()
        .sk("SPACE_DISCUSSION_MEMBER#".into())
        .limit(10);

    if let Some(bookmark) = bookmark {
        query_options = query_options.bookmark(bookmark);
    }

    let (members, bookmark) =
        SpaceDiscussionMember::query(&dynamo.client, discussion_pk.clone(), query_options).await?;

    Ok(Json(ListDiscussionMemberResponse { members, bookmark }))
}
