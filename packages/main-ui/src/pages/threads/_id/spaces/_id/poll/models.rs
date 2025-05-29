use bdk::prelude::*;

use crate::route::Route;

#[derive(Debug, Clone, PartialEq, Copy, Translate)]
pub enum PollSettingStep {
    #[translate(en = "Summary")]
    Summary,
    #[translate(en = "Poll")]
    Poll,
}

impl PollSettingStep {
    pub fn to_route(&self, _lang: Language, feed_id: i64, id: i64) -> Route {
        match self {
            PollSettingStep::Summary => Route::PollSummary { feed_id, id },
            PollSettingStep::Poll => Route::Poll { feed_id, id },
        }
    }
}

impl From<Route> for PollSettingStep {
    fn from(route: Route) -> Self {
        match route {
            Route::PollSummary { .. } => Self::Summary,
            Route::Poll { .. } => Self::Poll,
            _ => Self::Summary,
        }
    }
}
