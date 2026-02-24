use super::home::IncentivePoolContent;
use crate::*;

#[component]
pub fn CreatorPage(space_id: SpacePartition) -> Element {
    rsx! {
        IncentivePoolContent { space_id }
    }
}
