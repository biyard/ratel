use crate::common::*;

#[component]
pub fn Button(
    #[props(default)] class: String,
    #[props(default)] size: ButtonSize,
    #[props(default)] style: ButtonStyle,
    #[props(default)] shape: ButtonShape,
    #[props(default)] disabled: bool,
    #[props(default)] loading: ReadSignal<bool>,
    #[props(default)] loading_class: Option<String>,
    #[props(extends=GlobalAttributes)]
    #[props(extends=button)]
    attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let loading_class = match loading_class {
        Some(class) => class,
        None => "w-auto h-auto grow-0 max-w-4".to_string(),
    };
    rsx! {
        button {
            class: "{size} {style} {shape} {class}",
            disabled: disabled || loading(),
            onclick: move |e| {
                if disabled {
                    return;
                }
                if let Some(handler) = &onclick {
                    handler.call(e);
                }
            },
            ..attributes,
            if loading() {
                LoadingIndicator { class: loading_class, max_width: "32px" }
            } else {
                {children}
            }
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
    #[strum(serialize = "rounded-lg")]
    Square,
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
    #[strum(serialize = "py-3 px-5 font-bold text-[14px]/[16px]")]
    Medium,

    #[strum(serialize = "py-0 px-0 font-medium text-[15px]/[24px]")]
    Inline,

    #[strum(serialize = "p-1 font-medium text-[14px]/[14px]")]
    Icon,

    #[strum(serialize = "py-1.5 px-3 font-semibold text-[13px]/[16px]")]
    Small,
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
        serialize = "border bg-btn-outline-bg border-btn-outline-outline text-btn-outline-text hover:bg-btn-outline-hover-bg hover:border-btn-outline-hover-outline hover:text-btn-outline-hover-text disabled:bg-btn-outline-disable-bg disabled:border-btn-outline-disable-outline disabled:text-btn-outline-disable-text"
    )]
    Outline,

    #[strum(
        serialize = "bg-transparent border-transparent disabled:bg-transparent disabled:border-transparent text-text-primary hover:bg-hover disabled:text-text-secondary"
    )]
    Text,

    #[strum(
        serialize = "font-extrabold tracking-wide bg-gradient-to-b border disabled:opacity-50 from-[#ffe082] via-[#fcb300] to-[#8a5d00] text-[#1a1208] border-[color:rgba(252,179,0,0.6)] shadow-[var(--depth-sm),var(--rim-glow-primary)] hover:brightness-110"
    )]
    Hero,
}
