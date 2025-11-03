use super::*;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
pub struct IdentifyResponse {
    pub channel: ChannelResponse,
    pub id: String,
    pub pg_raw_response: String,
    pub pg_tx_id: String,
    pub requested_at: String,
    pub status: String,
    pub status_changed_at: String,
    pub updated_at: String,
    pub verified_at: String,
    pub verified_customer: VerifiedCustomer,
    pub version: String,
}
