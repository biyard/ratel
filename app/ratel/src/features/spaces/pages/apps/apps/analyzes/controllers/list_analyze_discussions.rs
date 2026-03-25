use crate::common::ListResponse;
use crate::features::spaces::pages::actions::actions::discussion::SpacePost;
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
    let opt = SpacePost::opt_with_bookmark(bookmark)
        .scan_index_forward(false)
        .limit(20);
    let (posts, bookmark) = SpacePost::find_by_space_ordered(cli, space_pk, opt).await?;

    let items = posts
        .into_iter()
        .map(|post| AnalyzeDiscussionItem {
            discussion_id: post.sk.clone().into(),
            title: post.title.trim().to_string(),
        })
        .collect();

    Ok(ListResponse { items, bookmark })
}
