use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, aide::OperationIo, JsonSchema, Default)]
pub struct MarkAsReadResponse {
    pub success: bool,
    pub updated_count: usize,
}
