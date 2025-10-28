use bdk::prelude::*;

use super::Partition;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
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
    TeamOnly(String), // Only team members with permission can access
}

impl Visibility {
    pub fn public() -> Self {
        Visibility::Public
    }

    pub fn team_only(team_pk: Partition) -> Result<Self, crate::Error> {
        if let Partition::Team(pk) = team_pk {
            Ok(Visibility::TeamOnly(pk))
        } else {
            Err(crate::Error::PostIncorrectConfiguredVisibility(
                "Visibility::team_only requires a team Partition".into(),
            ))
        }
    }
}
