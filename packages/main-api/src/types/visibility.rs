use bdk::prelude::*;

use super::Partition;

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
    TeamOnly(String), // Only team members with permission can access
}

impl Visibility {
    pub fn public() -> Self {
        Visibility::Public
    }

    pub fn team_only(team_pk: Partition) -> Result<Self, crate::Error2> {
        if let Partition::Team(pk) = team_pk {
            Ok(Visibility::TeamOnly(pk))
        } else {
            Err(crate::Error2::IncorrectConfiguredVisibility(
                "Visibility::team_only requires a team Partition".into(),
            ))
        }
    }
}
