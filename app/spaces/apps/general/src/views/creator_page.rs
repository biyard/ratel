use super::home::GeneralContent;
use crate::*;

#[component]
pub fn CreatorPage(space_id: SpacePartition) -> Element {
    rsx! {
        GeneralContent { space_id }
    }
}
