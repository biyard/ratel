use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
pub struct VerifiedCustomer {
    pub birth_date: String,
    pub gender: VerifiedGender,
    pub id: String,
    pub is_foreigner: bool,
    pub name: String,
    pub phone_number: String,
}

#[derive(
    Debug,
    Clone,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
    Default,
    DynamoEnum,
    JsonSchema,
    OperationIo,
)]
pub enum VerifiedGender {
    #[default]
    None,

    Male,
    Female,
}
