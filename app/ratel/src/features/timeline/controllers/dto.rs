use crate::features::posts::controllers::dto::PostResponse;
use crate::features::timeline::*;
use crate::features::timeline::models::TimelineReason;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

/// A single category row in the Netflix-style timeline.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct TimelineCategoryRow {
    pub category: String,
    pub items: Vec<PostResponse>,
    pub bookmark: Option<String>,
    pub has_more: bool,
}

/// Response containing multiple category rows for the Netflix-style layout.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct TimelineFeedResponse {
    pub categories: Vec<TimelineCategoryRow>,
}
