use dioxus::prelude::*;
use std::fmt;

pub const TYPOGRAPHY_JS: Asset = asset!(
    "/assets/typography.js",
    AssetOptions::js().with_preload(true)
);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Variant {
    Title1,
    Title2,
    Title3,
    H1,
    H2,
    H3,
    H4,
    Label1,
    Label2,
    Label3,
    Label4,
    Label5,
    Body1,
    Body2,
    Body3,
    Body4,
    Btn1,
    Btn2,
    Num1,
    Num2,
    Num3,
    TinyExt,
    Tiny,
    Micro,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Weight {
    Extrabold,
    Bold,
    Semibold,
    Medium,
    Regular,
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Variant::Title1 => "title-1",
            Variant::Title2 => "title-2",
            Variant::Title3 => "title-3",
            Variant::H1 => "h1",
            Variant::H2 => "h2",
            Variant::H3 => "h3",
            Variant::H4 => "h4",
            Variant::Label1 => "label-1",
            Variant::Label2 => "label-2",
            Variant::Label3 => "label-3",
            Variant::Label4 => "label-4",
            Variant::Label5 => "label-5",
            Variant::Body1 => "body-1",
            Variant::Body2 => "body-2",
            Variant::Body3 => "body-3",
            Variant::Body4 => "body-4",
            Variant::Btn1 => "btn-1",
            Variant::Btn2 => "btn-2",
            Variant::Num1 => "num-1",
            Variant::Num2 => "num-2",
            Variant::Num3 => "num-3",
            Variant::TinyExt => "tiny-ext",
            Variant::Tiny => "tiny",
            Variant::Micro => "micro",
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Weight::Extrabold => "extrabold",
            Weight::Bold => "bold",
            Weight::Semibold => "semibold",
            Weight::Medium => "medium",
            Weight::Regular => "regular",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Props, Clone, PartialEq)]
pub struct TypoProps {
    pub variant: Variant,
    pub weight: Weight,
    #[props(default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn Typo(props: TypoProps) -> Element {
    rsx! {
        document::Script { src: TYPOGRAPHY_JS }
        ratel-typo {
            variant: props.variant.to_string(),
            weight: props.weight.to_string(),
            class: props.class,
            {props.children}
        }
    }
}
