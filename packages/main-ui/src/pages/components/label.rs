use bdk::prelude::*;

#[component]
pub fn Label(label: String) -> Element {
    rsx! {
        div { class: "px-8 border border-border-primary bg-transparent rounded-[4px] font-semibold text-white text-xs/25",
            {label}
        }
    }
}
