use super::super::dto::{TeamMemberResponse, TeamRole, UpdateMemberRoleRequest};
use super::super::*;

use crate::features::posts::models::{Team, TeamOwner};
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};

#[patch("/api/teams/:team_pk/members/role", user: crate::features::auth::User, team: Team, permissions: TeamGroupPermissions)]
pub async fn update_member_role_handler(
    team_pk: TeamPartition,
    body: UpdateMemberRoleRequest,
) -> Result<TeamMemberResponse> {
    let conf = super::super::config::get();
    let cli = conf.common.dynamodb();
    let team_pk: Partition = team_pk.into();

    // Permission check — same as add/remove.
    let can_edit = permissions.contains(TeamGroupPermission::TeamAdmin)
        || permissions.contains(TeamGroupPermission::TeamEdit)
        || permissions.contains(TeamGroupPermission::GroupEdit);
    if !can_edit {
        return Err(Error::NoPermission);
    }

    // Cannot change own role.
    if body.user_pk == user.pk.to_string() {
        return Err(MemberError::CannotChangeOwnRole.into());
    }

    // Cannot change owner's role.
    if let Some(team_owner) = TeamOwner::get(cli, &team_pk, Some(&EntityType::TeamOwner)).await? {
        if team_owner.user_pk.to_string() == body.user_pk {
            return Err(MemberError::CannotChangeOwnerRole.into());
        }
    }

    // Verify target user exists.
    let target_user = crate::features::auth::User::get(cli, &body.user_pk, Some(EntityType::User))
        .await?
        .ok_or(MemberError::UserNotFound)?;

    // Find all existing UserTeamGroup entries for this user in this team.
    let opt = crate::features::auth::UserTeamGroupQueryOption::builder().sk(target_user.pk.to_string());
    let (existing_groups, _) =
        crate::features::auth::UserTeamGroup::find_by_team_pk(cli, team_pk.clone(), opt).await?;

    if existing_groups.is_empty() {
        return Err(MemberError::UserNotFound.into());
    }

    let new_team_group_sk = EntityType::TeamGroup(body.role.to_string());
    let new_user_team_group_sk = EntityType::UserTeamGroup(new_team_group_sk.to_string());
    let new_permissions: i64 = match body.role {
        TeamRole::Admin => TeamGroupPermissions::all().into(),
        TeamRole::Member => TeamGroupPermissions::member().into(),
    };

    // Fast path: already in the requested role with the only matching group.
    let already_in_target_role = existing_groups.len() == 1
        && existing_groups[0].sk.to_string() == new_user_team_group_sk.to_string();

    if !already_in_target_role {
        // Build transact items: delete all existing UserTeamGroup rows, insert the new one.
        let mut transact_items: Vec<aws_sdk_dynamodb::types::TransactWriteItem> = existing_groups
            .iter()
            .map(|utg| {
                crate::features::auth::UserTeamGroup::delete_transact_write_item(
                    utg.pk.clone(),
                    utg.sk.clone(),
                )
            })
            .collect();

        let new_user_team_group = crate::features::auth::UserTeamGroup::new(
            target_user.pk.clone(),
            new_team_group_sk,
            new_permissions,
            team.pk.clone(),
        );
        transact_items.push(new_user_team_group.upsert_transact_write_item());

        crate::transact_write_all_items!(cli, transact_items);
    }

    Ok(TeamMemberResponse {
        user_id: target_user.pk.to_string(),
        username: target_user.username.clone(),
        display_name: target_user.display_name.clone(),
        profile_url: target_user.profile_url.clone(),
        role: body.role,
        is_owner: false,
    })
}
