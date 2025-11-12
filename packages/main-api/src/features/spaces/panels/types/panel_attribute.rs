use crate::*;
use std::fmt;
use std::fmt::Display;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, OperationIo, Default,
)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum PanelAttribute {
    #[default]
    None,
    CollectiveAttribute(CollectiveAttribute),
    VerifiableAttribute(VerifiableAttribute),
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, OperationIo, Default,
)]
#[serde(rename_all = "snake_case")]
pub enum CollectiveAttribute {
    #[default]
    None,

    University,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, OperationIo, Default,
)]
#[serde(rename_all = "snake_case")]
pub enum VerifiableAttribute {
    #[default]
    None,

    Age,
    Gender,
}

impl fmt::Display for PanelAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PanelAttribute::None => write!(f, "none"),
            PanelAttribute::CollectiveAttribute(c) => write!(f, "collective_attribute:{}", c),
            PanelAttribute::VerifiableAttribute(v) => write!(f, "verifiable_attribute:{}", v),
        }
    }
}

impl Display for CollectiveAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CollectiveAttribute::None => write!(f, "none"),
            CollectiveAttribute::University => write!(f, "university"),
        }
    }
}

impl Display for VerifiableAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VerifiableAttribute::None => write!(f, "none"),
            VerifiableAttribute::Age => write!(f, "age"),
            VerifiableAttribute::Gender => write!(f, "gender"),
        }
    }
}

// fn panel_attr_key(attr: &PanelAttribute) -> &'static str {
//     match attr {
//         PanelAttribute::None => "none",
//         PanelAttribute::CollectiveAttribute(_) => "collective_attribute",
//         PanelAttribute::VerifiableAttribute(_) => "verifiable_attribute",
//     }
// }

// fn panel_attr_value(attr: &PanelAttribute) -> String {
//     match attr {
//         PanelAttribute::None => "none".to_string(),
//         PanelAttribute::CollectiveAttribute(c) => c.to_string(),
//         PanelAttribute::VerifiableAttribute(v) => v.to_string(),
//     }
// }
