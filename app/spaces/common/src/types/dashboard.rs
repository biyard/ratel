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
    pub fn emoji(&self) -> &str {
        match self {
            DashboardIcon::Action => "\u{1F4A1}",       // 💡
            DashboardIcon::Participants => "\u{1F465}",  // 👥
            DashboardIcon::IncentivePool => "\u{1F4B0}", // 💰
            DashboardIcon::Rewards => "\u{1F3C6}",       // 🏆
            DashboardIcon::BarChart => "\u{1F4CA}",      // 📊
        }
    }

    pub fn bg_class(&self) -> &str {
        match self {
            DashboardIcon::Action => "bg-yellow-500",
            DashboardIcon::Participants => "bg-cyan-500",
            DashboardIcon::IncentivePool => "bg-violet-500",
            DashboardIcon::Rewards => "bg-green-500",
            DashboardIcon::BarChart => "bg-blue-500",
        }
    }

    pub fn bg_hex(&self) -> &str {
        match self {
            DashboardIcon::Action => "#FCB300",
            DashboardIcon::Participants => "#06B6D4",
            DashboardIcon::IncentivePool => "#A855F7",
            DashboardIcon::Rewards => "#22C55E",
            DashboardIcon::BarChart => "#3B82F6",
        }
    }
}

// ─── Dashboard Extension (display type) ─────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DashboardExtension {
    pub id: String,
    pub data: DashboardComponentData,
}

impl DashboardExtension {
    pub fn grid_size(&self) -> (u8, u8) {
        match &self.data {
            DashboardComponentData::StatSummary(_) => (1, 2),
            DashboardComponentData::StatCard(_) => (1, 3),
            DashboardComponentData::ProgressList(_) => (1, 4),
            DashboardComponentData::TabChart(_) => (1, 4),
            DashboardComponentData::InfoCard(_) => (1, 2),
            DashboardComponentData::RankingTable(_) => (4, 3),
        }
    }

    pub fn order(&self) -> i32 {
        match &self.data {
            DashboardComponentData::StatSummary(_) => 1,
            DashboardComponentData::ProgressList(_) => 2,
            DashboardComponentData::TabChart(_) => 3,
            DashboardComponentData::InfoCard(_) => 4,
            DashboardComponentData::StatCard(_) => 5,
            DashboardComponentData::RankingTable(_) => 6,
        }
    }

    pub fn is_card(&self) -> bool {
        !matches!(&self.data, DashboardComponentData::RankingTable(_))
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
    pub main_value: String,
    pub main_label: String,
    pub items: Vec<StatSummaryItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatSummaryItem {
    pub label: String,
    pub value: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub trend: f64,
    #[serde(default)]
    pub trend_label: String,
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
    pub main_value: String,
    pub items: Vec<ProgressListItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgressListItem {
    pub label: String,
    pub current: f64,
    pub total: f64,
    #[serde(default = "default_color")]
    pub color: String,
}

fn default_color() -> String {
    "#3B82F6".to_string()
}

// ─── Tab Chart ──────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TabChartData {
    pub icon: DashboardIcon,
    pub main_value: String,
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
    pub main_value: String,
    #[serde(default)]
    pub unit: String,
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

// ─── Fixed Extension IDs ──────────────────────────────────

pub const EXT_ID_TAB_CHART: &str = "tab-chart-participants";
pub const EXT_ID_PROGRESS_LIST: &str = "progress-list-actions";
pub const EXT_ID_INFO_CARD: &str = "info-card-rewards";
pub const EXT_ID_STAT_CARD: &str = "stat-card-incentive";
pub const EXT_ID_STAT_SUMMARY: &str = "stat-summary-space-views";
pub const EXT_ID_RANKING_TABLE: &str = "ranking-table-incentive";
