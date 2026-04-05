use crate::common::utils::format::format_with_commas;
use crate::common::SpacePartition;
use dioxus::prelude::*;

#[get("/api/spaces/:space_id/dashboard", space: crate::common::models::space::SpaceCommon)]
pub async fn list_dashboard_data_handler(
    space_id: SpacePartition,
) -> Result<Vec<crate::features::spaces::space_common::types::dashboard::DashboardComponentData>> {
    use crate::features::spaces::space_common::models::dashboard::aggregate::DashboardAggregate;
    use crate::features::spaces::space_common::models::dashboard::recalculate::{
        build_dashboard_components, ActionTypeCounts,
    };
    use crate::features::spaces::space_common::models::space_reward::SpaceReward;
    use crate::features::spaces::space_common::types::dashboard::*;

    let cfg = crate::features::spaces::pages::dashboard::config::get();
    let cli = cfg.dynamodb();
    let space_pk: crate::common::Partition = space_id.clone().into();

    let agg = DashboardAggregate::get_or_default(cli, &space_pk).await?;

    // Query SpaceActions to get per-type counts
    let action_counts = {
        use crate::features::spaces::pages::actions::models::SpaceAction;
        use crate::features::spaces::pages::actions::types::SpaceActionType;

        let (actions, _) = SpaceAction::find_by_space(cli, &space_pk, SpaceAction::opt())
            .await
            .unwrap_or_default();

        let mut counts = ActionTypeCounts::default();
        for action in &actions {
            match action.space_action_type {
                SpaceActionType::Poll => counts.poll_count += 1,
                SpaceActionType::TopicDiscussion => counts.discussion_count += 1,
                SpaceActionType::Quiz => counts.quiz_count += 1,
                SpaceActionType::Follow => counts.follow_count += 1,
            }
        }
        counts
    };

    let mut components = build_dashboard_components(&agg, space.participants, action_counts);

    // InfoCard: populate with SpaceReward data
    let rewards = SpaceReward::list_by_action(cli, space_id.clone(), None)
        .await
        .unwrap_or_default();

    let reward_items: Vec<InfoCardItem> = rewards
        .iter()
        .map(|r| InfoCardItem {
            label: r.behavior.to_string(),
            description: if r.description.is_empty() {
                None
            } else {
                Some(r.description.clone())
            },
            value: format_with_commas(r.get_amount()),
        })
        .collect();

    if !reward_items.is_empty() {
        let total_points: i64 = rewards.iter().map(|r| r.get_amount()).sum();

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

    #[cfg(feature = "activity")]
    {
        use crate::features::activity::models::SpaceScore;
        use crate::features::spaces::space_common::types::dashboard::*;

        let score_opts = SpaceScore::opt().limit(50);

        if let Ok((scores, _)) = SpaceScore::find_by_space(cli, &space_pk, score_opts).await {
            if !scores.is_empty() {
                let entries: Vec<RankingEntry> = scores
                    .iter()
                    .enumerate()
                    .map(|(i, score)| RankingEntry {
                        rank: (i as u32) + 1,
                        name: score.user_name.clone(),
                        avatar: score.user_avatar.clone(),
                        score: score.total_score as f64,
                        change: 0,
                    })
                    .collect();

                components.push(DashboardComponentData::RankingTable(RankingTableData {
                    entries,
                    page_size: 10,
                }));
            }
        }
    }

    Ok(components)
}
