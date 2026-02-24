use crate::controllers::dto::*;
use crate::models::*;
use crate::types::*;
use crate::*;
use ratel_auth::OptionalUser;

#[get("/api/posts?bookmark", user: OptionalUser)]
pub async fn list_posts_handler(
    bookmark: Option<String>,
) -> Result<ListItemsResponse<PostResponse>> {
    let conf = crate::config::get();
    let cli = conf.dynamodb();

    let user: Option<ratel_auth::User> = user.into();

    tracing::debug!(
        "list_posts_handler: user = {:?} bookmark = {:?}",
        user,
        bookmark
    );

    let mut query_options = Post::opt().limit(10);

    if let Some(bookmark) = bookmark {
        query_options = query_options.bookmark(bookmark);
    }
    let (posts, bookmark) =
        Post::find_by_visibility(cli, Visibility::Public, query_options).await?;
    tracing::debug!(
        "list_posts_handler: found {} posts, next bookmark = {:?}",
        posts.len(),
        bookmark
    );

    let likes = match (&user, posts.is_empty()) {
        (Some(user), false) => {
            let likes = PostLike::batch_get(
                cli,
                posts
                    .iter()
                    .map(|post| PostLike::keys(&post.pk, &user.pk))
                    .collect(),
            )
            .await?;

            likes
        }
        _ => vec![],
    };

    tracing::debug!("list_posts_handler: returning {} items", posts.len());
    let items: Vec<PostResponse> = posts
        .into_iter()
        .map(|post| {
            let post_like_pk = post
                .pk
                .clone()
                .to_post_like_key()
                .expect("to_post_like_key");
            let liked = likes.iter().any(|like| like.pk == post_like_pk);
            PostResponse::from((user.clone(), post)).with_like(liked)
        })
        .collect();

    Ok(ListItemsResponse { items, bookmark })
}
