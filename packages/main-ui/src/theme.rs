#[derive(Debug, Clone)]
pub struct ThemeData {
    pub active: String,
    pub active00: String,
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
    pub hover: String,
    pub highlight: String,
    pub font_theme: FontTheme,
}

#[derive(Debug, Clone)]
pub struct FontTheme {
    pub exbold15: String,
    pub bold15: String,
}

impl Default for FontTheme {
    fn default() -> Self {
        FontTheme {
            exbold15: "font-extrabold text-[15px] leading-[22.5px]".to_string(),
            bold15: "font-bold text-[15px] leading[22.5px]".to_string(),
        }
    }
}

impl Default for ThemeData {
    fn default() -> Self {
        ThemeData {
            active: "#68D36C".to_string(), // bg-[#68D36C] text-[#68D36C] border-[#68D36C]
            active00: "#68D36C".to_string(), // bg-[#68D36C] text-[#68D36C] border-[#68D36C]
            active01: "#FF5A5D".to_string(), // bg-[#FF5A5D] text-[#FF5A5D] border-[#FF5A5D]
            active_true: "#3FA451".to_string(), // bg-[#3FA451] text-[#3FA451] border-[#3FA451]
            active_false: "#DA4447".to_string(), // bg-[#DA4447] text-[#DA4447] border-[#DA4447]
            background: "#2C2E42".to_string(), // bg-[#2C2E42] text-[#2C2E42] border-[#2C2E42]
            primary00: "#ADBCD7".to_string(), // bg-[#ADBCD7] text-[#ADBCD7] border-[#ADBCD7]
            primary03: "#74789E".to_string(), // bg-[#74789E] text-[#74789E] border-[#74789E]
            primary04: "#8588AB".to_string(), // bg-[#8588AB] text-[#8588AB] border-[#8588AB]
            primary05: "#212231".to_string(), // bg-[#212231] text-[#212231] border-[#212231]
            primary06: "#424563".to_string(), // bg-[#424563] text-[#424563] border-[#424563]
            primary07: "#404761".to_string(), // bg-[#404761] text-[#404761] border-[#404761]
            primary11: "#292B3C".to_string(), // bg-[#292B3C] text-[#292B3C] border-[#292B3C]
            primary100: "#B5AB65".to_string(), // bg-[#B5AB65] text-[#B5AB65] border-[#B5AB65]
            grey00: "#FFFFFF".to_string(), // bg-[#FFFFFF] text-[#FFFFFF] border-[#FFFFFF]
            hover: "#323342".to_string(), // bg-[#323342] text-[#323342] border-[#323342]
            highlight: "#C0B086".to_string(), // bg-[#C0B086] text-[#C0B086] border-[#C0B086]
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
