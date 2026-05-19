//! Chip balance widget — rendered in the ArcadeLayout top-bar.
//! Click toggles the exchange modal.

use crate::features::arcade::hooks::use_arcade_wallet;
use crate::features::arcade::i18n::ArcadeLayoutTranslate;
use crate::*;

#[component]
pub fn ChipBalance(on_click: EventHandler<()>) -> Element {
    let tr: ArcadeLayoutTranslate = use_translate();
    let wallet = use_arcade_wallet();
    let state = wallet.state()?;
    let chip = state().chip_balance;

    rsx! {
        button {
            class: "stat-chip gold ff-arcade__chip-btn",
            "aria-label": "{tr.chip_aria}",
            onclick: move |_| on_click.call(()),
            span { class: "stat-chip-icon", "◆" }
            span { "{chip} {tr.chip_unit}" }
        }
    }
}
