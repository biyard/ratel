use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
pub struct VerifiedCustomer {
    pub birth_date: String,
    pub gender: String,
    pub id: String,
    pub is_foreigner: bool,
    pub name: String,
    pub phone_number: String,
}
