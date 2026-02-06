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
    SubPartition,
)]
pub enum Partition {
    #[default]
    None,

    ExampleData(String),
}
