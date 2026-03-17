use crate::common::*;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Serialize,
    Deserialize,
    strum::Display,
    strum::EnumString,
    Default,
)]
pub enum CardDirection {
    #[default]
    #[strum(serialize = "flex flex-col")]
    Col,

    #[strum(serialize = "flex flex-row")]
    Row,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Serialize,
    Deserialize,
    strum::Display,
    strum::EnumString,
    Default,
)]
pub enum MainAxisAlign {
    #[default]
    #[strum(serialize = "justify-start")]
    Start,
    #[strum(serialize = "justify-center")]
    Center,
    #[strum(serialize = "justify-end")]
    End,
    #[strum(serialize = "justify-between")]
    Between,
    #[strum(serialize = "justify-around")]
    Around,
    #[strum(serialize = "justify-evenly")]
    Evenly,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Serialize,
    Deserialize,
    strum::Display,
    strum::EnumString,
    Default,
)]
pub enum CrossAxisAlign {
    #[default]
    #[strum(serialize = "items-start")]
    Start,
    #[strum(serialize = "items-center")]
    Center,
    #[strum(serialize = "items-end")]
    End,
    #[strum(serialize = "items-stretch")]
    Stretch,
    #[strum(serialize = "items-baseline")]
    Baseline,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Serialize,
    Deserialize,
    strum::Display,
    strum::EnumString,
    Default,
)]
pub enum CardVariant {
    #[default]
    #[strum(serialize = "bg-card-bg border bg-card-bg border-card-border")]
    Normal,

    #[strum(serialize = "border border-card-border")]
    Outlined,

    #[strum(serialize = "bg-card-bg")]
    Filled,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Serialize,
    Deserialize,
    strum::Display,
    strum::EnumString,
    Default,
)]
pub enum CardShape {
    #[default]
    #[strum(serialize = "rounded-[10px]")]
    Rounded,

    #[strum(serialize = "rounded-0")]
    Squere,
}

#[component]
pub fn Card(
    #[props(default)] variant: CardVariant,
    #[props(default)] shape: CardShape,
    #[props(default)] direction: CardDirection,
    #[props(default)] main_axis_align: MainAxisAlign,
    #[props(default)] cross_axis_align: CrossAxisAlign,
    #[props(default)] class: String,
    #[props(default)] onclick: EventHandler<MouseEvent>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "py-5 px-4 {shape} {variant} {direction} {main_axis_align} {cross_axis_align} {class}",
            ..attributes,
            {children}
        }
    }
}
