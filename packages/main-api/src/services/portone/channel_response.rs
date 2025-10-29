use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
pub struct ChannelResponse {
    pub id: String,
    pub name: String,
    pub key: String,
    pub pg_merchant_id: String,
    pub pg_provider: String,
    pub r#type: String,
}
