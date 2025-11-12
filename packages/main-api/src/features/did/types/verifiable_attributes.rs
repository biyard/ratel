use crate::*;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, JsonSchema, OperationIo,
)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum VerifiableAttribute {
    #[default]
    None,

    Age(Age),
    Gender(Gender),
    Generation(AgeGeneration),
    IsAdult(bool),
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    Default,
    JsonSchema,
    OperationIo,
    EnumProp,
)]
#[serde(rename_all = "snake_case")]
pub enum AgeGeneration {
    #[default]
    Teen,
    Twenties,
    Thirties,
    Forties,
    Fifties,
    Sixties,
    Seventies,
    Eighties,
    Nineties,
    Senior,
}

pub struct VerifiableAttributes(pub Vec<VerifiableAttribute>);

impl VerifiableAttributes {
    pub fn contains_age_generation(&self) -> bool {
        self.0
            .iter()
            .any(|attr| matches!(attr, VerifiableAttribute::Generation(_)))
    }
}
