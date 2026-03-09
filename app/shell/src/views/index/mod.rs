#[cfg(feature = "users")]
use crate::features::users::UserSidemenu;
use crate::*;
use crate::features::posts::components::{CreatePostButton, FeedList};

#[component]
pub fn Index() -> Element {
    let user_ctx = ratel_auth::hooks::use_user_context();
    let user = user_ctx().user.clone();

    rsx! {
        div { class: "flex overflow-x-hidden gap-5 justify-between py-3 mx-auto min-h-screen max-w-desktop max-tablet:px-2.5",
            if let Some(user) = &user {
                UserSidemenu { username: user.username.clone() }
            }

            div { class: "flex grow", FeedList {} }

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

#[cfg(not(feature = "users"))]
#[component]
fn UserSidemenu(username: String) -> Element {
    let _ = username;
    rsx! {}
}
