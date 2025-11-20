use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, aide::OperationIo, JsonSchema)]
pub struct MarkAsReadResponse {
    pub success: bool,
    pub updated_count: usize,
}
