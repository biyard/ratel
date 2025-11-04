use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
pub struct BillingKeyRequest {
    pub store_id: String,
    pub channel_key: String,
    pub customer: CustomerRequest,
    pub method: MethodRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
pub struct CustomerRequest {
    pub id: String,
    pub name: CustomerName,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
pub struct CustomerName {
    pub full: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
pub struct MethodRequest {
    pub card: CardRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
pub struct CardRequest {
    pub credential: CardCredentialRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
pub struct CardCredentialRequest {
    pub number: String,
    pub expiry_year: String,
    pub expiry_month: String,
    pub birth_or_business_registration_number: String,
    pub password_two_digits: String,
}
