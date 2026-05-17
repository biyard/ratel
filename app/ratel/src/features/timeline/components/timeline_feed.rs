use crate::features::posts::components::CreatePostButton;
use crate::features::timeline::components::{DraftTimeline, TimelineRow};
use crate::features::timeline::controllers::list_timeline::list_timeline_feed_handler;
use crate::features::timeline::*;

/// Netflix-style timeline feed with multiple category rows.
///
/// Each category (Following, From Your Teams, Popular) is rendered
/// as a horizontal scrollable row of post cards.
#[component]
pub fn TimelineFeed() -> Element {
    let feed = use_loader(move || async move { list_timeline_feed_handler(Some(10)).await })?;

    let feed_response = feed();

    rsx! {
        div { class: "flex flex-col gap-8 w-full",
            CreatePostButton { class: "self-end w-fit" }
            DraftTimeline {}
            if feed_response.categories.is_empty() {

                div { class: "flex flex-col justify-center items-center py-20 text-center text-text-secondary",
                    p { class: "text-lg font-medium", "Your timeline is empty" }
                    p { class: "mt-2 text-sm", "Follow people or join teams to see posts here." }
                }
            } else {
                for row in feed_response.categories.iter() {
                    TimelineRow { key: "cat-{row.category}", row: row.clone() }
                }
            }

        }
    }
}
