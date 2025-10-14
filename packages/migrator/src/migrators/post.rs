use crate::models::feed::PostLike;
use crate::types::sorted_visibility::SortedVisibility;
use crate::types::*;
use bdk::prelude::*;
use sqlx::postgres::PgRow;

use crate::models::feed::Post;
use crate::models::space::SpaceCommon;
use dto::Feed as F;
use dto::Space as S;

pub async fn migrate_posts(
    cli: &aws_sdk_dynamodb::Client,
    pool: &sqlx::PgPool,
    user: Option<crate::models::user::User>,
) -> Result<(), crate::Error2> {
    let mut total_count = 0;
    let user_id = if let Some(user) = &user {
        user.pk.try_into_inner()?.parse::<i64>().unwrap_or(0)
    } else {
        0
    };
    let posts = dto::Feed::query_builder(user_id)
        .feed_type_equals(dto::FeedType::Post)
        .limit(100)
        .with_count()
        .query()
        .map(|row: PgRow| {
            use sqlx::Row;

            total_count = row.try_get("total_count").unwrap_or_default();
            row.into()
        })
        .fetch_all(pool)
        .await
        .expect("Failed to fetch posts from Postgres");

    tracing::info!("Total posts to migrate: {} {}", total_count, posts.len());

    for post in posts {
        tracing::info!("Migrating post ID: {:?}", post);
        let F {
            id,
            created_at,
            updated_at,
            feed_type: _,     // All are Post type
            user_id: _,       // post.user_pk is used instead
            industry_id: _,   // industry is not stable
            parent_id: _,     // Post type does not use parent_id
            quote_feed_id: _, // Post type does not use quote_feed_id
            title,
            html_contents,
            url,
            url_type: _, // Skipped: url_type is not used in Post list view
            spaces,
            likes,
            is_liked,
            comments,
            comment_list: _, // NOTE: it will be migrated in getting specific posts
            files: _,        // Skipped: files are not used in Post list view
            rewards,
            shares,
            status,
            author,
            industry: _,      // industry is not stable
            is_bookmarked: _, // For user bookmark document
            onboard: _,       // deprecated
        } = post;

        let author = author.first().cloned().ok_or_else(|| {
            tracing::error!("Post with ID {} has no associated author", id);
            crate::Error2::InternalServerError(format!(
                "Post with ID {} has no associated author",
                id
            ))
        })?;

        tracing::info!("Migrating author: {:?}", author);

        let author: Author = match &author.user_type {
            &dto::UserType::Individual => super::user::migrate_by_id(cli, pool, author.id)
                .await
                .map_err(|e| {
                    tracing::error!(
                        "Failed to migrate author with ID {}: {}",
                        author.id,
                        e.to_string()
                    );
                    crate::Error2::InternalServerError(format!(
                        "Failed to migrate author with ID {}: {}",
                        author.id, e
                    ))
                })
                .unwrap()
                .into(),
            &dto::UserType::Team => super::team::migrate_by_id(cli, pool, author.id)
                .await
                .unwrap()
                .into(),
            _ => unimplemented!(),
        };

        let mut post = Post::new(
            title.unwrap_or_default(),
            html_contents,
            crate::types::PostType::Post,
            author.clone(),
        );

        post.likes = likes;
        post.comments = comments;
        post.shares = shares;
        post.rewards = if rewards == 0 { None } else { Some(rewards) };
        post.created_at = created_at;
        post.updated_at = updated_at;
        post.pk = crate::types::Partition::Feed(id.to_string());
        if let Some(url) = url {
            post.urls.push(url);
        }
        post.status = match status {
            dto::FeedStatus::Draft => crate::types::PostStatus::Draft,
            dto::FeedStatus::Published => crate::types::PostStatus::Published,
        };
        post.visibility = if post.status == crate::types::PostStatus::Draft {
            None
        } else {
            Some(crate::types::Visibility::Public)
        };
        post.sorted_visibility = if post.status == crate::types::PostStatus::Published {
            SortedVisibility::Public(post.created_at.to_string())
        } else {
            SortedVisibility::Draft(post.created_at.to_string())
        };
        post.user_pk = author.pk.clone();

        if spaces.len() > 0 {
            let space = &spaces[0];
            post.space_pk = Some(Partition::Space(space.id.to_string()));
            post.space_type = Some(match space.space_type {
                dto::SpaceType::Legislation => crate::types::SpaceType::Legislation,
                dto::SpaceType::Poll => crate::types::SpaceType::Poll,
                dto::SpaceType::Deliberation => crate::types::SpaceType::Deliberation,
                dto::SpaceType::Nft => crate::types::SpaceType::Nft,
                dto::SpaceType::Commitee => crate::types::SpaceType::Commitee,
                dto::SpaceType::SprintLeague => crate::types::SpaceType::SprintLeague,
                dto::SpaceType::Notice => crate::types::SpaceType::Notice,
                dto::SpaceType::Dagit => crate::types::SpaceType::Dagit,
            });
            post.booster = match space.booster_type.unwrap_or_default() {
                dto::BoosterType::NoBoost => Some(crate::types::BoosterType::NoBoost),
                dto::BoosterType::X2 => Some(crate::types::BoosterType::X2),
                dto::BoosterType::X10 => Some(crate::types::BoosterType::X10),
                dto::BoosterType::X100 => Some(crate::types::BoosterType::X100),
            };

            if let Some(ref space_type) = post.space_type {
                match space_type {
                    crate::types::SpaceType::Poll => {
                        // TODO: migrate poll space
                    }
                    crate::types::SpaceType::Commitee => {
                        // TODO: migrate committee space
                    }
                    crate::types::SpaceType::Notice => {
                        // TODO: migrate notice space
                    }
                    crate::types::SpaceType::Deliberation => {}
                    _ => unimplemented!("not used in production yet"),
                }
            }
        }

        if let Err(e) = post.create(cli).await {
            tracing::error!("Failed to create post {}: {:?}", id, e);
        } else {
            tracing::info!("Successfully migrated post {}", id);
        }

        if let Some(S {
            id,
            created_at,
            updated_at,
            booster_type,
            ..
        }) = spaces.first()
        {
            let space_pk = Partition::Space(id.to_string());
            let mut space = SpaceCommon::new(space_pk, post.pk.clone(), author.clone());
            space.created_at = *created_at;
            space.updated_at = *updated_at;
            space.booster = match booster_type.unwrap_or_default() {
                dto::BoosterType::NoBoost => BoosterType::NoBoost,
                dto::BoosterType::X2 => BoosterType::X2,
                dto::BoosterType::X10 => BoosterType::X10,
                dto::BoosterType::X100 => BoosterType::X100,
            };
            if let Err(e) = space.create(cli).await {
                tracing::error!(
                    "Failed to create space post document for post {} in space {}: {:?}",
                    id,
                    id,
                    e
                );
            } else {
                tracing::info!(
                    "Successfully created space post document for post {} in space {}",
                    id,
                    id
                );
            }
        }

        if is_liked {
            if let Err(e) = PostLike::new(post.pk.clone(), user.clone().unwrap().pk)
                .create(cli)
                .await
            {
                tracing::error!("Failed to create post like for post {}: {:?}", id, e);
            } else {
                tracing::info!("Successfully created post like for post {}", id);
            }
        }
    }

    Ok(())
}
