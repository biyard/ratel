use crate::common::ListResponse;
use crate::features::spaces::pages::actions::actions::follow::SpaceFollowUser;
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::features::spaces::pages::apps::models::SpaceApp;

/// One follow target a creator has registered for the space's follow
/// campaign. Each target maps directly to one filter chip in the
/// analyze cross-filter — there is no item / question hierarchy for
/// follow, just a flat list of targets.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(aide::OperationIo, schemars::JsonSchema))]
pub struct AnalyzeFollowItem {
    pub user_pk: Partition,
    pub display_name: String,
    pub username: String,
    pub profile_url: String,
}

#[get("/api/spaces/{space_id}/apps/analyzes/follows?bookmark", role: SpaceUserRole)]
pub async fn list_analyze_follows(
    space_id: SpacePartition,
    bookmark: Option<String>,
) -> Result<ListResponse<AnalyzeFollowItem>> {
    SpaceApp::can_edit(role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.into();

    let mut opt = SpaceFollowUser::opt()
        .sk(EntityType::SpaceSubscriptionUser(String::default()).to_string())
        .limit(20);
    if let Some(bookmark) = bookmark {
        opt = opt.bookmark(bookmark);
    }

    let (users, bookmark) = SpaceFollowUser::query(cli, space_pk, opt).await?;

    let items = users
        .into_iter()
        .filter(|u| u.user_pk != Partition::None)
        .map(|u| AnalyzeFollowItem {
            user_pk: u.user_pk,
            display_name: u.display_name,
            username: u.username,
            profile_url: u.profile_url,
        })
        .collect();

    Ok(ListResponse { items, bookmark })
}
