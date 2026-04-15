use super::super::dto::{RemoveMemberRequest, RemoveMemberResponse};
use super::super::*;

use crate::features::posts::models::Team;

#[delete("/api/teams/:team_pk/members", user: crate::features::auth::User, team: Team, role: crate::features::social::pages::member::dto::TeamRole)]
pub async fn remove_team_member_handler(
    team_pk: TeamPartition,
    body: RemoveMemberRequest,
) -> Result<RemoveMemberResponse> {
    let conf = super::super::config::get();
    let cli = conf.common.dynamodb();
    let team_pk: Partition = team_pk.into();

    if !role.is_admin_or_owner() {
        return Err(Error::NoPermission);
    }

    let mut success_count = 0i64;
    let mut failed_pks = vec![];

    for member_pk in &body.user_pks {
        let user = crate::features::auth::User::get(cli, member_pk, Some(EntityType::User)).await?;
        let Some(user) = user else {
            failed_pks.push(member_pk.to_string());
            continue;
        };

        crate::features::auth::UserTeam::delete(
            cli,
            &user.pk,
            Some(EntityType::UserTeam(team_pk.to_string())),
        )
        .await?;

        success_count += 1;
    }

    Ok(RemoveMemberResponse {
        total_removed: success_count,
        failed_pks,
    })
}
