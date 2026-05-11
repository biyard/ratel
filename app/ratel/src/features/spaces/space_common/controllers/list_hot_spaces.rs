//! Read-only Hot Spaces feed.
//!
//! Every viewer (anonymous + logged-in) sees the same global ranked stream.
//! Ranking work happens in `services::space_fanout`, which writes one
//! `HotSpace` row per eligible space (Public+Published with at least
//! `MIN_PARTICIPANTS_FOR_HOT` participants) on the `HSR#ALL` GSI partition,
//! sorted by `WindowedRankKey`.
//!
//! This handler is a single GSI query against that index. No timeline
//! gating, no per-viewer fanout, no in-memory sort.

use crate::common::*;
#[cfg(feature = "server")]
use crate::features::auth::OptionalUser;
use crate::features::spaces::space_common::models::{HOT_SPACE_RANK_PK, HotSpace, HotSpaceHeat};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
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

#[get("/api/hot-spaces")]
pub async fn list_hot_spaces_handler() -> Result<ListResponse<HotSpaceResponse>> {
    debug!("Listing hot spaces");
    let conf = crate::common::config::ServerConfig::default();
    let cli = conf.dynamodb();

    let opts = HotSpace::opt().limit(20);
    let (rows, next_bookmark) =
        HotSpace::find_by_rank(cli, HOT_SPACE_RANK_PK.to_string(), opts).await?;

    let items: Vec<HotSpaceResponse> = rows
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

    Ok((items, next_bookmark).into())
}
