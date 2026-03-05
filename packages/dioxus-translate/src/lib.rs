pub use dioxus::prelude::*;
pub use dioxus_translate_macro::*;
pub use dioxus_translate_types::Translator;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

static LANGUAGE: GlobalSignal<Language> = Signal::global(|| {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(lang) = read_local_storage(STORAGE_KEY) {
            if let Ok(l) = lang.parse::<Language>() {
                return l;
            }
        }

        if let Some(lang) = browser_language() {
            if let Ok(l) = lang.parse::<Language>() {
                return l;
            }
        }
    }

    Language::default()
});

pub const STORAGE_KEY: &str = "language";

pub fn use_translate<T: Translator>() -> T {
    let lang = use_language();
    let l = lang();

    translate::<T>(&l)
}

#[cfg(target_arch = "wasm32")]
pub fn use_language() -> Signal<Language> {
    LANGUAGE.signal()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn use_language() -> Signal<Language> {
    use_signal(|| language_from_cookie())
}

/// Reads the language from the `language` cookie in the current HTTP request.
/// Uses `FullstackContext::current()` to access request headers during SSR.
/// Returns `Language::default()` if no context or cookie is found.
#[cfg(not(target_arch = "wasm32"))]
pub fn language_from_cookie() -> Language {
    use dioxus::fullstack::FullstackContext;

    let Some(ctx) = FullstackContext::current() else {
        return Language::default();
    };
    let parts = ctx.parts_mut();
    parts
        .headers
        .get("cookie")
        .and_then(|v| v.to_str().ok())
        .and_then(|cookies| {
            cookies
                .split(';')
                .find_map(|c| c.trim().strip_prefix("language="))
        })
        .and_then(|v| v.parse::<Language>().ok())
        .unwrap_or_default()
}

/// Sets the global language signal value.
pub fn set_language(lang: Language) {
    LANGUAGE.signal().set(lang);
}

#[cfg(target_arch = "wasm32")]
fn read_local_storage(key: &str) -> Option<String> {
    web_sys::window()?
        .local_storage()
        .ok()??
        .get_item(key)
        .ok()?
}

#[cfg(target_arch = "wasm32")]
fn browser_language() -> Option<String> {
    let lang = web_sys::window()?.navigator().language()?;
    Some(lang.split('-').next().unwrap_or(&lang).to_string())
}

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
        let next = match self {
            Language::Ko => Language::En,
            Language::En => Language::Ko,
        };

        #[cfg(not(feature = "ko"))]
        let next = Language::En;

        LANGUAGE.signal().set(next);

        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::wasm_bindgen::JsCast;

            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item(STORAGE_KEY, &next.to_string());
                }

                if let Some(doc) = window.document() {
                    let html_document = doc.dyn_into::<web_sys::HtmlDocument>().unwrap();
                    let _ = html_document.set_cookie(&format!("language={}; path=/;", next));
                }
            }
        }

        next
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

pub trait Translate {
    fn translate(&self, lang: &Language) -> &'static str;
}
