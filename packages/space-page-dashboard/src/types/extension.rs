use crate::*;
use serde::{Deserialize, Serialize};

// ─── Component Type ────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DashboardComponentType {
    StatCard,
    StatSummary,
    ProgressList,
    TabChart,
    InfoCard,
    RankingTable,
}

impl DashboardComponentType {
    pub fn default_grid_size(&self) -> (u8, u8) {
        match self {
            Self::StatSummary => (1, 2),
            Self::ProgressList => (1, 2),
            Self::TabChart => (1, 2),
            Self::InfoCard => (1, 1),
            Self::StatCard => (1, 1),
            Self::RankingTable => (4, 4),
        }
    }

    pub fn default_order(&self) -> i32 {
        match self {
            Self::StatSummary => 1,
            Self::ProgressList => 2,
            Self::TabChart => 3,
            Self::InfoCard => 4,
            Self::StatCard => 5,
            Self::RankingTable => 6,
        }
    }
}

// ─── Top-level Extension ────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DashboardExtension {
    pub id: String,
    pub component_type: DashboardComponentType,
    pub data: DashboardComponentData,
}

impl DashboardExtension {
    pub fn grid_size(&self) -> (u8, u8) {
        self.component_type.default_grid_size()
    }

    pub fn order(&self) -> i32 {
        self.component_type.default_order()
    }
}

// ─── Component Data ───────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DashboardComponentData {
    StatCard(StatCardData),
    StatSummary(StatSummaryData),
    ProgressList(ProgressListData),
    TabChart(TabChartData),
    InfoCard(InfoCardData),
    RankingTable(RankingTableData),
}

// ─── Stat Card ────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatCardData {
    pub icon: String,
    pub icon_bg: String,
    pub label: String,
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

// ─── Stat Summary ────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatSummaryData {
    pub icon: String,
    pub icon_bg: String,
    pub main_value: String,
    pub main_label: String,
    pub items: Vec<StatSummaryItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatSummaryItem {
    pub label: String,
    pub value: String,
    pub trend: f64,
    #[serde(default)]
    pub trend_label: String,
    #[serde(default)]
    pub icon: String,
}

// ─── Progress List ──────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgressListData {
    pub icon: String,
    pub icon_bg: String,
    pub main_value: String,
    pub main_label: String,
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
    pub icon: String,
    pub icon_bg: String,
    pub main_value: String,
    pub main_label: String,
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
    pub icon: String,
    pub icon_bg: String,
    pub main_value: String,
    pub main_label: String,
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
    pub columns: Vec<String>,
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
