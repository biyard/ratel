use crate::*;
use ratel_post::components::{CreatePostButton, FeedList};

#[component]
pub fn Index() -> Element {
    rsx! {
        div { class: "flex overflow-x-hidden gap-5 justify-between py-3 mx-auto min-h-screen max-w-desktop max-tablet:px-2.5",
            UserSidemenu {}
            div { class: "flex grow",
                FeedList {}
            }
            div {
                class: "flex flex-col gap-2.5 w-70 shrink-0 max-tablet:fixed max-tablet:bottom-4 max-tablet:right-4 max-tablet:z-50 max-tablet:w-auto",
                aria_label: "Sidebar",
                CreatePostButton {}
            }
        }
    }
}
