use crate::features::posts::components::FeedList;
use crate::features::timeline::*;

#[component]
pub fn PopularTimeline() -> Element {
    rsx! {
        section { class: "flex flex-col gap-3 w-full",
            div { class: "flex items-center px-1",
                h2 { class: "text-lg font-semibold text-text-primary",
                    "Popular"
                }
            }
            FeedList {}
        }
    }
}
