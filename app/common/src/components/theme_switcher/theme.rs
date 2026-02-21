use super::*;
use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Default, Translate, strum::Display, strum::EnumString)]
pub enum Theme {
    #[translate(ko = "밝게", en = "Light")]
    #[strum(serialize = "light")]
    Light,
    #[translate(ko = "어둡게", en = "Dark")]
    #[strum(serialize = "dark")]
    Dark,
    #[default]
    #[translate(ko = "시스템", en = "System")]
    #[strum(serialize = "system")]
    System,
}

impl Theme {
    pub fn label(&self) -> &'static str {
        match self {
            Theme::Light => "Light",
            Theme::Dark => "Dark",
            Theme::System => "System",
        }
    }

    pub fn icon(&self) -> Element {
        match self {
            Theme::Light => rsx! {
                Sun {
                    class: "[&>path]:stroke-icon-primary [&>circle]:stroke-icon-primary",
                    height: "24",
                }
            },
            Theme::Dark => rsx! {
                Moon {
                    class: "[&>path]:stroke-icon-primary [&>circle]:stroke-icon-primary",
                    height: "24",
                }
            },
            Theme::System => rsx! {
                SunMoon {
                    class: "[&>path]:stroke-icon-primary [&>circle]:stroke-icon-primary",
                    height: "24",
                }
            },
        }
    }
}
