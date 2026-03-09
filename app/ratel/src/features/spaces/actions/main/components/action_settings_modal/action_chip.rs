use crate::features::spaces::actions::main::*;

#[component]
pub fn ActionSettingsActionChip(label: String, on_remove: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "flex min-h-11 w-full items-start justify-between gap-2.5 rounded-[10px] bg-neutral-800 p-2.5",
            div { class: "flex min-w-0 items-start gap-2.5",
                icons::game::Thunder {
                    width: "18",
                    height: "18",
                    class: "shrink-0 text-web-font-primary [&>path]:stroke-current",
                }
                span { class: "whitespace-normal break-words font-bold font-raleway text-[14px]/[16px] text-web-font-primary",
                    {label}
                }
            }

            Button {
                size: ButtonSize::Icon,
                style: ButtonStyle::Text,
                shape: ButtonShape::Square,
                class: "flex size-[18px] shrink-0 items-center justify-center rounded-none p-0 text-web-font-neutral hover:bg-transparent",
                onclick: move |e| on_remove.call(e),
                icons::validations::Clear {
                    width: "18",
                    height: "18",
                    class: "[&>path]:stroke-current",
                }
            }
        }
    }
}
