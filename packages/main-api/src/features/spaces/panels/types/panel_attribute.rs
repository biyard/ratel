use crate::features::did::VerifiableAttribute;
use crate::*;

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
    Age,
    Gender,
}

impl PanelAttribute {
    pub fn to_key(&self) -> String {
        match self {
            PanelAttribute::None
            | PanelAttribute::CollectiveAttribute(CollectiveAttribute::None)
            | PanelAttribute::VerifiableAttribute(VerifiableAttribute::None) => "none".to_string(),

            PanelAttribute::CollectiveAttribute(CollectiveAttribute::Age)
            | PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(_)) => "AGE".to_string(),
            PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(_))
            | PanelAttribute::CollectiveAttribute(CollectiveAttribute::Gender) => {
                "GENDER".to_string()
            }
            PanelAttribute::CollectiveAttribute(CollectiveAttribute::University) => {
                "UNIVERSITY".to_string()
            }
            PanelAttribute::VerifiableAttribute(VerifiableAttribute::Generation(_)) => {
                "GENERATION".to_string()
            }
            PanelAttribute::VerifiableAttribute(VerifiableAttribute::IsAdult(_)) => {
                "IS_ADULT".to_string()
            }
        }
    }

    pub fn to_value(&self) -> Option<String> {
        match self {
            PanelAttribute::None | PanelAttribute::CollectiveAttribute(_) => None,
            PanelAttribute::VerifiableAttribute(v) => match v {
                VerifiableAttribute::Age(Age::Specific(age)) => {
                    return Some(age.to_string());
                }
                VerifiableAttribute::Age(Age::Range {
                    inclusive_min,
                    inclusive_max,
                }) => {
                    return Some(format!("{}-{}", inclusive_min, inclusive_max));
                }
                VerifiableAttribute::Gender(g) => {
                    return Some(g.to_string());
                }
                VerifiableAttribute::Generation(g) => {
                    return Some(g.to_string());
                }
                VerifiableAttribute::IsAdult(is_adult) => {
                    if *is_adult {
                        return Some(format!("ADULT"));
                    } else {
                        return Some(format!("MINOR"));
                    }
                }
                VerifiableAttribute::None => None,
            },
        }
    }
}
