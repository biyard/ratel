use crate::features::posts::*;

#[derive(
    Debug, Clone, PartialEq, Eq, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
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
