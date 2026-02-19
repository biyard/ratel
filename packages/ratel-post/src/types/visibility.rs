use crate::*;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    SerializeDisplay,
    DeserializeFromStr,
    Default,
    DynamoEnum,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum Visibility {
    #[default]
    Public,
    Private,
    TeamOnly(String),
}
