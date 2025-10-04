use crate::{
    AppState,
    controllers::v3::posts::post_response::PostResponse,
    models::{
        feed::{Post, PostQueryOption},
        user::User,
    },
    types::{PostStatus, list_items_response::ListItemsResponse},
};
use aide::NoApi;
use axum::extract::State;
use axum::*;
use bdk::prelude::*;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, schemars::JsonSchema,
)]
pub struct ListMyPostsRequest {
    #[schemars(description = "A pagination token, if any")]
    pub bookmark: Option<String>,
}

pub async fn list_my_posts_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Json(ListMyPostsRequest { bookmark }): Json<ListMyPostsRequest>,
) -> Result<Json<ListItemsResponse<PostResponse>>, crate::Error2> {
    tracing::debug!("Handling request: {:?}", bookmark);

    let mut opt = PostQueryOption::builder().sk(PostStatus::Published.to_string());

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
