use bdk::prelude::*;

use crate::{
    models::{
        team::{Team, TeamGroup},
        user::*,
    },
    types::{EntityType, Partition, Theme},
    utils::password::hash_password,
};
use dto::Group as G;
use dto::Team as T;
use dto::User as U;

pub async fn migrate_users(pool: &sqlx::PgPool, cli: &aws_sdk_dynamodb::Client) {
    let users: Vec<dto::User> = dto::User::query_builder()
        .user_type_equals(dto::UserType::Individual)
        .query()
        .map(Into::into)
        .fetch_all(pool)
        .await
        .expect("Failed to fetch users from Postgres");

    tracing::info!("Total users to migrate: {}", users.len());

    for user in users {
        let user_id = user.id;
        if let Err(e) = migrate_user(cli, user).await {
            tracing::error!("Failed to migrate user({}): {}", user_id, e);
        }
    }
}

pub async fn migrate_user(
    cli: &aws_sdk_dynamodb::Client,
    U {
        id,
        created_at,
        updated_at,
        nickname,
        principal,
        email,
        profile_url,
        term_agreed,
        informed_agreed,
        user_type: _,
        parent_id: _,
        username,
        followers_count,
        followings_count,
        groups,
        teams,
        html_contents,
        followers,
        followings,
        badges,
        evm_address,
        password,
        membership: _,
        theme,
        points,
        referral_code,
        phone_number: _,
        phone: _,
        telegram_id,
        telegram_raw,
        industry: _,
    }: U,
) -> Result<User, crate::Error> {
    let pk = Partition::User(id.to_string());
    let mut user = User::new(
        nickname,
        email,
        profile_url,
        term_agreed,
        informed_agreed,
        crate::types::UserType::Individual,
        username,
        if password.is_empty() {
            None
        } else {
            let hashed_password = hash_password(&password);
            Some(hashed_password)
        },
    );
    user.pk = pk.clone();
    user.created_at = created_at;
    user.updated_at = updated_at;
    user.description = html_contents;
    user.followers_count = followers_count;
    user.followings_count = followings_count;
    user.points = points;

    if let Some(theme) = theme {
        user.theme =
            serde_json::from_str(&(theme as i32).to_string()).unwrap_or(Theme::SystemDefault)
    };

    let _ = user.create(cli).await;

    let user_principal = UserPrincipal::new(pk.clone(), principal);
    let _ = user_principal.create(cli).await;
    if !referral_code.is_empty() {
        let _ = UserReferralCode::new(pk.clone(), referral_code)
            .create(cli)
            .await;
    };

    if let Some(tid) = telegram_id {
        let _ = UserTelegram::new(pk.clone(), tid, telegram_raw)
            .create(cli)
            .await;
    };

    if !evm_address.is_empty() {
        let _ = UserEvmAddress::new(pk.clone(), evm_address)
            .create(cli)
            .await;
    };

    for T {
        id,
        nickname,
        profile_url,
        username,
        ..
    } in teams
    {
        let _ = UserTeam::new(
            pk.clone(),
            Team {
                display_name: nickname,
                profile_url,
                username,
                pk: Partition::Team(id.to_string()),
                ..Default::default()
            },
        )
        .create(cli)
        .await;
    }

    for G {
        id,
        creator_id,
        permissions,
        ..
    } in groups
    {
        let _ = UserTeamGroup::new(
            pk.clone(),
            TeamGroup {
                pk: Partition::Team(creator_id.to_string()),
                sk: EntityType::TeamGroup(id.to_string()),
                permissions,
                ..Default::default()
            },
        )
        .create(cli)
        .await;
    }

    let mut rels = vec![];
    for follower in followers {
        rels.push(UserRelationship::new(
            pk.clone(),
            Partition::User(follower.id.to_string()),
            crate::types::Relationship::Follower,
        ));
    }

    for following in followings {
        for rel in &mut rels {
            if rel.sk
                == EntityType::UserRelationship(
                    Partition::User(following.id.to_string()).to_string(),
                )
            {
                rel.relationship = crate::types::Relationship::Mutual;
                continue;
            }
        }

        rels.push(UserRelationship::new(
            pk.clone(),
            Partition::User(following.id.to_string()),
            crate::types::Relationship::Following,
        ));
    }

    // TODO: implement migration of badges
    let _ = badges;

    Ok(user)
}
