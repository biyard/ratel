use crate::features::arcade::games::fact_or_fold::pages::game_room::FactFoldRoomTranslate;
use crate::*;

/// `LiveDebateView` — Stage 5. Filled in step 9 of the game-room PR.
#[component]
pub fn LiveDebateView() -> Element {
    let tr: FactFoldRoomTranslate = use_translate();
    rsx! {
        section { class: "view", "data-active": true, "data-view": "debate",
            div { class: "ff-room__placeholder", "{tr.stage_stub_live_debate}" }
        }
    }
}
