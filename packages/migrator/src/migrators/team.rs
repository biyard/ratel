use bdk::prelude::*;

use crate::{
    models::{
        team::{Team, TeamGroup, team_member::TeamMember},
        user::*,
    },
    types::{EntityType, Partition},
};
use dto::Group as G;
use dto::User as U;

pub async fn migrate_by_id(
    cli: &aws_sdk_dynamodb::Client,
    pool: &sqlx::PgPool,
    id: i64,
) -> Result<Team, crate::Error> {
    migrate_team(
        cli,
        pool,
        dto::User::query_builder()
            .id_equals(id)
            .user_type_equals(dto::UserType::Team)
            .query()
            .map(dto::User::from)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                crate::Error::InternalServerError(format!(
                    "Failed to fetch user from Postgres: {}",
                    e
                ))
            })?,
    )
    .await
}

pub async fn migrate_team(
    cli: &aws_sdk_dynamodb::Client,
    pool: &sqlx::PgPool,
    U {
        id,
        created_at,
        updated_at,
        nickname: display_name,
        profile_url,
        user_type: _,
        parent_id: _,
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
    let mut team = Team::new(display_name, profile_url, username, html_contents);
    team.created_at = created_at;
    team.updated_at = updated_at;
    team.pk = team_pk.clone();
    team.followers = followers_count;
    team.followings = followings_count;

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

        let member_pk = Partition::User(creator_id.to_string());
        let u = User::get(cli, member_pk, Some(EntityType::User))
            .await
            .unwrap();
        let user = if u.is_none() {
            let user = super::user::migrate_by_id(cli, pool, creator_id).await;

            if let Err(ref e) = user {
                tracing::error!("Failed to migrate user {}: {:?}", creator_id, e);
                continue;
            } else {
                user.unwrap()
            }
        } else {
            u.unwrap()
        };

        if let Err(e) = TeamMember::new(team_pk.clone(), user).create(cli).await {
            tracing::error!(
                "Failed to create team member for creator {}: {:?}",
                creator_id,
                e
            );
        }

        for member in members {
            let member_pk = Partition::User(member.id.to_string());
            let u = User::get(cli, member_pk, Some(EntityType::User))
                .await
                .unwrap();
            let user = if u.is_none() {
                let user = super::user::migrate_by_id(cli, pool, member.id).await;

                if let Err(ref e) = user {
                    tracing::error!("Failed to migrate user {}: {:?}", creator_id, e);
                    continue;
                } else {
                    user.unwrap()
                }
            } else {
                u.unwrap()
            };

            if let Err(e) = TeamMember::new(team_pk.clone(), user).create(cli).await {
                tracing::error!(
                    "Failed to create team member for creator {}: {:?}",
                    creator_id,
                    e
                );
            }
        }
    }

    Ok(team)
}
