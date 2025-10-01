use bdk::prelude::*;
use sqlx::postgres::PgRow;

use crate::models::{feed::Post, user::User};
use dto::Feed as F;

use super::user::migrate_by_id;

pub async fn migrate_posts(
    cli: &aws_sdk_dynamodb::Client,
    pool: &sqlx::PgPool,
) -> Result<(), crate::Error2> {
    let mut total_count = 0;
    let posts = dto::Feed::query_builder(0)
        .feed_type_equals(dto::FeedType::Post)
        .query()
        .map(|row: PgRow| {
            use sqlx::Row;

            total_count = row.try_get("total_count").unwrap_or_default();
            row.into()
        })
        .fetch_all(pool)
        .await
        .expect("Failed to fetch posts from Postgres");

    tracing::info!("Total posts to migrate: {}", total_count);

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
            is_liked, // TODO: For user like document
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
            crate::Error2::InternalServerError(format!(
                "Post with ID {} has no associated author",
                id
            ))
        })?;

        let author = migrate_by_id(cli, pool, author.id).await?;

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
        post.user_pk = author.pk.clone();

        if let Err(e) = post.create(cli).await {
            tracing::error!("Failed to create post {}: {:?}", id, e);
        } else {
            tracing::info!("Successfully migrated post {}", id);
        }
    }

    Ok(())
}
