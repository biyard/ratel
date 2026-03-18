use crate::common::ListResponse;
use crate::features::spaces::pages::actions::actions::poll::SpacePoll;
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::features::spaces::pages::apps::models::SpaceApp;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(aide::OperationIo, schemars::JsonSchema))]
pub struct AnalyzePollItem {
    pub poll_id: SpacePollEntityType,
    pub questions_count: usize,
    pub default: bool,
}

#[get("/api/spaces/{space_id}/apps/analyzes/polls?bookmark", role: SpaceUserRole)]
pub async fn list_analyze_polls(
    space_id: SpacePartition,
    bookmark: Option<String>,
) -> Result<ListResponse<AnalyzePollItem>> {
    SpaceApp::can_edit(role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.into();

    let mut opt = SpacePoll::opt()
        .sk(EntityType::SpacePoll(String::default()).to_string())
        .scan_index_forward(false)
        .limit(20);
    if let Some(bookmark) = bookmark {
        opt = opt.bookmark(bookmark);
    }

    let (polls, bookmark) = SpacePoll::query(cli, space_pk, opt).await?;
    let items = polls
        .into_iter()
        .map(|poll| AnalyzePollItem {
            poll_id: SpacePollEntityType::from(poll.sk.clone()),
            questions_count: poll.questions.len(),
            default: poll.is_default_poll(),
        })
        .collect();

    Ok(ListResponse { items, bookmark })
}
