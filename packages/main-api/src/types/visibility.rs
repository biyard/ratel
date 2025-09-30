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
// None : is Invisible (Only owner can access)
// Public : is Visible to all
// Team : is Visible to team members with permission
pub enum Visibility {
    #[default]
    Public, // All user/team can access
    Team(String), // Only team members with permission can access
}
