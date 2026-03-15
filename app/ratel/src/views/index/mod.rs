#[cfg(feature = "social")]
use crate::features::social::UserSidemenu;
use crate::*;
use crate::features::posts::components::{CreatePostButton, FeedList};
use crate::features::timeline::components::TimelineFeed;

#[component]
pub fn Index() -> Element {
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let user = user_ctx().user.clone();

    rsx! {
        div { class: "flex overflow-x-hidden gap-5 justify-between py-3 mx-auto min-h-screen max-w-desktop max-tablet:px-2.5",
            if let Some(ref user) = user {
                UserSidemenu { username: user.username.clone() }
            }

            div { class: "flex flex-col grow gap-4",
                if user.is_some() {
                    TimelineFeed {}
                } else {
                    PopularFeedSection {}
                }
            }

            if user.is_some() {
                div {
                    class: "flex flex-col gap-2.5 shrink-0 max-tablet:fixed max-tablet:bottom-4 max-tablet:right-4 max-tablet:z-50 max-tablet:w-auto",
                    aria_label: "Sidebar",
                    CreatePostButton {}
                }
            }
        }
    }
}

/// Anonymous users see a popular posts feed with a header.
#[component]
fn PopularFeedSection() -> Element {
    rsx! {
        section { class: "flex flex-col gap-3 w-full",
            div { class: "flex items-center px-1",
                h2 { class: "text-lg font-semibold text-text-primary",
                    "Popular Posts"
                }
            }
            FeedList {}
        }
    }
}

#[cfg(not(feature = "social"))]
#[component]
fn UserSidemenu(username: String) -> Element {
    let _ = username;
    rsx! {}
}
