use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema, OperationIo)]
pub enum VerifiableAttribute {
    #[default]
    None,

    Age(i8),
    Gender(Gender),
    Generation(AgeGeneration),
    IsAdult,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema, OperationIo)]
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
