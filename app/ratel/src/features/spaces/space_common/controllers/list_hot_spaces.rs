//! Read-only Hot Spaces feed.
//!
//! All ranking work happens in `services::space_fanout`, which writes two
//! denormalized snapshots:
//!
//! - **Global** `HotSpace` rows keyed by `Partition::Space(uuid)` on a
//!   shared `HSR#ALL` GSI partition — used as the anonymous fallback so a
//!   logged-out viewer still sees the platform-wide ranked stream.
//! - **Per-viewer** `UserHotSpace` rows keyed by `Partition::User(viewer_id)`
//!   on a `UHS#{viewer_id}` GSI partition — written for every viewer the
//!   author is reachable from (followers + team members + author themself).
//!   Logged-in callers read this stream so they only see spaces fanned out
//!   to them.
//!
//! Both rows share the same `WindowedRankKey` encoding so client ordering
//! stays identical across the two paths.

use crate::common::*;
#[cfg(feature = "server")]
use crate::features::auth::OptionalUser;
use crate::features::spaces::space_common::models::{
    HOT_SPACE_RANK_PK, HotSpace, HotSpaceHeat, UserHotSpace, user_hot_space_rank_pk,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct HotSpaceResponse {
    pub space_id: SpacePartition,
    pub post_id: FeedPartition,
    pub title: String,
    pub description: String,
    pub logo: String,
    pub author_display_name: String,
    pub participants: i64,
    pub rewards: i64,
    pub poll_count: i64,
    pub discussion_count: i64,
    pub quiz_count: i64,
    pub follow_count: i64,
    pub total_actions: i64,
    pub heat: HotSpaceHeat,
    pub rank: i64,
    pub created_at: i64,
}

#[get("/api/home/hot-spaces?bookmark", user: OptionalUser)]
pub async fn list_hot_spaces_handler(
    bookmark: Option<String>,
) -> Result<ListResponse<HotSpaceResponse>> {
    let conf = crate::common::config::ServerConfig::default();
    let cli = conf.dynamodb();

    // Logged-in viewers see their personal stream (UserHotSpace rows
    // fanned out to them); anonymous callers fall back to the global
    // HotSpace stream so the home page still has content for visitors.
    let items: Vec<HotSpaceResponse>;
    let next_bookmark: Option<String>;

    if let Some(user) = user.0 {
        let viewer_id = match &user.pk {
            Partition::User(id) => id.clone(),
            other => other.to_string(),
        };
        let opts = UserHotSpace::opt_with_bookmark(bookmark).limit(20);
        let (rows, nb) =
            UserHotSpace::find_by_viewer(cli, user_hot_space_rank_pk(&viewer_id), opts).await?;
        items = rows
            .into_iter()
            .enumerate()
            .map(|(idx, h)| HotSpaceResponse {
                space_id: h.space_pk.into(),
                post_id: h.post_pk.into(),
                title: h.title,
                description: h.description,
                logo: h.logo,
                author_display_name: h.author_display_name,
                participants: h.participants,
                rewards: h.rewards,
                poll_count: h.poll_count,
                discussion_count: h.discussion_count,
                quiz_count: h.quiz_count,
                follow_count: h.follow_count,
                total_actions: h.total_actions,
                heat: h.heat,
                rank: idx as i64 + 1,
                created_at: h.space_created_at,
            })
            .collect();
        next_bookmark = nb;
    } else {
        let opts = HotSpace::opt_with_bookmark(bookmark).limit(20);
        let (rows, nb) =
            HotSpace::find_by_rank(cli, HOT_SPACE_RANK_PK.to_string(), opts).await?;
        items = rows
            .into_iter()
            .enumerate()
            .map(|(idx, h)| HotSpaceResponse {
                space_id: h.pk.into(),
                post_id: h.post_pk.into(),
                title: h.title,
                description: h.description,
                logo: h.logo,
                author_display_name: h.author_display_name,
                participants: h.participants,
                rewards: h.rewards,
                poll_count: h.poll_count,
                discussion_count: h.discussion_count,
                quiz_count: h.quiz_count,
                follow_count: h.follow_count,
                total_actions: h.total_actions,
                heat: h.heat,
                rank: idx as i64 + 1,
                created_at: h.space_created_at,
            })
            .collect();
        next_bookmark = nb;
    }

    Ok((items, next_bookmark).into())
}
