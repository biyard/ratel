use crate::*;
use ratel_post::components::FeedList;

#[component]
pub fn Index() -> Element {
    rsx! {
        div { class: "flex overflow-x-hidden gap-5 justify-between py-3 mx-auto min-h-screen max-w-desktop max-tablet:px-2.5",
            UserSidemenu {}
            div { class: "flex grow",
                FeedList {}
            }
        }
    }
}
