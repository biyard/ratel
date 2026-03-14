use crate::common::SpacePartition;
use dioxus::prelude::*;

#[get("/api/spaces/:space_id/dashboard", space: crate::common::models::space::SpaceCommon)]
pub async fn list_dashboard_data_handler(
    space_id: SpacePartition,
) -> Result<Vec<crate::features::spaces::space_common::types::dashboard::DashboardComponentData>> {
    use crate::features::spaces::space_common::models::dashboard::aggregate::DashboardAggregate;
    use crate::features::spaces::space_common::models::dashboard::recalculate::build_dashboard_components;
    use crate::features::spaces::space_common::models::space_reward::SpaceReward;
    use crate::features::spaces::space_common::types::dashboard::*;

    let cfg = crate::features::spaces::pages::dashboard::config::get();
    let cli = cfg.dynamodb();
    let space_pk: crate::common::Partition = space_id.clone().into();

    let agg = DashboardAggregate::get_or_default(cli, &space_pk).await?;

    let mut components = build_dashboard_components(&agg, space.participants);

    // InfoCard: populate with SpaceReward data
    let rewards = SpaceReward::list_by_action(cli, space_id.clone(), None)
        .await
        .unwrap_or_default();

    let reward_items: Vec<InfoCardItem> = rewards
        .iter()
        .map(|r| InfoCardItem {
            label: r.description.clone(),
            value: format!("{}", r.get_amount()),
        })
        .collect();

    if !reward_items.is_empty() {
        let total_points: i64 = rewards.iter().map(|r| r.total_points).sum();

        components.push(DashboardComponentData::InfoCard(InfoCardData {
            icon: DashboardIcon::Rewards,
            total_points,
            items: reward_items,
        }));
    }

    // StatCard: only include when IncentivePool app is installed
    // let (app_pk, app_sk) =
    //     crate::features::spaces::pages::apps::SpaceApp::keys(&space_pk, crate::features::spaces::pages::apps::SpaceAppType::IncentivePool);
    // FIXME: We Need to chekc SpaceIncentive It self, not Space App,
    // let has_incentive = crate::features::spaces::pages::apps::SpaceApp::get(cli, &app_pk, Some(&app_sk))
    //     .await
    //     .map(|opt| opt.is_some())
    //     .unwrap_or(false);
    // Read Incentive Pool Data
    // let incentive_pool = space_incentive::SpaceIncentive::get_or_default(cli, &space_pk).await?;

    // if has_incentive {
    //     components.push(DashboardComponentData::StatCard(StatCardData {
    //         icon: DashboardIcon::IncentivePool,
    //         value: "0".to_string(),
    //         trend: 0.0,
    //         trend_label: String::new(),
    //         total_winners: "0".to_string(),
    //         rank_rate: String::new(),
    //         incentive_pool: String::new(),
    //     }));
    // }

    Ok(components)
}
