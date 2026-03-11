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

#[component]
pub fn Card(
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
            class: "py-5 px-4 border rounded-[10px] bg-card-bg border-card-border {direction} {main_axis_align} {cross_axis_align} {class}",
            ..attributes,
            {children}
        }
    }
}
