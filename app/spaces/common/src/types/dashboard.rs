use serde::{Deserialize, Serialize};

// ─── Dashboard Icon ──────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DashboardIcon {
    #[default]
    Action,
    Participants,
    IncentivePool,
    Rewards,
    BarChart,
}

impl DashboardIcon {
    pub fn class(&self) -> &str {
        match self {
            DashboardIcon::Action => "bg-yellow-500",
            DashboardIcon::Participants => "bg-cyan-500",
            DashboardIcon::IncentivePool => "bg-violet-500",
            DashboardIcon::Rewards => "bg-green-500",
            DashboardIcon::BarChart => "bg-blue-500",
        }
    }
}

// ─── Component Data ───────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DashboardComponentData {
    StatCard(StatCardData),
    StatSummary(StatSummaryData),
    ProgressList(ProgressListData),
    TabChart(TabChartData),
    InfoCard(InfoCardData),
    RankingTable(RankingTableData),
}

// ─── Stat Summary ─────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatSummaryData {
    pub icon: DashboardIcon,
    pub participants: i64,
    pub likes: i64,
    pub comments: i64,
    pub total_actions: i64,
}

// ─── Stat Card ────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatCardData {
    pub icon: DashboardIcon,
    pub value: String,
    pub trend: f64,
    #[serde(default)]
    pub trend_label: String,
    #[serde(default)]
    pub total_winners: String,
    #[serde(default)]
    pub rank_rate: String,
    #[serde(default)]
    pub incentive_pool: String,
}

// ─── Progress List ──────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgressListData {
    pub icon: DashboardIcon,
    pub poll_count: i64,
    pub post_count: i64,
}

fn default_color() -> String {
    "#3B82F6".to_string()
}

// ─── Tab Chart ──────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TabChartData {
    pub icon: DashboardIcon,
    pub participants: i64,
    pub tabs: Vec<TabChartTab>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TabChartTab {
    pub label: String,
    pub categories: Vec<TabChartCategory>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TabChartCategory {
    pub name: String,
    pub value: f64,
    #[serde(default)]
    pub percentage: String,
    #[serde(default = "default_color")]
    pub color: String,
}

// ─── Info Card ────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfoCardData {
    pub icon: DashboardIcon,
    pub total_points: i64,
    pub items: Vec<InfoCardItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfoCardItem {
    pub label: String,
    pub value: String,
}

// ─── Ranking Table ──────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RankingTableData {
    pub entries: Vec<RankingEntry>,
    #[serde(default = "default_page_size")]
    pub page_size: usize,
}

fn default_page_size() -> usize {
    10
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RankingEntry {
    pub rank: u32,
    pub name: String,
    #[serde(default)]
    pub avatar: String,
    pub score: f64,
    #[serde(default)]
    pub change: i32,
}

