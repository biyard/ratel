use bdk::prelude::*;

use super::Partition;

#[derive(
    Debug,
    Clone,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
    DynamoEnum,
    JsonSchema,
    aide::OperationIo,
)]
pub enum SortedVisibility {
    Draft(String),
    Public(String),           // All user/team can access
    TeamOnly(String, String), // Only team members with permission can access
}

impl Default for SortedVisibility {
    fn default() -> Self {
        SortedVisibility::Draft(chrono::Utc::now().timestamp_micros().to_string())
    }
}

impl SortedVisibility {
    pub fn draft(now: i64) -> SortedVisibility {
        SortedVisibility::Draft(now.to_string())
    }

    pub fn public(now: i64) -> SortedVisibility {
        SortedVisibility::Public(now.to_string())
    }

    pub fn team_only(team_pk: Partition, now: i64) -> Result<SortedVisibility, crate::Error2> {
        let pk = match team_pk {
            Partition::Team(pk) => pk,
            _ => {
                return Err(crate::Error2::PostIncorrectConfiguredVisibility(
                    "SortedVisibility::team_only requires a team Partition".into(),
                ));
            }
        };

        Ok(SortedVisibility::TeamOnly(pk, now.to_string()))
    }
}
