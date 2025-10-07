use crate::AppState;
use crate::controllers::v3::posts::post_response::PostResponse;
use crate::models::feed::*;
use crate::models::user::User;
use crate::types::*;
use aide::NoApi;
use axum::extract::*;
use bdk::prelude::*;

pub async fn list_my_drafts_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Query(Pagination { bookmark }): ListItemsQuery,
) -> Result<Json<ListItemsResponse<PostResponse>>, crate::Error2> {
    tracing::debug!("Handling request: {:?}", bookmark);

    let mut opt = PostQueryOption::builder().sk(PostStatus::Draft.to_string());

    if let Some(bookmark) = bookmark {
        opt = opt.bookmark(bookmark);
    }

    let posts = Post::find_by_user_and_status(&dynamo.client, &user.pk, opt).await?;

    let response_items: Vec<PostResponse> = posts.0.into_iter().map(PostResponse::from).collect();

    Ok(Json(ListItemsResponse {
        items: response_items,
        bookmark: posts.1,
    }))
}
