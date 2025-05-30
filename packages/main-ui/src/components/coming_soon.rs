use bdk::prelude::*;

use crate::components::icons;

#[component]
pub fn ComingSoon(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {..attributes,
            div { class: "w-full h-full bg-bg flex flex-col items-center justify-center rounded-[20px] gap-30 max-tablet:py-50",
                icons::ComingSoon {}
                p { class: "text-5xl font-bold text-text-primary max-tablet:text-2xl",
                    "Coming soon"
                }
            }
        }
    }
}
