use bdk::prelude::{
    by_components::icons::emoji::{ThumbsDown, ThumbsUp},
    *,
};
use dto::CryptoStance;

#[component]
pub fn CryptoStanceIcon(#[props(default = 24)] size: i32, stance: CryptoStance) -> Element {
    rsx! {
        if stance == CryptoStance::Supportive || stance == CryptoStance::StronglySupportive {
            ThumbsUp {
                class: "[&>path]:stroke-c-c-20",
                width: "{size}",
                height: "{size}",
            }
        } else if stance == CryptoStance::Against || stance == CryptoStance::StronglyAgainst {
            ThumbsDown {
                class: "[&>path]:stroke-c-p-20",
                width: "{size}",
                height: "{size}",
            }
        }
    }
}
