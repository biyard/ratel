use crate::common::ListResponse;
use crate::features::spaces::pages::actions::actions::discussion::SpacePost;
use crate::features::spaces::pages::actions::models::SpaceAction;
use crate::features::spaces::pages::actions::types::SpaceActionType;
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::features::spaces::pages::apps::models::SpaceApp;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(aide::OperationIo, schemars::JsonSchema))]
pub struct AnalyzeDiscussionItem {
    pub discussion_id: SpacePostEntityType,
    pub title: String,
}

#[get("/api/spaces/{space_id}/apps/analyzes/discussions?bookmark", role: SpaceUserRole)]
pub async fn list_analyze_discussions(
    space_id: SpacePartition,
    bookmark: Option<String>,
) -> Result<ListResponse<AnalyzeDiscussionItem>> {
    SpaceApp::can_edit(role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.into();

    let mut items = Vec::new();
    let mut next_bookmark = bookmark;
    let page_limit = 50;

    loop {
        let opt = SpaceAction::opt_with_bookmark(next_bookmark.clone())
            .scan_index_forward(false)
            .limit(page_limit);
        let (actions, bookmark) = SpaceAction::find_by_space(cli, &space_pk, opt).await?;

        for action in actions {
            if action.space_action_type != SpaceActionType::TopicDiscussion {
                continue;
            }

            let discussion_id: SpacePostEntityType = action.pk.1.clone().into();
            let discussion_sk: EntityType = discussion_id.clone().into();
            let post = SpacePost::get(cli, &space_pk, Some(&discussion_sk)).await?;

            let title = post
                .as_ref()
                .map(|post| post.title.trim().to_string())
                .filter(|title| !title.is_empty())
                .or_else(|| {
                    let title = action.title.trim().to_string();
                    (!title.is_empty()).then_some(title)
                })
                .unwrap_or_default();

            items.push(AnalyzeDiscussionItem {
                discussion_id,
                title,
            });

            if items.len() >= 20 {
                return Ok(ListResponse { items, bookmark });
            }
        }

        if bookmark.is_none() {
            return Ok(ListResponse { items, bookmark });
        }

        next_bookmark = bookmark;
    }
}
