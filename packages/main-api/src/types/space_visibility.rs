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
pub enum SpaceVisibility {
    #[default]
    Private,
    Public,
    // Only team members can access
    Team(String),

    // Only members in the specific team group can access
    TeamGroupMember(String),
}
