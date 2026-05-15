use crate::features::arcade::games::fact_or_fold::pages::game_room::FactFoldRoomTranslate;
use crate::*;

/// `SettlementView` — Stage 6. Filled in step 10 of the game-room PR.
#[component]
pub fn SettlementView() -> Element {
    let tr: FactFoldRoomTranslate = use_translate();
    rsx! {
        section { class: "view", "data-active": true, "data-view": "result",
            div { class: "ff-room__placeholder", "{tr.stage_stub_settlement}" }
        }
    }
}
