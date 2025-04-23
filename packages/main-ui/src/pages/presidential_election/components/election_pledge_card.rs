use bdk::prelude::*;
use dto::ElectionPledge;

#[component]
pub fn ElectionPledgeCard(promise: ElectionPledge) -> Element {
    rsx! {
        div {
            id: "election-pledge-{promise.id}",
            class: "w-full border border-c-wg-70 py-16 px-20 rounded-[10px]",
            article {
                class: "election-pledge",
                dangerous_inner_html: promise.promise,
            }
        }
    }
}
