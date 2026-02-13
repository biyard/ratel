use crate::*;

#[component]
pub fn Button(
    #[props(default)] size: ButtonSize,
    #[props(default)] style: ButtonStyle,
    #[props(default)] shape: ButtonShape,

    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    rsx! {
        button {
            class: "{size} {style} {shape}",
            onclick: move |e| {
                if let Some(handler) = &onclick {
                    handler.call(e);
                }
            },
            ..attributes,
            {children}
        }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    DeserializeFromStr,
    SerializeDisplay,
    strum::Display,
    strum::EnumString,
    Default,
)]
pub enum ButtonShape {
    #[default]
    #[strum(serialize = "rounded-full")]
    Rounded,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    DeserializeFromStr,
    SerializeDisplay,
    strum::Display,
    strum::EnumString,
    Default,
)]
pub enum ButtonSize {
    #[default]
    #[strum(serialize = "py-3 px-5 text-[14px]/[16px] font-bold")]
    Medium,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    DeserializeFromStr,
    SerializeDisplay,
    strum::Display,
    strum::EnumString,
    Default,
)]
pub enum ButtonStyle {
    #[default]
    #[strum(
        serialize = "bg-btn-primary-bg border-btn-primary-outline text-btn-primary-text hover:bg-btn-primary-hover-bg hover:border-btn-primary-hover-outline hover:text-btn-primary-hover-text disabled:bg-btn-primary-disable-bg disabled:border-btn-primary-disable-outline disabled:text-btn-primary-disable-text"
    )]
    Primary,

    #[strum(
        serialize = "bg-btn-secondary-bg border-btn-secondary-outline text-btn-secondary-text hover:bg-btn-secondary-hover-bg hover:border-btn-secondary-hover-outline hover:text-btn-secondary-hover-text disabled:bg-btn-secondary-disable-bg disabled:border-btn-secondary-disable-outline disabled:text-btn-secondary-disable-text"
    )]
    Secondary,

    #[strum(
        serialize = "bg-btn-outline-bg border-btn-outline-outline text-btn-outline-text hover:bg-btn-outline-hover-bg hover:border-btn-outline-hover-outline hover:text-btn-outline-hover-text disabled:bg-btn-outline-disable-bg disabled:border-btn-outline-disable-outline disabled:text-btn-outline-disable-text"
    )]
    Outline,
}
