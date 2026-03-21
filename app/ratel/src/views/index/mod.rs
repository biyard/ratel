use crate::features::posts::components::CreatePostButton;
use crate::features::timeline::components::{
    DraftTimeline, FollowingTimeline, PopularTimeline, TeamTimeline,
};
use crate::*;

#[component]
pub fn Index() -> Element {
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let user = user_ctx().user.clone();

    rsx! {
        div { class: "relative flex overflow-x-hidden gap-5 justify-between py-3 pl-2 mx-auto w-full min-h-screen max-tablet:px-2.5",
            div { class: "flex flex-col gap-4 w-full",
                if user.is_some() {
                    DraftTimeline {}
                    FollowingTimeline {}
                    TeamTimeline {}
                }
                PopularTimeline {}
            }
            if user.is_some() {
                div { class: "fixed bottom-6 right-6 z-50",
                    CreatePostButton { class: "w-fit" }
                }
            }
        }
    }
}
