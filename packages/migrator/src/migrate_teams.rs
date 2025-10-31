use bdk::prelude::*;
use main_api::models::TeamOwner;

use crate::{
    models::{
        team::{Team, TeamGroup},
        user::*,
    },
    types::{EntityType, Partition},
};
use dto::Group as G;
use dto::User as U;

pub async fn migrate_teams(pool: &sqlx::PgPool, cli: &aws_sdk_dynamodb::Client) {
    let teams: Vec<dto::User> = dto::User::query_builder()
        .user_type_equals(dto::UserType::Team)
        .query()
        .map(Into::into)
        .fetch_all(pool)
        .await
        .expect("Failed to fetch teams from Postgres");
    tracing::info!("Total teams to migrate: {}", teams.len());

    for team in teams {
        let team_id = team.id;
        if let Err(e) = migrate_team(cli, team).await {
            tracing::error!("Failed to migrate team({}): {}", team_id, e);
        }
    }
}

pub async fn migrate_team(
    cli: &aws_sdk_dynamodb::Client,
    U {
        id,
        created_at,
        updated_at,
        nickname: display_name,
        profile_url,
        user_type: _,
        parent_id,
        username,
        followers_count,
        followings_count,
        groups,
        html_contents,
        followers: _,
        followings: _,
        membership: _,
        industry: _,
        ..
    }: U,
) -> Result<Team, crate::Error> {
    let team_pk = Partition::Team(id.to_string());
    let mut team = Team::new(display_name, profile_url, username.clone(), html_contents);
    team.created_at = created_at;
    team.updated_at = updated_at;
    team.pk = team_pk.clone();
    team.followers = followers_count;
    team.followings = followings_count;

    let user_pk = Partition::User(parent_id.unwrap().to_string());

    let user = User::get(cli, user_pk, Some(EntityType::User))
        .await
        .unwrap()
        .unwrap();

    if let Err(e) = team.create(cli).await {
        tracing::error!("Failed to create team {}: {:?}", id, e);
        return Err(e);
    }

    if let Err(e) = TeamOwner::new(team_pk.clone(), user.clone())
        .create(cli)
        .await
    {
        tracing::error!(
            "Failed to create team member for creator {}: {:?}",
            user.pk,
            e
        );
    }

    for group in groups {
        let G {
            id,
            created_at,
            updated_at: _,
            name,
            description,
            image_url: _,
            creator_id,
            member_count,
            members,
            permissions,
        } = group;
        let mut tg = TeamGroup::new(team_pk.clone(), name, description, permissions.into());

        let group_sk = EntityType::TeamGroup(id.to_string());
        tg.sk = group_sk;
        tg.created_at = created_at;
        tg.members = member_count;

        if let Err(e) = tg.create(cli).await {
            tracing::error!("Failed to create team group {}: {:?}", id, e);
        }
    }

    Ok(team)
}
