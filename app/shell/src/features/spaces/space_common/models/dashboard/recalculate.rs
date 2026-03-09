use super::aggregate::DashboardAggregate;
use crate::features::spaces::space_common::types::dashboard::*;

/// Pure function: aggregate → base dashboard components.
/// InfoCard (with reward items), StatCard, and RankingTable are added by the caller.
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
            tabs: vec![],
        }),
    ]
}
