use crate::common::*;

#[component]
pub fn Badge(
    #[props(default)] color: BadgeColor,
    #[props(default)] size: BadgeSize,
    #[props(default)] variant: BadgeVariant,
    #[props(default)] fill: BadgeFill,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let color_class = match fill {
        BadgeFill::Flat => color.to_string(),
        BadgeFill::Gradient => color.gradient_class().to_string(),
    };
    rsx! {
        div { class: "{color_class} {size} {variant}", ..attributes, {children} }
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

impl BadgeColor {
    /// Gradient-fill class strings used when `fill: BadgeFill::Gradient` is passed.
    /// Each variant maps to a two-stop linear gradient + inset highlight + subtle border.
    pub fn gradient_class(&self) -> &'static str {
        match self {
            BadgeColor::Grey => "text-badge-grey bg-gradient-to-br from-badge-grey/30 to-badge-grey/5 border border-badge-grey/40 shadow-[inset_0_1px_0_rgba(255,255,255,0.08)]",
            BadgeColor::Blue => "text-badge-blue bg-gradient-to-br from-badge-blue/30 to-badge-blue/5 border border-badge-blue/40 shadow-[inset_0_1px_0_rgba(255,255,255,0.08)]",
            BadgeColor::Green => "text-badge-green bg-gradient-to-br from-badge-green/30 to-badge-green/5 border border-badge-green/40 shadow-[inset_0_1px_0_rgba(255,255,255,0.08)]",
            BadgeColor::Orange => "text-badge-orange bg-gradient-to-br from-badge-orange/30 to-badge-orange/5 border border-badge-orange/40 shadow-[inset_0_1px_0_rgba(255,255,255,0.08)]",
            BadgeColor::Pink => "text-badge-pink bg-gradient-to-br from-badge-pink/30 to-badge-pink/5 border border-badge-pink/40 shadow-[inset_0_1px_0_rgba(255,255,255,0.08)]",
            BadgeColor::Purple => "text-badge-purple bg-gradient-to-br from-badge-purple/30 to-badge-purple/5 border border-badge-purple/40 shadow-[inset_0_1px_0_rgba(255,255,255,0.08)]",
            BadgeColor::Red => "text-badge-red bg-gradient-to-br from-badge-red/30 to-badge-red/5 border border-badge-red/40 shadow-[inset_0_1px_0_rgba(255,255,255,0.08)]",
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
pub enum BadgeSize {
    #[default]
    #[strum(serialize = "font-semibold px-[13px] py-[7px] text-[12px]/[16px]")]
    Normal,

    #[strum(serialize = "py-1 px-2 font-semibold text-[12px]")]
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

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum BadgeFill {
    #[default]
    Flat,
    Gradient,
}
