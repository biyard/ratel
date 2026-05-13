use crate::features::posts::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Clone, PartialEq, Eq, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub enum Visibility {
    #[default]
    Public,
    Private,
    TeamOnly(String),
}

impl From<SpaceVisibility> for Visibility {
    fn from(space_visibility: SpaceVisibility) -> Self {
        match space_visibility {
            SpaceVisibility::Private => Visibility::Private,
            SpaceVisibility::Public => Visibility::Public,
            SpaceVisibility::Team(team_pk) => Visibility::TeamOnly(team_pk),
        }
    }
}
