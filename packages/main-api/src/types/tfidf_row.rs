use crate::*;

#[derive(
    Debug, Clone, Default, Serialize, Deserialize, aide::OperationIo, schemars::JsonSchema,
)]
pub struct TfidfRow {
    pub keyword: String,
    pub tf_idf: f64,
}
