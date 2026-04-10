use super::*;

#[component]
pub fn ParticipationLayoverHeader(title: String) -> Element {
    let mut layover = use_layover();

    rsx! {
        div { class: "flex flex-row gap-5 items-start py-5 px-5 w-full border-b shrink-0 border-separator",
            Button {
                size: ButtonSize::Icon,
                style: ButtonStyle::Text,
                shape: ButtonShape::Square,
                class: "flex justify-center items-center p-0 bg-transparent border hover:bg-transparent size-6 rounded-[4px] border-border",
                onclick: move |_| layover.close(),
                icons::ratel::XMarkIcon {
                    width: "16",
                    height: "16",
                    class: "w-4 h-4 [&>path]:stroke-foreground-muted",
                }
            }

            h4 { class: "font-bold text-[20px]/[24px] tracking-[-0.2px]", {title} }
        }
    }
}
