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
            DeliberationSettingStep::Summary => Route::Summary { feed_id, id },
            DeliberationSettingStep::Deliberation => Route::Deliberation { feed_id, id },
            DeliberationSettingStep::Poll => Route::Poll { feed_id, id },
            DeliberationSettingStep::FinalConsensus => Route::FinalConsensus { feed_id, id },
        }
    }
}

impl From<Route> for DeliberationSettingStep {
    fn from(route: Route) -> Self {
        match route {
            Route::Summary { .. } => Self::Summary,
            Route::Deliberation { .. } => Self::Deliberation,
            Route::FinalConsensus { .. } => Self::FinalConsensus,
            Route::Poll { .. } => Self::Poll,
            _ => Self::Summary,
        }
    }
}
