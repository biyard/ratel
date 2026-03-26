use crate::features::timeline::components::{
    DraftTimeline, FollowingTimeline, PopularTimeline, SpaceTimeline, TeamTimeline,
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
                    SpaceTimeline {}
                    FollowingTimeline {}
                    TeamTimeline {}
                }
                PopularTimeline {}
            }
        }
    }
}
