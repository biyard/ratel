use crate::common::*;

#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum AiDraftTemplate {
    #[default]
    OpinionGathering,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum AiDraftLanguage {
    #[default]
    Ko,
    En,
}

impl AiDraftLanguage {
    pub fn as_code(&self) -> &'static str {
        match self {
            AiDraftLanguage::Ko => "ko",
            AiDraftLanguage::En => "en",
        }
    }
}
