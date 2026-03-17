use crate::common::*;

#[component]
pub fn Badge(
    #[props(default)] color: BadgeColor,
    #[props(default)] size: BadgeSize,
    #[props(default)] variant: BadgeVariant,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "{color} {size} {variant}", ..attributes, {children} }
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
pub enum BadgeColor {
    #[default]
    #[strum(serialize = "text-badge-grey bg-badge-grey/20 border-badge-grey/30")]
    Grey,
    #[strum(serialize = "text-badge-blue bg-badge-blue/20 border-badge-blue/30")]
    Blue,
    #[strum(serialize = "text-badge-green bg-badge-green/20 border-badge-green/30")]
    Green,
    #[strum(serialize = "text-badge-orange bg-badge-orange/20 border-badge-orange/30")]
    Orange,
    #[strum(serialize = "text-badge-pink bg-badge-pink/20 border-badge-pink/30")]
    Pink,
    #[strum(serialize = "text-badge-purple bg-badge-purple/20 border-badge-purple/30")]
    Purple,
    #[strum(serialize = "text-badge-red bg-badge-red/20 border-badge-red/30")]
    Red,
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
pub enum BadgeSize {
    #[default]
    #[strum(serialize = "px-[13px] py-[7px] text-[12px]/[16px] font-semibold")]
    Normal,

    #[strum(serialize = "py-1 px-2 text-[12px] font-semibold")]
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
pub enum BadgeVariant {
    #[default]
    Default,
    #[strum(serialize = "rounded-full border")]
    Rounded,
}
