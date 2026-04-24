// TODO(hot-spaces-ranking): `count_actions` below is an N+1 DynamoDB scan.
// The denormalized-count follow-up (SpaceActionCount entity) in
// docs/superpowers/plans/2026-04-21-hot-spaces-ranking.md removes it.

use super::list_hot_spaces::{HotSpaceHeat, HotSpaceResponse};
use crate::common::*;
#[cfg(feature = "server")]
use crate::common::models::space::{SpaceCommon, SpaceParticipant};
use crate::features::auth::User;
#[cfg(feature = "server")]
use crate::features::posts::models::Post;
#[cfg(feature = "server")]
use crate::features::spaces::pages::actions::models::SpaceAction;
#[cfg(feature = "server")]
use crate::features::spaces::pages::actions::types::SpaceActionType;
#[cfg(feature = "server")]
use std::collections::HashMap;

#[get("/api/home/my-spaces?bookmark", user: User)]
pub async fn list_my_home_spaces_handler(
    bookmark: Option<String>,
) -> Result<ListResponse<HotSpaceResponse>> {
    let conf = crate::common::config::ServerConfig::default();
    let cli = conf.dynamodb();

    // SpaceParticipant GSI1 is sorted by created_at. Default scan_index_forward
    // is already descending, so the most recently joined spaces come first.
    let opts = SpaceParticipant::opt_with_bookmark(bookmark).limit(10);
    let (participants, next_bookmark) =
        SpaceParticipant::find_by_user(cli, &user.pk, opts).await?;

    let space_keys: Vec<(Partition, EntityType)> = participants
        .iter()
        .map(|sp| (sp.space_pk.clone(), EntityType::SpaceCommon))
        .collect();

    let fetched: Vec<SpaceCommon> = if space_keys.is_empty() {
        vec![]
    } else {
        SpaceCommon::batch_get(cli, space_keys).await?
    };

    // DynamoDB BatchGetItem does not preserve input order, so realign the
    // fetched spaces with the participant query order (most-recent-first).
    let mut by_pk: HashMap<String, SpaceCommon> = fetched
        .into_iter()
        .map(|s| (s.pk.to_string(), s))
        .collect();
    let spaces: Vec<SpaceCommon> = participants
        .iter()
        .filter_map(|sp| by_pk.remove(&sp.space_pk.to_string()))
        .filter(|s| s.is_published())
        .collect();

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
    for (idx, space) in spaces.into_iter().enumerate() {
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
            rank: idx as i64 + 1,
            created_at: space.created_at,
        });
    }

    Ok((items, next_bookmark).into())
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
            SpaceActionType::Meet => {}
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
