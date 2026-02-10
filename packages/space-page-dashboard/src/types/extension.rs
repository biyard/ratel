use crate::*;
use serde::{Deserialize, Serialize};

// ─── Widget Type ────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DashboardWidgetType {
    StatCard,
    StatSummary,
    ProgressList,
    TabChart,
    InfoCard,
    RankingTable,
}

// ─── Top-level Extension ────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DashboardExtension {
    pub id: String,
    pub widget_type: DashboardWidgetType,
    pub order: i32,
    pub title: String,
    /// Grid column span (1–4). Defaults to 1.
    #[serde(default = "default_span")]
    pub span: u8,
    pub data: DashboardWidgetData,
}

fn default_span() -> u8 {
    1
}

// ─── Widget Data (type-safe enum) ───────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DashboardWidgetData {
    StatCard(StatCardData),
    StatSummary(StatSummaryData),
    ProgressList(ProgressListData),
    TabChart(TabChartData),
    InfoCard(InfoCardData),
    RankingTable(RankingTableData),
}

// ─── Stat Card (simple) ────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatCardData {
    pub icon: String,
    pub label: String,
    pub value: String,
    pub trend: f64,
    #[serde(default)]
    pub trend_label: String,
}

// ─── Stat Summary (complex: main value + sub-stats) ────────

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

// ─── Tab Chart (horizontal bars with tabs) ──────────────────

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

// ─── Info Card (key-value pairs) ────────────────────────────

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
