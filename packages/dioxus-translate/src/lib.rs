pub use dioxus_translate_macro::translate;
pub use dioxus_translate_types::Translator;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub fn translate<T: Translator>(lang: &Language) -> T {
    match lang {
        #[cfg(feature = "ko")]
        Language::Ko => T::ko(),
        Language::En => T::en(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Copy, JsonSchema)]
pub enum Language {
    #[cfg(feature = "ko")]
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "en")]
    En,
}

impl Default for Language {
    fn default() -> Self {
        #[cfg(feature = "web")]
        {
            use std::str::FromStr;

            let w = web_sys::window().unwrap();
            let loc = w.location().pathname().unwrap_or_default().clone();
            let paths: Vec<_> = loc.split("/").collect();
            if paths.len() > 1 {
                return Language::from_str(paths[1]).unwrap();
            }
        }
        Language::En
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "ko")]
            Language::Ko => write!(f, "ko"),
            Language::En => write!(f, "en"),
        }
    }
}

impl std::str::FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            #[cfg(feature = "ko")]
            "ko" => Ok(Language::Ko),
            "en" => Ok(Language::En),
            _ => Ok(Language::En),
        }
    }
}

impl Language {
    pub fn to_string(&self) -> String {
        match self {
            #[cfg(feature = "ko")]
            Language::Ko => "ko".to_string(),
            Language::En => "en".to_string(),
        }
    }
}
