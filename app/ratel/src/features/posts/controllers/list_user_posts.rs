use crate::features::posts::controllers::dto::*;
use crate::features::posts::models::*;
use crate::features::posts::types::*;
use crate::features::posts::*;
use crate::features::auth::OptionalUser;

#[get("/api/posts/by-user/:username?bookmark", user: OptionalUser)]
pub async fn list_user_posts_handler(
    username: String,
    bookmark: Option<String>,
) -> Result<ListItemsResponse<PostResponse>> {
    let conf = crate::features::posts::config::get();
    let cli = conf.dynamodb();

    let user: Option<crate::features::auth::User> = user.into();

    tracing::debug!(
        "list_user_posts_handler: username = {:?} bookmark = {:?}",
        username,
        bookmark
    );

    let (users, _) = crate::features::auth::User::find_by_username(cli, &username, Default::default()).await?;
    let target_user = users.into_iter().next().ok_or(Error::PostInvalidUsername)?;
    let user_pk = target_user.pk;
    let is_owner = match &user {
        Some(user) => user.pk == user_pk,
        None => false,
    };

    let mut query_options = Post::opt().limit(10).sk(if is_owner {
        // FIXME: When user is owner, it doesn't support time-sorted result.
        PostStatus::Published.to_string()
    } else {
        format!("{}#{}", PostStatus::Published, Visibility::Public)
    });

    if let Some(bookmark) = bookmark {
        query_options = query_options.bookmark(bookmark);
    }

    let (posts, bookmark) = Post::find_by_user_and_status(cli, &user_pk, query_options).await?;

    tracing::debug!(
        "list_user_posts_handler: found {} posts, next bookmark = {:?}",
        posts.len(),
        bookmark
    );

    let likes = match (&user, posts.is_empty()) {
        (Some(user), false) => {
            PostLike::batch_get(
                cli,
                posts
                    .iter()
                    .map(|post| PostLike::keys(&post.pk, &user.pk))
                    .collect(),
            )
            .await?
        }
        _ => vec![],
    };

    tracing::debug!("list_user_posts_handler: returning {} items", posts.len());
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

#[get("/api/posts/by-team/:teamname?category&bookmark", user: OptionalUser)]
pub async fn list_team_posts_handler(
    teamname: String,
    category: Option<String>,
    bookmark: Option<String>,
) -> Result<ListItemsResponse<PostResponse>> {
    let conf = crate::features::posts::config::get();
    let cli = conf.dynamodb();

    let user: Option<crate::features::auth::User> = user.into();

    tracing::debug!(
        "list_team_posts_handler: teamname = {:?} category = {:?} bookmark = {:?}",
        teamname,
        category,
        bookmark
    );

    let opt = Team::opt().limit(1);
    let (teams, _): (Vec<Team>, _) = Team::find_by_username_prefix(cli, &teamname, opt).await?;
    let team = teams
        .into_iter()
        .next()
        .ok_or(Error::NotFound(format!("Team not found: {}", teamname)))?;
    let team_pk = team.pk;

    let fetch_limit = if category.is_some() { 50 } else { 10 };
    let mut query_options = Post::opt().limit(fetch_limit).sk(PostStatus::Published.to_string());

    if let Some(bookmark) = bookmark {
        query_options = query_options.bookmark(bookmark);
    }

    let (posts, bookmark) = Post::find_by_user_and_status(cli, &team_pk, query_options).await?;

    let posts = if let Some(ref cat) = category {
        posts.into_iter().filter(|p| p.categories.iter().any(|c| c == cat.as_str())).collect::<Vec<_>>()
    } else {
        posts
    };

    tracing::debug!(
        "list_team_posts_handler: found {} posts, next bookmark = {:?}",
        posts.len(),
        bookmark
    );

    let likes = match (&user, posts.is_empty()) {
        (Some(user), false) => {
            PostLike::batch_get(
                cli,
                posts
                    .iter()
                    .map(|post| PostLike::keys(&post.pk, &user.pk))
                    .collect(),
            )
            .await?
        }
        _ => vec![],
    };

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
