use crate::common::components::{Robots, SeoMeta};
use crate::features::posts::components::CreatePostButton;
use crate::features::timeline::components::{
    DraftTimeline, FollowingTimeline, PopularTimeline, SpaceTimeline, TeamTimeline,
};
use crate::*;

#[component]
pub fn Index() -> Element {
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let user = user_ctx().user.clone();

    let keywords = vec![
        "ratel".to_string(),
        "knowledge platform".to_string(),
        "ai knowledge base".to_string(),
        "human knowledge dataset".to_string(),
        "ai training data".to_string(),
        "community intelligence".to_string(),
        "participatory platform".to_string(),
        "survey rewards".to_string(),
        "poll rewards".to_string(),
        "web3 knowledge economy".to_string(),
        "ai memory platform".to_string(),
        "vector knowledge database".to_string(),
        "collective intelligence".to_string(),
    ];

    rsx! {
        SeoMeta {
            title: "Ratel – AI Knowledge Platform Powered by Human Essences",
            description: "Ratel is a participatory knowledge platform where users share expertise, opinions, and insights as structured Essences. AI agents learn from the knowledge base while users earn rewards through surveys, polls, and discussions.",
            image: "https://metadata.ratel.foundation/logos/logo-symbol.png",
            url: "https://ratel.foundation",
            robots: Robots::IndexNofollow,
            keywords,
        }
        div { class: "flex overflow-x-hidden relative gap-5 justify-between py-3 pl-2 mx-auto w-full min-h-screen max-tablet:px-2.5",
            div { class: "flex flex-col gap-4 w-full",
                if user.is_some() {
                    CreatePostButton { class: "self-end w-fit" }
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
