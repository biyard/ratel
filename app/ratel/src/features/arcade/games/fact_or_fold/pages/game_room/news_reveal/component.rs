use crate::features::arcade::games::fact_or_fold::pages::game_room::FactFoldRoomTranslate;
use crate::*;

/// `NewsRevealView` — Stage 1. Filled in step 5 of the game-room PR.
#[component]
pub fn NewsRevealView() -> Element {
    let tr: FactFoldRoomTranslate = use_translate();
    rsx! {
        section { class: "view", "data-active": true, "data-view": "round",
            div { class: "ff-room__placeholder", "{tr.stage_stub_news_reveal}" }
        }
    }
}
