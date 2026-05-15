use crate::features::arcade::games::fact_or_fold::pages::game_room::FactFoldRoomTranslate;
use crate::*;

/// `ReasoningRevealView` — Stage 4. Filled in step 8 of the game-room PR.
#[component]
pub fn ReasoningRevealView() -> Element {
    let tr: FactFoldRoomTranslate = use_translate();
    rsx! {
        section { class: "view", "data-active": true, "data-view": "reveal",
            div { class: "ff-room__placeholder", "{tr.stage_stub_reasoning_reveal}" }
        }
    }
}
