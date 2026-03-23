use super::aggregate::DashboardAggregate;
use crate::features::spaces::space_common::types::dashboard::*;

/// Counts of actions per type, used to build per-type progress bars on the dashboard.
#[derive(Debug, Clone, Default)]
pub struct ActionTypeCounts {
    pub poll_count: i64,
    pub discussion_count: i64,
    pub quiz_count: i64,
    pub follow_count: i64,
}

/// Pure function: aggregate → base dashboard components.
/// InfoCard (with reward items), StatCard, and RankingTable are added by the caller.
pub fn build_dashboard_components(
    agg: &DashboardAggregate,
    participants: i64,
    action_counts: ActionTypeCounts,
) -> Vec<DashboardComponentData> {
    let total_actions =
        action_counts.poll_count + action_counts.discussion_count + action_counts.quiz_count + action_counts.follow_count;

    vec![
        DashboardComponentData::StatSummary(StatSummaryData {
            icon: DashboardIcon::BarChart,
            participants,
            likes: agg.like_count,
            comments: agg.comment_count,
            total_actions,
        }),
        DashboardComponentData::ProgressList(ProgressListData {
            icon: DashboardIcon::Action,
            poll_count: action_counts.poll_count,
            post_count: action_counts.discussion_count,
            quiz_count: action_counts.quiz_count,
            follow_count: action_counts.follow_count,
        }),
        DashboardComponentData::TabChart(TabChartData {
            icon: DashboardIcon::Participants,
            participants,
            tabs: vec![],
        }),
    ]
}
