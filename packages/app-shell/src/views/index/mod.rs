use crate::*;
use ratel_post::components::FeedList;

#[component]
pub fn Index() -> Element {
    rsx! {
        FeedList {}
    }
}
