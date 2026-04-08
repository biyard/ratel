use crate::features::spaces::pages::actions::actions::discussion::*;
use crate::features::spaces::pages::actions::models::SpaceAction;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
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

#[mcp_tool(name = "update_discussion", description = "Update a discussion (title, html_contents, category_name, started_at, ended_at). Requires creator role.")]
#[patch("/api/spaces/{space_id}/discussions/{discussion_sk}", role: SpaceUserRole)]
pub async fn update_discussion(
    #[mcp(description = "Space partition key")]
    space_id: SpacePartition,
    #[mcp(description = "Discussion sort key (e.g. 'SpacePost#<uuid>')")]
    discussion_sk: SpacePostEntityType,
    #[mcp(description = "Discussion update data as JSON. Fields: title, html_contents, category_name, started_at, ended_at (all optional)")]
    req: UpdateDiscussionRequest,
) -> Result<SpacePost> {
    SpacePost::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.clone().into();
    let discussion_sk_entity: EntityType = discussion_sk.clone().into();

    let now = crate::common::utils::time::get_now_timestamp_millis();
    let mut updater = SpacePost::updater(&space_pk, &discussion_sk_entity).with_updated_at(now);

    let action_pk = CompositePartition::<SpacePartition, String>(
        space_id.clone(),
        discussion_sk.to_string(),
    );
    let mut action_updater =
        SpaceAction::updater(&action_pk, &EntityType::SpaceAction).with_updated_at(now);
    let mut update_action = false;

    if let Some(title) = req.title {
        updater = updater.with_title(title.clone());
        action_updater = action_updater.with_title(title);
        update_action = true;
    }
    if let Some(html_contents) = req.html_contents {
        updater = updater.with_html_contents(html_contents.clone());
        action_updater = action_updater.with_description(html_contents);
        update_action = true;
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

    if update_action {
        action_updater.execute(cli).await?;
    }

    if let Some(category_name) = req.category_name {
        if !category_name.is_empty() {
            let cat = SpaceCategory::new(space_id, category_name.clone());
            let _ = cat.upsert(cli).await;
        }
    }

    Ok(post)
}
