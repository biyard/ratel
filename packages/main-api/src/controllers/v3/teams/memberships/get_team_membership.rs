use crate::{
    features::membership::*,
    models::{team::Team, team::TeamOwner, user::User},
    types::EntityType,
    *,
};
use by_axum::aide::NoApi;

pub async fn get_team_membership_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Extension(team): Extension<Team>,
) -> Result<Json<TeamMembershipResponse>> {
    let cli = &dynamo.client;

    // Check if user is the team owner
    let is_owner = if let Some(ref user) = user {
        let team_owner = TeamOwner::get(cli, &team.pk, Some(&EntityType::TeamOwner)).await?;
        team_owner.map(|o| o.user_pk == user.pk).unwrap_or(false)
    } else {
        false
    };

    // Try to get existing team membership
    let team_membership = TeamMembership::get(cli, team.pk.clone(), Some(EntityType::TeamMembership))
        .await?;

    let team_membership = match team_membership {
        Some(mut membership) => {
            // Check and update expiration if needed
            if membership.check_and_update_expiration() {
                membership.upsert(cli).await?;
            }
            membership
        }
        None => {
            // Create a Free membership for the team if none exists
            let free_membership_pk = Partition::Membership(MembershipTier::Free.to_string());
            let free_membership = Membership::get(
                cli,
                free_membership_pk.clone(),
                Some(EntityType::Membership),
            )
            .await?
            .ok_or(Error::NoMembershipFound)?;

            let team_membership = TeamMembership::new(
                team.pk.clone().into(),
                free_membership.pk.clone().into(),
                free_membership.duration_days,
                free_membership.credits,
            )?;
            team_membership.create(cli).await?;
            team_membership
        }
    };

    Ok(Json(TeamMembershipResponse::from_team_membership(
        team_membership,
        is_owner,
    )))
}
