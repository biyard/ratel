use crate::features::arcade::games::fact_or_fold::pages::game_room::FactFoldRoomTranslate;
use crate::*;

/// `FirstBetView` — Stage 2. Filled in step 6 of the game-room PR.
#[component]
pub fn FirstBetView() -> Element {
    let tr: FactFoldRoomTranslate = use_translate();
    rsx! {
        section { class: "view", "data-active": true, "data-view": "bet",
            div { class: "ff-room__placeholder", "{tr.stage_stub_first_bet}" }
        }
    }
}
