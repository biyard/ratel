pub use dioxus_translate_macro::*;
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
        Language::En
    }
}

impl Language {
    pub fn switch(&self) -> Self {
        #[cfg(feature = "ko")]
        match self {
            Language::Ko => return Language::En,
            Language::En => return Language::Ko,
        }

        #[allow(unreachable_code)]
        Language::En
    }

    pub fn open_graph_locale(&self) -> String {
        match self {
            #[cfg(feature = "ko")]
            Language::Ko => "ko_KR".to_string(),
            Language::En => "en_US".to_string(),
        }
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

    pub fn all() -> Vec<Language> {
        #[cfg(feature = "ko")]
        {
            vec![Language::Ko, Language::En]
        }
        #[cfg(not(feature = "ko"))]
        {
            vec![Language::En]
        }
    }
}
