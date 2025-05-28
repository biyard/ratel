use bdk::prelude::*;

use crate::route::Route;

#[derive(Debug, Clone, PartialEq, Copy, Translate)]
pub enum NftSettingStep {
    #[translate(en = "Summary")]
    Summary,
    #[translate(en = "NFT")]
    Nft,
}

impl NftSettingStep {
    pub fn to_route(&self, _lang: Language, feed_id: i64, id: i64) -> Route {
        match self {
            NftSettingStep::Summary => Route::NftSummary { feed_id, id },
            NftSettingStep::Nft => Route::Nft { feed_id, id },
        }
    }
}

impl From<Route> for NftSettingStep {
    fn from(route: Route) -> Self {
        match route {
            Route::NftSummary { .. } => Self::Summary,
            Route::Nft { .. } => Self::Nft,
            _ => Self::Summary,
        }
    }
}
