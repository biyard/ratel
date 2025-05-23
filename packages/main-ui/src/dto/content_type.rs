pub use bdk::prelude::*;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Translate, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum ContentType {
    #[translate(ko = "Crypto", en = "Crypto")]
    #[default]
    Crypto,
    #[translate(ko = "Social", en = "Social")]
    Social,
}
