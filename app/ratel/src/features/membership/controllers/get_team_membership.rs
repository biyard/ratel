use super::*;
use crate::features::auth::User;
use crate::features::membership::controllers::normalize_error;
#[cfg(feature = "server")]
use crate::features::membership::models::ensure_team_membership_monthly_refill;
use crate::features::membership::models::{
    Membership, MembershipTier, TeamMembership, TeamMembershipResponse,
};
use crate::features::membership::*;
use crate::features::posts::models::Team;
use crate::features::social::pages::member::dto::TeamRole;

#[get("/v3/teams/:username/memberships", user: User, team: Team, role: TeamRole)]
pub async fn get_team_membership_handler(username: String) -> Result<TeamMembershipResponse> {
    let result = async {
        let conf = crate::features::membership::config::get();
        let cli = conf.common.dynamodb();
        let _ = user;

        if !role.is_admin_or_owner() {
            return Err(Error::NotFound("Permission denied".to_string()));
        }

        let team_membership =
            TeamMembership::get(cli, team.pk.clone(), Some(EntityType::TeamMembership)).await?;

        let (team_membership, membership) = match team_membership {
            Some(team_membership) => {
                let team_membership =
                    ensure_team_membership_monthly_refill(cli, team_membership).await?;
                let membership_pk: Partition = team_membership.membership_pk.clone().into();
                let membership = Membership::get(cli, membership_pk, Some(EntityType::Membership))
                    .await?
                    .ok_or_else(|| Error::NotFound("Membership not found".to_string()))?;
                (team_membership, membership)
            }
            None => {
                let free_membership_pk = Partition::Membership(MembershipTier::Free.to_string());
                let free_membership = Membership::get(
                    cli,
                    free_membership_pk.clone(),
                    Some(EntityType::Membership),
                )
                .await?
                .ok_or_else(|| Error::NotFound("Membership not found".to_string()))?;

                let team_membership = TeamMembership::new(
                    team.pk.clone().into(),
                    free_membership.pk.clone().into(),
                    free_membership.duration_days,
                    free_membership.credits,
                )?;
                team_membership.create(cli).await?;

                (team_membership, free_membership)
            }
        };

        let mut resp: TeamMembershipResponse = team_membership.into();
        resp.max_credits_per_space = membership.max_credits_per_space;
        Ok(resp)
    }
    .await;

    result.map_err(normalize_error)
}
