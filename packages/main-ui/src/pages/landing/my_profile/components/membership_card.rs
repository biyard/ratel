use bdk::prelude::*;
use dto::Membership;
use num_format::{Locale, ToFormattedString};

#[component]
pub fn MembershipCard(selected: bool, membership: Membership) -> Element {
    rsx! {
        button { class: "flex flex-col min-h-400 bg-component-bg py-40 px-10 rounded-sm items-center justify-between gap-24 cursor-pointer",
            div { class: "flex flex-col gap-24",
                h3 { {membership.translate(&Language::En)} }
                p { {membership.get_description()} }
            }


            p { "$ {membership.get_price().to_formatted_string(&Locale::en)}" }
        }
    }
}
