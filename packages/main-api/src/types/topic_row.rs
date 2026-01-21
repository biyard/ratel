use crate::*;

#[derive(
    Debug, Clone, Default, Serialize, Deserialize, aide::OperationIo, schemars::JsonSchema,
)]
pub struct TopicRow {
    pub topic: String,
    pub keyword: String,
}
