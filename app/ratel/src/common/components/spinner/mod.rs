use crate::common::*;

#[component]
pub fn Spinner(
    #[props(default = SpinnerSize::Small)] size: SpinnerSize,
    #[props(default)] class: String,
) -> Element {
    rsx! {
        lucide_dioxus::LoaderCircle {
            class: "animate-spin {size} {class}",
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, strum::Display)]
pub enum SpinnerSize {
    #[default]
    #[strum(serialize = "w-4 h-4")]
    Small,
    #[strum(serialize = "w-5 h-5")]
    Medium,
    #[strum(serialize = "w-6 h-6")]
    Large,
}
