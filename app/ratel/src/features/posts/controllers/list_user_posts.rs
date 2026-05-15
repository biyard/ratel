use crate::common::models::space::SpaceCommon;
use crate::common::types::SpacePublishState;
use crate::features::auth::OptionalUser;
use crate::features::posts::controllers::dto::*;
use crate::features::posts::models::*;
use crate::features::posts::types::*;
use crate::features::posts::*;
use crate::features::sub_team::models::{
    SubTeamAnnouncement, SubTeamAnnouncementFanout, SubTeamAnnouncementStatus,
};
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

    let (users, _) =
        crate::features::auth::User::find_by_username(cli, &username, Default::default()).await?;
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
    let mut query_options = Post::opt()
        .limit(fetch_limit)
        .sk(PostStatus::Published.to_string());

    if let Some(bookmark) = bookmark {
        query_options = query_options.bookmark(bookmark);
    }

    let (posts, bookmark) = Post::find_by_user_and_status(cli, &team_pk, query_options).await?;

    // Union the team's own Posts with anchor Posts pointed at by every
    // `SubTeamAnnouncementFanout` marker filed under this team — those
    // are broadcasts a parent team published, surfaced into this team's
    // wall WITHOUT cloning the Post row (the anchor lives at
    // `Feed(announcement_id)` on the parent team's pk). One row per
    // announcement, same URL / likes / comments everywhere. Broadcasts
    // are only added to the result when the viewer is a member of this
    // team — non-members and anonymous visitors see the team's own
    // Posts only.
    let posts = merge_broadcast_anchors(cli, &team_pk, posts, user.as_ref().map(|u| &u.pk)).await;

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

    // Hide broadcast Posts whose attached Space is still Draft. The
    // Space designer's "Publish" button is what flips publish_state →
    // Published, and until then no team should see the half-built Space
    // in their feed — children via the fanout markers, parent via the
    // anchor on their own wall. (The broadcast management tab still
    // surfaces the row so the parent admin can find and design it.)
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

/// Union the team's own Posts with anchor Posts pointed at by every
/// `SubTeamAnnouncementFanout` marker filed under this team's pk.
///
/// The marker pattern replaces the older "clone the Post into every
/// child team's feed" fanout. One anchor Post lives on the parent's
/// pk; each recognized child gets a single lightweight marker row
/// pointing at it. This function reads those markers, batch-gets the
/// anchor Posts, and merges them into the wall result, dedup'd against
/// the team's own Posts.
///
/// **Audience gate:** broadcast anchors are only included when the
/// `viewer` is a member of this team (the wall's team) — non-members
/// landing on a recognized child's wall URL don't see the parent's
/// broadcast Posts. Anonymous viewers are also excluded. The full
/// audience for a broadcast (parent's members + every recognized
/// child's members) is enforced separately in `get_post_handler` via
/// `broadcast_access::can_view_broadcast_post` so the canonical
/// `/posts/{id}` URL also denies non-members.
///
/// Union sources of broadcast anchor Posts for this wall:
///
/// 1. **Child team wall** — anchors reached via `SubTeamAnnouncementFanout`
///    markers filed under this child team's pk by the fanout service.
/// 2. **Parent team wall** — anchors reached via the parent's own
///    `SubTeamAnnouncement` rows. The parent's gsi5 page query
///    eventually returns the same Posts, but only when they fall
///    inside the current page's `created_at` window. Pulling them
///    directly here guarantees broadcasts pin to page 0 regardless
///    of how many regular Posts have been written since.
///
/// Both sources run unconditionally — each is empty for the wrong
/// kind of wall (child has no announcements, parent has no markers),
/// so the union is a no-op in those cases.
///
/// Sort order: `pinned_as_announcement` desc first (broadcast anchors
/// always pin), then `created_at` desc for the rest.
#[cfg(feature = "server")]
async fn merge_broadcast_anchors(
    cli: &aws_sdk_dynamodb::Client,
    team_pk: &Partition,
    own_posts: Vec<Post>,
    viewer: Option<&Partition>,
) -> Vec<Post> {
    // Collect anchor pks from both sources before any membership /
    // batch_get work — if neither source returns anything, short-circuit.
    let mut anchor_pks: Vec<Partition> = Vec::new();

    // Source 1 — fanout markers (child team wall).
    let opt = SubTeamAnnouncementFanout::opt()
        .limit(100)
        .sk("SUB_TEAM_ANNOUNCEMENT_FANOUT".to_string());
    match SubTeamAnnouncementFanout::query(cli, team_pk.clone(), opt).await {
        Ok((markers, _)) => {
            for m in markers {
                anchor_pks.push(Partition::Feed(m.announcement_id));
            }
        }
        Err(e) => {
            tracing::warn!("merge_broadcast_anchors: marker query failed: {e}");
        }
    };

    // Source 2 — parent team's own announcements. Only run when the
    // wall is on a Team partition (non-Team walls have no announcements).
    if matches!(team_pk, Partition::Team(_)) {
        let opt = SubTeamAnnouncement::opt()
            .limit(100)
            .sk("SUB_TEAM_ANNOUNCEMENT#".to_string());
        match SubTeamAnnouncement::query(cli, team_pk.clone(), opt).await {
            Ok((announcements, _)) => {
                for a in announcements {
                    // Only Published announcements have a live anchor
                    // Post. Direct messages and drafts skip this path —
                    // direct messages already land in own_posts via
                    // `user_pk = child_team_pk`, and drafts have no
                    // anchor Post to surface yet.
                    if !matches!(a.status, SubTeamAnnouncementStatus::Published) {
                        continue;
                    }
                    if a.target_child_team_id.is_some() {
                        continue;
                    }
                    anchor_pks.push(Partition::Feed(a.announcement_id));
                }
            }
            Err(e) => {
                tracing::warn!("merge_broadcast_anchors: announcement query failed: {e}");
            }
        };
    }

    if anchor_pks.is_empty() {
        return own_posts;
    }

    // Dedup anchor pks before the batch_get — both sources can name
    // the same announcement on edge cases.
    let mut seen: HashMap<String, ()> = HashMap::new();
    anchor_pks.retain(|p| seen.insert(p.to_string(), ()).is_none());

    // Membership gate — only surface broadcast anchors to members of
    // this wall's team. Anonymous viewers never see them.
    let viewer_is_member = match viewer {
        Some(v) => crate::features::sub_team::services::broadcast_access::is_team_member_public(
            cli, team_pk, v,
        )
        .await
        .unwrap_or(false),
        None => false,
    };
    if !viewer_is_member {
        return own_posts;
    }

    let keys: Vec<(Partition, EntityType)> = anchor_pks
        .into_iter()
        .map(|pk| (pk, EntityType::Post))
        .collect();
    let anchors: Vec<Post> = Post::batch_get(cli, keys).await.unwrap_or_default();
    let anchors: Vec<Post> = anchors
        .into_iter()
        .filter(|p| matches!(p.status, PostStatus::Published))
        .collect();

    let mut by_pk: HashMap<String, Post> = HashMap::new();
    for p in own_posts {
        by_pk.insert(p.pk.to_string(), p);
    }
    for p in anchors {
        by_pk.entry(p.pk.to_string()).or_insert(p);
    }

    let mut merged: Vec<Post> = by_pk.into_values().collect();
    // Pinned (broadcast anchors flagged `pinned_as_announcement`) sit at
    // the top; the rest fall back to `created_at` desc. Stable ordering
    // means a newly published broadcast pops to the front of the wall
    // even when older non-broadcast posts have higher created_at.
    merged.sort_by(|a, b| {
        b.pinned_as_announcement
            .cmp(&a.pinned_as_announcement)
            .then_with(|| b.created_at.cmp(&a.created_at))
    });
    merged
}

/// Hide broadcast Posts whose attached Space is still in `Draft`
/// publish-state from **child team walls only**. Parent admins must
/// still see their own broadcast on the parent team's wall — they're
/// the ones who need to open the Space designer and publish it, so
/// hiding it from the parent would leave them with no entry point
/// outside the broadcast management tab.
///
/// Rule:
/// - Wall team == announcement's parent team (`announcement_parent_team_id`)
///   → show the row regardless of Space publish_state.
/// - Wall team != parent (child team wall, surfaced via fanout marker)
///   → hide until the parent publishes the Space.
#[cfg(feature = "server")]
async fn filter_unpublished_broadcast_spaces(
    cli: &aws_sdk_dynamodb::Client,
    team_pk: &Partition,
    posts: Vec<Post>,
) -> Vec<Post> {
    let wall_team_id: Option<&str> = match team_pk {
        Partition::Team(id) => Some(id.as_str()),
        _ => None,
    };

    let mut to_check: Vec<Partition> = posts
        .iter()
        .filter(|p| p.announcement_id.is_some() && p.space_pk.is_some())
        .filter(|p| p.announcement_parent_team_id.as_deref() != wall_team_id)
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
            if p.announcement_id.is_none() {
                return true;
            }
            if p.announcement_parent_team_id.as_deref() == wall_team_id {
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
