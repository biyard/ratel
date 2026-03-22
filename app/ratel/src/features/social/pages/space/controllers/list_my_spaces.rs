use std::collections::HashMap;

use super::super::*;
use crate::common::models::space::{SpaceCommon, SpaceParticipant};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct MySpaceResponse {
    pub space_pk: Partition,
    pub post_pk: Partition,
    pub title: String,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub created_at: i64,
    pub visibility: SpaceVisibility,
    pub status: Option<SpaceStatus>,
    pub logo: String,
    pub participants: i64,
}

#[get("/api/me/spaces?bookmark&active_only", user: crate::features::auth::User)]
pub async fn list_my_spaces_handler(
    bookmark: Option<String>,
    active_only: Option<bool>,
) -> Result<ListResponse<MySpaceResponse>> {
    let conf = crate::common::config::ServerConfig::default();
    let cli = conf.dynamodb();
    let filter_active = active_only.unwrap_or(false);
    let page_limit: i32 = 10;

    // When active_only is true, keep fetching participant pages server-side
    // until we collect up to `page_limit` active spaces or exhaust all pages.
    // A hard cap on pages scanned prevents unbounded DynamoDB reads for users
    // with many inactive participations.
    let max_pages: i32 = 5;
    let mut pages_scanned: i32 = 0;
    let mut collected_spaces: Vec<SpaceCommon> = Vec::new();
    let mut current_bookmark = bookmark;
    let final_bookmark;

    loop {
        pages_scanned += 1;
        let mut opt = SpaceParticipant::opt().limit(page_limit);
        if let Some(ref bm) = current_bookmark {
            opt = opt.bookmark(bm.clone());
        }

        let (participants, next_bookmark) =
            SpaceParticipant::find_by_user(cli, &user.pk, opt).await?;

        let space_keys: Vec<(Partition, EntityType)> = participants
            .iter()
            .map(|sp| (sp.space_pk.clone(), EntityType::SpaceCommon))
            .collect();

        let spaces: Vec<SpaceCommon> = if space_keys.is_empty() {
            vec![]
        } else {
            SpaceCommon::batch_get(cli, space_keys).await?
        };

        if filter_active {
            // In active_only mode, include all active spaces from the current page.
            // This may cause the number of returned items to exceed page_limit,
            // but it avoids skipping active spaces that appear later in the page
            // (pagination data loss when final_bookmark advances past them).
            collected_spaces.extend(spaces.into_iter().filter(|s| s.is_active()));
        } else {
            collected_spaces.extend(spaces);
        }

        // Stop if we have enough items or there are no more pages
        if collected_spaces.len() >= page_limit as usize || next_bookmark.is_none() {
            final_bookmark = next_bookmark;
            break;
        }

        // If not filtering, one page is sufficient
        if !filter_active {
            final_bookmark = next_bookmark;
            break;
        }

        // Hard cap: stop scanning after max_pages to bound DynamoDB reads.
        // Return the real next_bookmark so callers can continue scanning if
        // they choose, instead of permanently hiding the section when there
        // are more active spaces beyond the scan cap.
        if pages_scanned >= max_pages {
            final_bookmark = next_bookmark;
            break;
        }

        current_bookmark = next_bookmark;
    }

    let post_keys: Vec<(Partition, EntityType)> = collected_spaces
        .iter()
        .filter_map(|s| s.pk.clone().to_post_key().ok())
        .map(|pk| (pk, EntityType::Post))
        .collect();

    let posts: Vec<crate::features::posts::models::Post> = if post_keys.is_empty() {
        vec![]
    } else {
        crate::features::posts::models::Post::batch_get(cli, post_keys).await?
    };

    // Build a HashMap for O(1) title lookups instead of O(spaces * posts) linear scans
    let title_map: HashMap<String, String> = posts
        .into_iter()
        .map(|p| (p.pk.to_string(), p.title))
        .collect();

    let items: Vec<MySpaceResponse> = collected_spaces
        .into_iter()
        .map(|space| {
            let title = space
                .pk
                .clone()
                .to_post_key()
                .ok()
                .and_then(|post_pk| title_map.get(&post_pk.to_string()).cloned())
                .unwrap_or_default();

            MySpaceResponse {
                space_pk: space.pk.clone(),
                post_pk: space.post_pk,
                title,
                author_display_name: space.author_display_name,
                author_profile_url: space.author_profile_url,
                created_at: space.created_at,
                visibility: space.visibility,
                status: space.status,
                logo: space.logo,
                participants: space.participants,
            }
        })
        .collect();

    Ok(ListResponse {
        items,
        bookmark: final_bookmark,
    })
}
