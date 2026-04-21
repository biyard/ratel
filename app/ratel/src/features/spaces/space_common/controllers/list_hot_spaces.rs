// TODO(hot-spaces-ranking): The current handler fetches up to 50 public spaces
// and sorts them by an in-request `activity_score`, then calls `count_actions`
// for each space (N+1 DynamoDB queries). This breaks once the total number of
// public spaces exceeds the fetch window, and latency grows linearly with it.
//
// Superseded design: `SpaceHotScore` entity on GSI8 with pre-computed scores
// maintained by EventBridge, read path is one GSI query + parallel batch_gets.
// Plan: docs/superpowers/plans/2026-04-21-hot-spaces-ranking.md

use crate::common::models::space::SpaceCommon;
use crate::common::*;
#[cfg(feature = "server")]
use crate::features::posts::models::Post;
#[cfg(feature = "server")]
use crate::features::spaces::pages::actions::models::SpaceAction;
#[cfg(feature = "server")]
use crate::features::spaces::pages::actions::types::SpaceActionType;
#[cfg(feature = "server")]
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum HotSpaceHeat {
    Blazing,
    Trending,
    Rising,
}

impl Default for HotSpaceHeat {
    fn default() -> Self {
        HotSpaceHeat::Rising
    }
}

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

#[get("/api/home/hot-spaces?bookmark")]
pub async fn list_hot_spaces_handler(
    bookmark: Option<String>,
) -> Result<ListResponse<HotSpaceResponse>> {
    let conf = crate::common::config::ServerConfig::default();
    let cli = conf.dynamodb();

    // Fetch a wider window than the page size so the activity-score ranking
    // reflects the top spaces globally rather than only the page slice. The
    // bookmark still paginates through the scored list on the client side by
    // echoing the raw DynamoDB bookmark.
    let opts = SpaceCommon::opt_with_bookmark(bookmark).limit(50);
    let visibility_pk = format!(
        "{}#{}",
        SpacePublishState::Published,
        SpaceVisibility::Public
    );
    let (spaces, next_bookmark) =
        SpaceCommon::find_by_visibility(cli, visibility_pk, opts).await?;

    // Fetch post titles for spaces whose content is empty
    let post_keys: Vec<(Partition, EntityType)> = spaces
        .iter()
        .filter_map(|s| s.pk.clone().to_post_key().ok())
        .map(|pk| (pk, EntityType::Post))
        .collect();

    let posts: Vec<Post> = if post_keys.is_empty() {
        vec![]
    } else {
        Post::batch_get(cli, post_keys).await.unwrap_or_default()
    };

    let title_map: HashMap<String, String> = posts
        .iter()
        .map(|p| (p.pk.to_string(), p.title.clone()))
        .collect();
    let desc_map: HashMap<String, String> = posts
        .iter()
        .map(|p| (p.pk.to_string(), extract_description(&p.html_contents)))
        .collect();

    let mut items: Vec<HotSpaceResponse> = Vec::with_capacity(spaces.len());
    for space in spaces.into_iter() {
        let post_pk = space.pk.clone().to_post_key().ok();
        let post_pk_str = post_pk.as_ref().map(|p| p.to_string()).unwrap_or_default();

        let title = title_map.get(&post_pk_str).cloned().unwrap_or_default();
        let description = if !space.content.is_empty() {
            extract_description(&space.content)
        } else {
            desc_map.get(&post_pk_str).cloned().unwrap_or_default()
        };

        let (poll_count, discussion_count, quiz_count, follow_count) =
            count_actions(cli, &space.pk).await;
        let total_actions = poll_count + discussion_count + quiz_count + follow_count;

        let heat = derive_heat(space.participants);

        items.push(HotSpaceResponse {
            space_id: space.pk.clone().into(),
            post_id: post_pk.unwrap_or_default().into(),
            title,
            description,
            logo: space.logo,
            author_display_name: space.author_display_name,
            participants: space.participants,
            rewards: space.rewards.unwrap_or(0),
            poll_count,
            discussion_count,
            quiz_count,
            follow_count,
            total_actions,
            heat,
            rank: 0,
            created_at: space.created_at,
        });
    }

    // Rank by activity score so the carousel surfaces spaces with real
    // engagement, not just whichever spaces DynamoDB returned first.
    let now_ms = crate::common::utils::time::get_now_timestamp_millis();
    items.sort_by(|a, b| {
        activity_score(b, now_ms)
            .partial_cmp(&activity_score(a, now_ms))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    items.truncate(10);
    for (idx, item) in items.iter_mut().enumerate() {
        item.rank = idx as i64 + 1;
    }

    Ok((items, next_bookmark).into())
}

/// Activity score combining cumulative participation, action richness, and a
/// freshness bonus for recently created spaces. Tuned so a 10k-participant
/// established space still ranks above a brand-new empty one, while a fresh
/// space with a handful of actions can outrank a stagnant large space.
fn activity_score(item: &HotSpaceResponse, now_ms: i64) -> f64 {
    let participants = (item.participants.max(0) as f64).ln_1p();
    let actions = item.total_actions.max(0) as f64;
    let age_days = ((now_ms - item.created_at).max(0) as f64) / (1000.0 * 60.0 * 60.0 * 24.0);
    // Half-life of ~14 days: e^(-age/14). Stays close to 1 for the first week,
    // falls below 0.5 after two weeks, and becomes negligible after ~2 months.
    let freshness = (-age_days / 14.0).exp();

    participants * 3.0 + actions * 2.0 + freshness * 5.0
}

#[cfg(feature = "server")]
async fn count_actions(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
) -> (i64, i64, i64, i64) {
    let opts = SpaceAction::opt_all();
    let actions = match SpaceAction::find_by_space(cli, space_pk.clone(), opts).await {
        Ok((actions, _)) => actions,
        Err(_) => return (0, 0, 0, 0),
    };

    let mut polls = 0i64;
    let mut discussions = 0i64;
    let mut quizzes = 0i64;
    let mut follows = 0i64;
    for a in actions {
        match a.space_action_type {
            SpaceActionType::Poll => polls += 1,
            SpaceActionType::TopicDiscussion => discussions += 1,
            SpaceActionType::Quiz => quizzes += 1,
            SpaceActionType::Follow => follows += 1,
        }
    }
    (polls, discussions, quizzes, follows)
}

fn derive_heat(participants: i64) -> HotSpaceHeat {
    if participants >= 5_000 {
        HotSpaceHeat::Blazing
    } else if participants >= 500 {
        HotSpaceHeat::Trending
    } else {
        HotSpaceHeat::Rising
    }
}

#[cfg(feature = "server")]
fn extract_description(html: &str) -> String {
    let re_img = regex::Regex::new(r"<img[^>]*>").unwrap();
    let without_images = re_img.replace_all(html, "");

    let re_tags = regex::Regex::new(r"<[^>]+>").unwrap();
    let without_tags = re_tags.replace_all(&without_images, "");

    let re_urls = regex::Regex::new(r"https?://[^\s]+").unwrap();
    let without_urls = re_urls.replace_all(&without_tags, "");

    let re_whitespace = regex::Regex::new(r"\s+").unwrap();
    re_whitespace
        .replace_all(&without_urls, " ")
        .trim()
        .to_string()
}
