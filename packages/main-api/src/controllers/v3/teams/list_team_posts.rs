use crate::controllers::v3::posts::post_response::PostResponse;
use crate::models::feed::{Post, PostLike, PostQueryOption};
use crate::models::user::User;
use crate::types::list_items_response::ListItemsResponse;
use crate::*;

use super::dto::{TeamPath, TeamPathParam};

#[derive(Debug, Deserialize, serde::Serialize, aide::OperationIo, JsonSchema, Validate)]
pub struct ListTeamPostsQueryParams {
    pub bookmark: Option<String>,
    /// Filter posts by status (draft, published, etc.)

    #[serde(default)]
    pub status: PostStatus,
}

pub async fn list_team_posts_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    NoApi(permission): NoApi<Permissions>,
    Path(TeamPathParam { team_pk }): TeamPath,
    Query(ListTeamPostsQueryParams { bookmark, status }): Query<ListTeamPostsQueryParams>,
) -> Result<Json<ListItemsResponse<PostResponse>>> {
    tracing::debug!(
        "list_team_posts_handler: user = {:?}, team_pk = {:?}, bookmark = {:?}, status = {:?}",
        user,
        team_pk,
        bookmark,
        status
    );

    if status == PostStatus::Draft && !permission.is_admin() {
        return Err(Error::NoPermission);
    }

    let opt = Post::opt_with_bookmark(bookmark).sk(status.to_string());

    let (posts, bookmark) = Post::find_by_user_and_status(&dynamo.client, &team_pk, opt).await?;

    tracing::debug!(
        "list_team_posts_handler: found {} posts, next bookmark = {:?}",
        posts.len(),
        bookmark
    );

    // Fetch post likes for authenticated users
    let likes = match (&user, posts.is_empty()) {
        (Some(user), false) => {
            PostLike::batch_get(
                &dynamo.client,
                posts
                    .iter()
                    .map(|post| PostLike::keys(&post.pk, &user.pk))
                    .collect(),
            )
            .await?
        }
        _ => vec![],
    };

    tracing::debug!("list_team_posts_handler: returning {} items", posts.len());
    let items: Vec<PostResponse> = posts
        .into_iter()
        .map(|post| {
            let post_like_pk = post
                .pk
                .clone()
                .to_post_like_key()
                .expect("to_post_like_key");
            let liked = likes.iter().any(|like| like.pk == post_like_pk);
            PostResponse::from(post).with_like(liked)
        })
        .collect();

    Ok(Json(ListItemsResponse { items, bookmark }))
}
