use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, aide::OperationIo, JsonSchema)]
pub struct DeleteNotificationResponse {
    pub success: bool,
}
