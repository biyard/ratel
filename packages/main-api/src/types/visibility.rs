use bdk::prelude::*;

#[derive(
    Debug,
    Clone,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
    Default,
    DynamoEnum,
    JsonSchema,
)]
pub enum Visibility {
    #[default]
    Public, // All user/team can access
    Team(String), // Only team members with permission can access
}
