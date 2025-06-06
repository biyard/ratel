use bdk::prelude::*;

use crate::route::Route;

#[derive(Debug, Clone, PartialEq, Copy, Translate)]
pub enum DeliberationSettingStep {
    #[translate(en = "Summary")]
    Summary,
    #[translate(en = "Deliberation")]
    Deliberation,
    #[translate(en = "Poll")]
    Poll,
    #[translate(en = "Final Consensus")]
    FinalConsensus,
}

impl DeliberationSettingStep {
    pub fn to_route(&self, _lang: Language, feed_id: i64, id: i64) -> Route {
        match self {
            DeliberationSettingStep::Summary => Route::DeliberationSummary { feed_id, id },
            DeliberationSettingStep::Deliberation => Route::Deliberation { feed_id, id },
            DeliberationSettingStep::Poll => Route::DeliberationPoll { feed_id, id },
            DeliberationSettingStep::FinalConsensus => {
                Route::DeliberationFinalConsensus { feed_id, id }
            }
        }
    }
}

impl From<Route> for DeliberationSettingStep {
    fn from(route: Route) -> Self {
        match route {
            Route::DeliberationSummary { .. } => Self::Summary,
            Route::Deliberation { .. } => Self::Deliberation,
            Route::DeliberationFinalConsensus { .. } => Self::FinalConsensus,
            Route::DeliberationPoll { .. } => Self::Poll,
            _ => Self::Summary,
        }
    }
}
