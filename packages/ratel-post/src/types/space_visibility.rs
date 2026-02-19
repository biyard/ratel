use crate::*;

use super::Visibility;

#[derive(
    Debug,
    Clone,
    SerializeDisplay,
    DeserializeFromStr,
    Default,
    DynamoEnum,
    PartialEq,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum SpaceVisibility {
    #[default]
    Private,
    Public,
    Team(String),
    TeamGroupMember(String),
}

impl Into<Visibility> for SpaceVisibility {
    fn into(self) -> Visibility {
        match self {
            SpaceVisibility::Private => Visibility::Private,
            SpaceVisibility::Public => Visibility::Public,
            SpaceVisibility::Team(team_pk) => Visibility::TeamOnly(team_pk),
            SpaceVisibility::TeamGroupMember(team_pk) => Visibility::TeamOnly(team_pk),
        }
    }
}
