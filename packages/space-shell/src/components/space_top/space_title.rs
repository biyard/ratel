use crate::*;

#[component]
pub fn SpaceTitle() -> Element {
    rsx! {
        div { class: "text-[15px] font-bold text-white",
            "Crypto/Temporary Increase of Staking Rewards to 8% for 90 Days"
        }
    }
}
