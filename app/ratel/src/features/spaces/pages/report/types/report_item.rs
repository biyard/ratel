use crate::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, SerializeDisplay, DeserializeFromStr, DynamoEnum,
)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub enum ReportStatus {
    #[default]
    Draft,
    Published,
}

/// UI-side filter chip state on the report list carousel. Mapped to
/// the server's optional `status` query parameter via
/// [`ReportFilter::to_status`]: `All` skips the filter (main-table
/// query), `Drafts` / `Published` pin it to the matching
/// [`ReportStatus`] which routes the request through the `gsi1`
/// `find_by_status` GSI.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ReportFilter {
    #[default]
    All,
    Drafts,
    Published,
}

impl ReportFilter {
    pub fn to_status(self) -> Option<ReportStatus> {
        match self {
            ReportFilter::All => None,
            ReportFilter::Drafts => Some(ReportStatus::Draft),
            ReportFilter::Published => Some(ReportStatus::Published),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct ReportListItem {
    pub id: String,
    pub status: ReportStatus,
    pub title: String,
    pub description: String,
    pub created_at: i64,
}
