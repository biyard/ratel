use crate::*;

#[component]
pub fn SpaceLabel() -> Element {
    rsx! {
        div { class: "box-border flex flex-row items-start px-[13px] py-[7px]
                bg-[rgba(34,197,94,0.2)] border border-[rgba(34,197,94,0.3)] rounded-full font-semibold text-sm leading-4 text-[#22C55E]",
            "Ongoing"
        }
    }
}
