use crate::controllers::dto::*;
use crate::models::*;
use crate::types::*;
use crate::*;
use ratel_auth::User;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ListUserDraftsQueryParams {
    pub bookmark: Option<String>,
}

// FIXME: Use GET when dioxus server functions support query params without body.
#[post("/api/posts/drafts", user: User)]
pub async fn list_user_drafts_handler(
    params: ListUserDraftsQueryParams,
) -> Result<ListItemsResponse<PostResponse>> {
    let conf = crate::config::get();
    let cli = conf.dynamodb();

    tracing::debug!(
        "list_user_drafts_handler: bookmark = {:?}",
        params.bookmark
    );

    let mut query_options = Post::opt()
        .limit(10)
        .sk(PostStatus::Draft.to_string());

    if let Some(bookmark) = params.bookmark {
        query_options = query_options.bookmark(bookmark);
    }

    let (posts, bookmark) =
        Post::find_by_user_and_status(cli, &user.pk, query_options).await?;

    tracing::debug!(
        "list_user_drafts_handler: found {} drafts, next bookmark = {:?}",
        posts.len(),
        bookmark
    );

    let items: Vec<PostResponse> = posts
        .into_iter()
        .map(|post| PostResponse::from((Some(user.clone()), post)))
        .collect();

    Ok(ListItemsResponse { items, bookmark })
}
