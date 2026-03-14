use super::*;

#[component]
pub fn ParticipationLayoverHeader(title: String) -> Element {
    let mut layover = use_layover();

    rsx! {
        div { class: "flex flex-row items-start gap-5 px-5 py-5 w-full shrink-0 border-b border-neutral-800 bg-[#1A1A1A]",
            Button {
                size: ButtonSize::Icon,
                style: ButtonStyle::Text,
                shape: ButtonShape::Square,
                class: "flex size-6 items-center justify-center rounded-[4px] border border-[#262626] bg-transparent p-0 hover:bg-transparent",
                onclick: move |_| layover.close(),
                icons::ratel::XMarkIcon {
                    width: "16",
                    height: "16",
                    class: "h-4 w-4 [&>path]:stroke-[#737373]",
                }
            }

            h4 { class: "font-bold text-[20px]/[24px] tracking-[-0.2px] text-white",
                {title}
            }
        }
    }
}
