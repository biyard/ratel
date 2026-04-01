use std::collections::HashSet;

use crate::common::models::auth::UserFollow;
use crate::features::timeline::*;
use crate::features::timeline::models::{TimelineEntry, TimelineReason};

/// Collect all follower user_pks of `target_pk`, adding them to `entries` with a custom reason.
pub(super) async fn collect_followers_with_reason(
    cli: &aws_sdk_dynamodb::Client,
    target_pk: &Partition,
    seen: &mut HashSet<String>,
    entries: &mut Vec<(Partition, TimelineReason)>,
    reason: TimelineReason,
) -> Result<()> {
    let mut bookmark: Option<String> = None;
    loop {
        let mut opt = UserFollow::opt()
            .sk(EntityType::Follower(String::default()).to_string())
            .limit(100);
        if let Some(bk) = bookmark {
            opt = opt.bookmark(bk);
        }

        let (follows, next_bookmark) = UserFollow::query(cli, target_pk.clone(), opt).await?;
        for follow in follows {
            let key = follow.user_pk.to_string();
            if seen.insert(key) {
                entries.push((follow.user_pk, reason.clone()));
            }
        }

        bookmark = next_bookmark;
        if bookmark.is_none() {
            break;
        }
    }
    Ok(())
}

/// Collect follower PKs into `seen` without creating entries (for dedup purposes).
pub(super) async fn collect_follower_pks(
    cli: &aws_sdk_dynamodb::Client,
    target_pk: &Partition,
    seen: &mut HashSet<String>,
) -> Result<()> {
    let mut bookmark: Option<String> = None;
    loop {
        let mut opt = UserFollow::opt()
            .sk(EntityType::Follower(String::default()).to_string())
            .limit(100);
        if let Some(bk) = bookmark {
            opt = opt.bookmark(bk);
        }

        let (follows, next_bookmark) = UserFollow::query(cli, target_pk.clone(), opt).await?;
        for follow in follows {
            seen.insert(follow.user_pk.to_string());
        }

        bookmark = next_bookmark;
        if bookmark.is_none() {
            break;
        }
    }
    Ok(())
}

/// Write timeline entries in transact_write_items batches of 25.
pub(super) async fn write_timeline_entries(
    cli: &aws_sdk_dynamodb::Client,
    post_pk: &Partition,
    author_pk: &Partition,
    created_at: i64,
    entries: &[(Partition, TimelineReason)],
) -> Result<()> {
    for chunk in entries.chunks(25) {
        let transacts: Vec<aws_sdk_dynamodb::types::TransactWriteItem> = chunk
            .iter()
            .map(|(user_pk, reason)| {
                let entry =
                    TimelineEntry::new(user_pk, post_pk, author_pk, created_at, reason.clone());
                entry.create_transact_write_item()
            })
            .collect();

        cli.transact_write_items()
            .set_transact_items(Some(transacts))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to write timeline entries: {}", e);
                Error::InternalServerError("Failed to write timeline entries".into())
            })?;
    }

    Ok(())
}
