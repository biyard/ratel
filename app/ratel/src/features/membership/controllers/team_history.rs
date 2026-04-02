use super::*;
use crate::features::auth::User;
use crate::features::membership::controllers::history::PurchaseHistoryItem;
use crate::features::membership::models::TeamPurchase;
use crate::features::membership::*;
use crate::features::posts::models::Team;
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};

#[get("/v3/teams/:username/memberships/history?bookmark", user: User, team: Team, permissions: TeamGroupPermissions)]
pub async fn get_team_purchase_history_handler(
    username: String,
    bookmark: Option<String>,
) -> Result<ListResponse<PurchaseHistoryItem>> {
    use crate::features::membership::models::TeamPurchaseQueryOption;

    if !permissions.contains(TeamGroupPermission::TeamAdmin) {
        return Err(Error::NotFound("Permission denied".to_string()));
    }

    let conf = crate::features::membership::config::get();
    let cli = conf.common.dynamodb();

    let mut opt = TeamPurchaseQueryOption::builder()
        .limit(20)
        .scan_index_forward(false);

    if let Some(bookmark_str) = bookmark {
        opt = opt.bookmark(bookmark_str);
    }

    let (purchases, last_key) = TeamPurchase::find_by_team(
        cli,
        CompositePartition::team_purchase_pk(team.pk.clone().into()),
        opt,
    )
    .await?;

    let items: Vec<PurchaseHistoryItem> = purchases.into_iter().map(Into::into).collect();

    Ok(ListResponse {
        items,
        bookmark: last_key,
    })
}
