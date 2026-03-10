use crate::features::spaces::pages::apps::apps::panels::*;

#[component]
pub fn AttributeButton(
    label: String,
    selected: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let class = if selected {
        "h-9 !px-3 border-primary !bg-primary/10 !text-text-primary"
    } else {
        "h-9 !px-3 border-input-box-border !bg-input-box-bg !text-text-primary"
    };

    rsx! {
        Button {
            size: ButtonSize::Small,
            style: ButtonStyle::Outline,
            shape: ButtonShape::Square,
            class: class.to_string(),
            onclick,
            {label}
        }
    }
}
