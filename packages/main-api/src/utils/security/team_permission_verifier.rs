use crate::{
    models::{
        team::TeamOwner,
        user::{UserTeamGroup, UserTeamGroupQueryOption},
    },
    types::{EntityType, TeamGroupPermissions},
};

use super::*;

// #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub struct TeamPermissionVerifier {
    bit_mask: i64, // Merged Permissions from all user_team_groups in a team
}

impl TeamPermissionVerifier {
    pub async fn new(
        client: &aws_sdk_dynamodb::Client,
        user_pk: String,
        team_pk: String,
    ) -> Result<Self, Error> {
        // Check if user is the team owner
        let team_owner = TeamOwner::get(client, &team_pk, Some(EntityType::TeamOwner)).await?;

        if let Some(team_owner) = team_owner {
            if team_owner.user_pk.to_string() == user_pk {
                return Ok(Self { bit_mask: i64::MAX });
            }
        }

        // Or if user is in any group with the required permission
        let mut merged_permissions: i64 = 0;
        let mut bookmark: Option<String> = None;
        loop {
            let mut option = UserTeamGroupQueryOption::builder().sk(user_pk.clone());
            if let Some(b) = &bookmark {
                option = option.bookmark(b.clone());
            }
            let (user_team_groups, next) =
                UserTeamGroup::find_by_team_pk(client, &team_pk, option).await?;
            for utg in &user_team_groups {
                merged_permissions |= utg.team_group_permissions;
            }
            if next.is_none() {
                break;
            }
            bookmark = next;
        }

        tracing::debug!(
            "Found User-Team Groups {:?}",
            TeamGroupPermissions::from(merged_permissions)
        );
        Ok(Self {
            bit_mask: merged_permissions,
        })
    }
}

impl PermissionVerifier for TeamPermissionVerifier {
    fn has_all_permission(&self, required_bit_mask: i64) -> bool {
        (self.bit_mask & required_bit_mask) == required_bit_mask
    }

    fn has_any_permissions(&self, required_bit_mask: i64) -> bool {
        (self.bit_mask & required_bit_mask) != 0
    }
}
