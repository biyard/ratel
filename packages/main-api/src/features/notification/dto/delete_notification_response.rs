use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, aide::OperationIo, JsonSchema, Default)]
pub struct DeleteNotificationResponse {
    pub success: bool,
}
