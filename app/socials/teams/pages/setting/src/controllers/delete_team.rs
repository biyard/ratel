use crate::controllers::dto::DeleteTeamResponse;
use crate::*;

#[cfg(feature = "server")]
use aws_sdk_dynamodb::types::TransactWriteItem;
use ratel_post::models::{Team, TeamGroup, TeamOwner};

#[post("/api/teams/:teamname/settings/delete", user: ratel_auth::User)]
pub async fn delete_team_handler(teamname: String) -> Result<DeleteTeamResponse> {
    #[cfg(not(feature = "server"))]
    {
        let _ = teamname;
        return Err(Error::NotSupported(
            "Team delete is only available on server.".to_string(),
        ));
    }

    #[cfg(feature = "server")]
    {
        let conf = crate::config::get();
        let cli = conf.common.dynamodb();

        let gsi2_sk_prefix = Team::compose_gsi2_sk(String::default());
        let team_query_option = Team::opt().sk(gsi2_sk_prefix).limit(1);
        let (teams, _) =
            Team::find_by_username_prefix(cli, teamname.clone(), team_query_option).await?;

        let team = teams
            .into_iter()
            .find(|t| t.username == teamname)
            .ok_or(Error::NotFound("Team not found".to_string()))?;

        let team_pk = team.pk.clone();

        let team_owner = TeamOwner::get(cli, &team_pk, Some(&EntityType::TeamOwner))
            .await?
            .ok_or(Error::NotFound("Team owner not found".into()))?;

        if team_owner.user_pk != user.pk {
            return Err(Error::Unauthorized(
                "Only the team owner can delete a team".into(),
            ));
        }

        let mut transact_items: Vec<TransactWriteItem> = Vec::new();

        let mut team_group_bookmark: Option<String> = None;
        loop {
            let mut option = ratel_post::models::TeamGroupQueryOption::builder()
                .limit(50)
                .sk(EntityType::TeamGroup(String::default()).to_string());
            if let Some(b) = &team_group_bookmark {
                option = option.bookmark(b.clone());
            }
            let (team_groups, next) = TeamGroup::query(cli, team_pk.clone(), option).await?;
            for group in team_groups {
                transact_items.push(TeamGroup::delete_transact_write_item(
                    group.pk.clone(),
                    group.sk.clone(),
                ));
            }
            if next.is_none() {
                break;
            }
            team_group_bookmark = next;
        }

        let mut bookmark: Option<String> = None;
        loop {
            let mut option = ratel_auth::UserTeamQueryOption::builder();
            if let Some(b) = &bookmark {
                option = option.bookmark(b.clone());
            }
            let (user_teams, next) = ratel_auth::UserTeam::find_by_team(
                cli,
                &EntityType::UserTeam(team_pk.to_string()),
                option,
            )
            .await?;
            for user_team in user_teams {
                transact_items.push(ratel_auth::UserTeam::delete_transact_write_item(
                    user_team.pk,
                    user_team.sk,
                ));
            }
            if next.is_none() {
                break;
            }
            bookmark = next;
        }

        let mut user_group_bookmark: Option<String> = None;
        loop {
            let mut option = ratel_auth::UserTeamGroupQueryOption::builder().limit(50);
            if let Some(b) = &user_group_bookmark {
                option = option.bookmark(b.clone());
            }
            let (user_team_groups, next) =
                ratel_auth::UserTeamGroup::find_by_team_pk(cli, &team_pk, option).await?;
            for utg in user_team_groups {
                transact_items.push(ratel_auth::UserTeamGroup::delete_transact_write_item(
                    utg.pk, utg.sk,
                ));
            }
            if next.is_none() {
                break;
            }
            user_group_bookmark = next;
        }

        transact_items.push(TeamOwner::delete_transact_write_item(
            team_owner.pk.clone(),
            team_owner.sk.clone(),
        ));
        transact_items.push(Team::delete_transact_write_item(
            team.pk.clone(),
            team.sk.clone(),
        ));

        let deleted_count = transact_items.len();

        for chunk in transact_items.chunks(25) {
            cli.transact_write_items()
                .set_transact_items(Some(chunk.to_vec()))
                .send()
                .await
                .map_err(|e| {
                    tracing::error!("Failed to delete team entities: {}", e);
                    Error::InternalServerError("Failed to delete team".into())
                })?;
        }

        Ok(DeleteTeamResponse {
            message: format!("Team '{}' has been successfully deleted", teamname),
            deleted_count,
        })
    }
}
