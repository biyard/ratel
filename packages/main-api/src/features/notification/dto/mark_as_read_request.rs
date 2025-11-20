use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, aide::OperationIo, JsonSchema)]
pub struct MarkAsReadRequest {
    pub notification_ids: Vec<String>,
}
