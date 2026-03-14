use crate::*;

#[derive(
    Debug,
    Clone,
    SerializeDisplay,
    DeserializeFromStr,
    Default,
    DynamoEnum,
    JsonSchema,
    OperationIo,
    PartialEq,
    Eq,
)]
pub enum EntityType {
    #[default]
    None,

    TS(i64),
}
