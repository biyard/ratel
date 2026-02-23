use crate::controllers::dto::*;
use crate::models::*;
use crate::types::*;
use crate::*;
use ratel_auth::OptionalUser;

#[derive(Serialize, Deserialize)]
pub struct ListUserPostsQueryParams {
    pub username: String,
    pub bookmark: Option<String>,
}

#[post("/api/posts/by-user", user: OptionalUser)]
pub async fn list_user_posts_handler(
    params: ListUserPostsQueryParams,
) -> Result<ListItemsResponse<PostResponse>> {
    let conf = crate::config::get();
    let cli = conf.dynamodb();

    let user: Option<ratel_auth::User> = user.into();

    tracing::debug!(
        "list_user_posts_handler: username = {:?} bookmark = {:?}",
        params.username,
        params.bookmark
    );

    let (users, _) =
        ratel_auth::User::find_by_username(cli, &params.username, Default::default()).await?;
    let target_user = users.into_iter().next().ok_or(Error::NotFound(format!(
        "User not found: {}",
        params.username
    )))?;
    let user_pk = target_user.pk;

    let mut query_options = Post::opt().limit(10).sk(PostStatus::Published.to_string());

    if let Some(bookmark) = params.bookmark {
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

#[derive(Serialize, Deserialize)]
pub struct ListTeamPostsQueryParams {
    pub teamname: String,
    pub bookmark: Option<String>,
}

#[post("/api/posts/by-team", user: OptionalUser)]
pub async fn list_team_posts_handler(
    params: ListTeamPostsQueryParams,
) -> Result<ListItemsResponse<PostResponse>> {
    let conf = crate::config::get();
    let cli = conf.dynamodb();

    let user: Option<ratel_auth::User> = user.into();

    tracing::debug!(
        "list_team_posts_handler: teamname = {:?} bookmark = {:?}",
        params.teamname,
        params.bookmark
    );

    let opt = Team::opt().limit(1);
    let (teams, _): (Vec<Team>, _) =
        Team::find_by_username_prefix(cli, &params.teamname, opt).await?;
    let team = teams.into_iter().next().ok_or(Error::NotFound(format!(
        "Team not found: {}",
        params.teamname
    )))?;
    let team_pk = team.pk;

    let mut query_options = Post::opt().limit(10).sk(PostStatus::Published.to_string());

    if let Some(bookmark) = params.bookmark {
        query_options = query_options.bookmark(bookmark);
    }

    let (posts, bookmark) = Post::find_by_user_and_status(cli, &team_pk, query_options).await?;

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
