use crate::features::arcade::games::fact_or_fold::pages::game_room::FactFoldRoomTranslate;
use crate::*;

/// `ReasoningWriteView` — Stage 3. Filled in step 7 of the game-room PR.
#[component]
pub fn ReasoningWriteView() -> Element {
    let tr: FactFoldRoomTranslate = use_translate();
    rsx! {
        section { class: "view", "data-active": true, "data-view": "reason",
            div { class: "ff-room__placeholder", "{tr.stage_stub_reasoning_write}" }
        }
    }
}
