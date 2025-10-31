use crate::types::sorted_visibility::SortedVisibility;
use crate::types::*;
use bdk::prelude::*;
use main_api::models::Team;
use main_api::models::User;
use sqlx::postgres::PgRow;

use crate::models::feed::Post;
use crate::models::space::SpaceCommon;
use dto::Feed as F;
use dto::Space as S;

pub async fn migrate_posts(pool: &sqlx::PgPool, cli: &aws_sdk_dynamodb::Client) {
    let posts: Vec<dto::Feed> = dto::Feed::query_builder(0)
        .feed_type_equals(dto::FeedType::Post)
        .query()
        .map(Into::into)
        .fetch_all(pool)
        .await
        .expect("Failed to fetch posts from Postgres");
    tracing::info!("Total posts to migrate: {}", posts.len());

    for post in posts {
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

        let author = author
            .first()
            .cloned()
            .ok_or_else(|| {
                tracing::error!("Post with ID {} has no associated author", id);
                crate::Error::InternalServerError(format!(
                    "Post with ID {} has no associated author",
                    id
                ))
            })
            .unwrap();

        let author: Author = match &author.user_type {
            &dto::UserType::Individual => User::get(
                cli,
                Partition::User(author.id.to_string()),
                Some(EntityType::User),
            )
            .await
            .unwrap()
            .unwrap()
            .into(),
            &dto::UserType::Team => Team::get(
                cli,
                Partition::Team(author.id.to_string()),
                Some(EntityType::Team),
            )
            .await
            .unwrap()
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
        post.user_pk = author.pk.clone();

        if spaces.len() > 0 {
            let space = &spaces[0];
            // NOTE: Space Id with feed id
            post.space_pk = Some(Partition::Space(id.to_string()));
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
            post.booster = None;
            if space.publishing_scope == dto::PublishingScope::Private {
                post.visibility = Some(crate::types::Visibility::Private);
            }
            post.space_visibility = Some(match space.publishing_scope {
                dto::PublishingScope::Private => crate::types::SpaceVisibility::Private,
                dto::PublishingScope::Public => crate::types::SpaceVisibility::Public,
            });
        }

        if let Err(e) = post.create(cli).await {
            tracing::error!("Failed to create post {}: {:?}", id, e);
        }
    }
}
