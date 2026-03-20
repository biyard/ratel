use super::super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct MonthlyPointSummary {
    pub month: String,
    pub points: i64,
    pub total_points: i64,
    pub monthly_token_supply: i64,
    pub is_swapped: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct MonthlyHistoryResponse {
    pub project_name: String,
    pub token_symbol: String,
    pub total_accumulated_points: i64,
    pub items: Vec<MonthlyPointSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct PointChartData {
    pub label: String,
    pub points: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct PointChartResponse {
    pub period: String,
    pub date_range: String,
    pub items: Vec<PointChartData>,
}
