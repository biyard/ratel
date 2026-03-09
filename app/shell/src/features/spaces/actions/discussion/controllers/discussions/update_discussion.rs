use crate::features::spaces::actions::discussion::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDiscussionRequest {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub html_contents: Option<String>,
    #[serde(default)]
    pub category_name: Option<String>,
    #[serde(default)]
    pub started_at: Option<i64>,
    #[serde(default)]
    pub ended_at: Option<i64>,
}

#[patch("/api/spaces/{space_id}/discussions/{discussion_sk}", role: SpaceUserRole)]
pub async fn update_discussion(
    space_id: SpacePartition,
    discussion_sk: SpacePostEntityType,
    req: UpdateDiscussionRequest,
) -> Result<SpacePost> {
    SpacePost::can_edit(&role)?;
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.clone().into();
    let discussion_sk_entity: EntityType = discussion_sk.into();

    let now = common::utils::time::get_now_timestamp_millis();
    let mut updater = SpacePost::updater(&space_pk, &discussion_sk_entity).with_updated_at(now);

    if let Some(title) = req.title {
        updater = updater.with_title(title);
    }
    if let Some(html_contents) = req.html_contents {
        updater = updater.with_html_contents(html_contents);
    }
    if let Some(category_name) = &req.category_name {
        updater = updater.with_category_name(category_name.clone());
    }
    if let Some(started_at) = req.started_at {
        updater = updater.with_started_at(started_at);
    }
    if let Some(ended_at) = req.ended_at {
        updater = updater.with_ended_at(ended_at);
    }

    let post = updater.execute(cli).await?;

    if let Some(category_name) = req.category_name {
        if !category_name.is_empty() {
            let cat = SpaceCategory::new(space_id, category_name.clone());
            let _ = cat.upsert(cli).await;
        }
    }

    Ok(post)
}
