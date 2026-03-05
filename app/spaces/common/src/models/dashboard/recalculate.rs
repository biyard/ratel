use super::aggregate::DashboardAggregate;
use crate::types::dashboard::*;

/// Pure function: aggregate row → Vec<DashboardComponentData>.
/// Used by the read endpoint to build dashboard cards on-the-fly.
pub fn build_dashboard_components(
    agg: &DashboardAggregate,
    participants: i64,
) -> Vec<DashboardComponentData> {
    vec![
        DashboardComponentData::StatSummary(StatSummaryData {
            icon: DashboardIcon::BarChart,
            participants,
            likes: agg.like_count,
            comments: agg.comment_count,
            total_actions: agg.poll_count + agg.post_count,
        }),
        DashboardComponentData::ProgressList(ProgressListData {
            icon: DashboardIcon::Action,
            poll_count: agg.poll_count,
            post_count: agg.post_count,
        }),
        DashboardComponentData::TabChart(TabChartData {
            icon: DashboardIcon::Participants,
            participants,
            //FIXME: Add Panel Info
            tabs: vec![],
        }),
        DashboardComponentData::InfoCard(InfoCardData {
            icon: DashboardIcon::Rewards,
            total_points: agg.total_points,
            //FIXME: Parse SpaceRewards
            items: vec![],
        }),
        DashboardComponentData::StatCard(StatCardData {
            icon: DashboardIcon::IncentivePool,
            value: "0".to_string(),
            trend: 0.0,
            trend_label: String::new(),
            total_winners: "0".to_string(),
            rank_rate: String::new(),
            incentive_pool: String::new(),
        }),
        DashboardComponentData::RankingTable(RankingTableData {
            page_size: 10,
            entries: vec![],
        }),
    ]
}
