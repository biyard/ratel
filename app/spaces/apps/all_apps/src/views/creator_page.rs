use super::home::AllAppsContent;
use crate::*;

#[component]
pub fn CreatorPage(space_id: SpacePartition) -> Element {
    rsx! {
        AllAppsContent { space_id }
    }
}
