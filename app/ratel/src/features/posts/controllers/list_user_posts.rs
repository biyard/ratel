use crate::features::posts::controllers::dto::*;
use crate::features::posts::models::*;
use crate::features::posts::types::*;
use crate::features::posts::*;
use crate::features::auth::OptionalUser;
use crate::common::models::space::SpaceCommon;
use crate::common::types::SpacePublishState;
use std::collections::HashMap;

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
            PostResponse::from(post).with_like(liked)
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

    let gsi2_sk_prefix = Team::compose_gsi2_sk(String::default());
    let opt = Team::opt().limit(1).sk(gsi2_sk_prefix);
    let (teams, _): (Vec<Team>, _) = Team::find_by_username_prefix(cli, &teamname, opt).await?;
    let team = teams
        .into_iter()
        .find(|t| t.username == teamname)
        .ok_or(Error::NotFound(format!("Team not found: {}", teamname)))?;
    let team_pk = team.pk;

    let fetch_limit = if category.is_some() { 50 } else { 10 };
    let mut query_options = Post::opt().limit(fetch_limit).sk(PostStatus::Published.to_string());

    if let Some(bookmark) = bookmark {
        query_options = query_options.bookmark(bookmark);
    }

    let (posts, bookmark) = Post::find_by_user_and_status(cli, &team_pk, query_options).await?;

    // Category filter:
    // - When caller passes `category`, return ONLY posts that carry it
    //   (used by the bylaws page with "Bylaws" / "ClubBylaws").
    // - When omitted, hide bylaws-category posts so the team's main
    //   feed never surfaces them — they live on the dedicated bylaws
    //   page only.
    let posts = if let Some(ref cat) = category {
        posts
            .into_iter()
            .filter(|p| p.categories.iter().any(|c| c == cat.as_str()))
            .collect::<Vec<_>>()
    } else {
        posts
            .into_iter()
            .filter(|p| {
                !p.categories
                    .iter()
                    .any(|c| c == "Bylaws" || c == "ClubBylaws")
            })
            .collect::<Vec<_>>()
    };

    // Hide fanned-out broadcast Posts from OTHER teams whose attached
    // Space is still Draft. The Space designer's "Publish" button is
    // what flips publish_state → Published, and until then the sub-team
    // shouldn't see the half-built Space in their feed.
    // The parent team's own anchor post (announcement_parent_team_id ==
    // team being queried) bypasses this gate so the parent admin still
    // sees their own broadcasts before designing the Space.
    let posts = filter_unpublished_broadcast_spaces(cli, &team_pk, posts).await;

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
            PostResponse::from(post).with_like(liked)
        })
        .collect();

    Ok(ListItemsResponse { items, bookmark })
}

/// Hide broadcast Posts fanned-out from a different parent team whose
/// linked Space hasn't been published yet. Posts without a Space, posts
/// without an announcement linkage, and the parent's OWN anchor post
/// (announcement_parent_team_id == team being queried) all pass through.
#[cfg(feature = "server")]
async fn filter_unpublished_broadcast_spaces(
    cli: &aws_sdk_dynamodb::Client,
    team_pk: &Partition,
    posts: Vec<Post>,
) -> Vec<Post> {
    let team_id_str = match team_pk {
        Partition::Team(id) => id.clone(),
        _ => String::new(),
    };

    let mut to_check: Vec<Partition> = posts
        .iter()
        .filter(|p| {
            let is_fanned_out = p
                .announcement_parent_team_id
                .as_deref()
                .is_some_and(|parent_id| parent_id != team_id_str);
            is_fanned_out && p.space_pk.is_some()
        })
        .filter_map(|p| p.space_pk.clone())
        .collect();
    to_check.sort_by(|a, b| a.to_string().cmp(&b.to_string()));
    to_check.dedup_by(|a, b| a.to_string() == b.to_string());

    if to_check.is_empty() {
        return posts;
    }

    let keys: Vec<(Partition, EntityType)> = to_check
        .iter()
        .cloned()
        .map(|pk| (pk, EntityType::SpaceCommon))
        .collect();
    let spaces: Vec<SpaceCommon> = SpaceCommon::batch_get(cli, keys).await.unwrap_or_default();
    let publish_states: HashMap<String, SpacePublishState> = spaces
        .into_iter()
        .map(|s| (s.pk.to_string(), s.publish_state))
        .collect();

    posts
        .into_iter()
        .filter(|p| {
            let is_fanned_out = p
                .announcement_parent_team_id
                .as_deref()
                .is_some_and(|parent_id| parent_id != team_id_str);
            if !is_fanned_out {
                return true;
            }
            let Some(space_pk) = p.space_pk.as_ref() else {
                return true;
            };
            matches!(
                publish_states.get(&space_pk.to_string()),
                Some(SpacePublishState::Published)
            )
        })
        .collect()
}
