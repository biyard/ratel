#[derive(Debug, Clone)]
pub struct ThemeData {
    pub active: String,
    pub active01: String,
    pub active_true: String,
    pub active_false: String,
    pub background: String,
    pub primary00: String,
    pub primary03: String,
    pub primary04: String,
    pub primary05: String,
    pub primary06: String,
    pub primary07: String,
    pub primary11: String,
    pub primary100: String,
    pub grey00: String,
    pub font_theme: FontTheme,
}

#[derive(Debug, Clone)]
pub struct FontTheme {
    pub bold15: String,
}

impl Default for FontTheme {
    fn default() -> Self {
        FontTheme {
            bold15: "font-bold text-[15px] leading[22.5px]".to_string(),
        }
    }
}

impl Default for ThemeData {
    fn default() -> Self {
        ThemeData {
            active: "#68D36C".to_string(),
            active01: "#FF5A5D".to_string(),
            active_true: "#3FA451".to_string(),
            active_false: "#DA4447".to_string(),
            background: "#2C2E42".to_string(),
            primary00: "#ADBCD7".to_string(),
            primary03: "#74789E".to_string(),
            primary04: "#8588AB".to_string(),
            primary05: "#212231".to_string(),
            primary06: "#424563".to_string(),
            primary07: "#404761".to_string(),
            primary11: "#292B3C".to_string(),
            primary100: "#B5AB65".to_string(),
            grey00: "#FFFFFF".to_string(),
            font_theme: FontTheme::default(),
        }
    }
}

use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct Theme {
    pub data: Signal<ThemeData>,
}

impl Theme {
    pub fn init() {
        use_context_provider(|| Self {
            data: Signal::new(ThemeData::default()),
        });
    }

    pub fn get_data(&self) -> ThemeData {
        (self.data)()
    }

    pub fn get_font_theme(&self) -> FontTheme {
        (self.data)().font_theme.clone()
    }
}
