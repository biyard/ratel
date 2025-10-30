use bdk::prelude::*;

use super::Visibility;

#[derive(
    Debug,
    Clone,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
    Default,
    DynamoEnum,
    JsonSchema,
    PartialEq,
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

impl Into<Visibility> for SpaceVisibility {
    fn into(self) -> Visibility {
        match self {
            SpaceVisibility::Private => Visibility::Private, // Private spaces are treated as Public in post visibility context
            SpaceVisibility::Public => Visibility::Public,
            SpaceVisibility::Team(team_pk) => Visibility::TeamOnly(team_pk),
            SpaceVisibility::TeamGroupMember(team_pk) => Visibility::TeamOnly(team_pk),
        }
    }
}
