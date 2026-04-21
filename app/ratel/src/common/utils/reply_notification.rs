/// Reply-on-comment notification helpers.
///
/// Recipient resolution (parent comment author + prior thread participants)
/// and email lookup happen at notification *send* time — not when the API
/// handler creates the notification — so the reply API stays fast. The
/// handler just fires a `NotificationData::ReplyOnComment` with identifiers
/// and the stream/event poller does the heavy work via `send_reply_on_comment`.
use serde::{Deserialize, Serialize};

/// Origin of a reply-on-comment notification. Determines which model the
/// parent comment and its prior replies live under.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum ReplyCommentSource {
    Post,
    SpaceDiscussion,
}

impl Default for ReplyCommentSource {
    fn default() -> Self {
        Self::Post
    }
}

const PREVIEW_MAX_CHARS: usize = 160;

pub fn build_preview(content: &str) -> String {
    let stripped = crate::common::utils::mention::strip_mention_markup(content);
    if stripped.chars().count() > PREVIEW_MAX_CHARS {
        let truncated: String = stripped.chars().take(PREVIEW_MAX_CHARS).collect();
        format!("{truncated}...")
    } else {
        stripped
    }
}

#[cfg(feature = "server")]
pub async fn send_reply_on_comment(
    source: &ReplyCommentSource,
    parent_comment_pk: &str,
    parent_comment_sk: &str,
    replier_pk: &str,
    replier_name: &str,
    reply_content: &str,
    cta_url: &str,
) -> crate::Result<()> {
    use crate::common::types::{EntityType, Partition};
    use crate::features::auth::models::EmailTemplate;
    use crate::features::auth::types::email_operation::EmailOperation;
    use std::str::FromStr;

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let ses = cfg.ses();

    let pk = match Partition::from_str(parent_comment_pk) {
        Ok(p) => p,
        Err(e) => {
            tracing::error!(
                "Invalid parent_comment_pk {}: {}",
                parent_comment_pk,
                e
            );
            return Ok(());
        }
    };
    let sk = match EntityType::from_str(parent_comment_sk) {
        Ok(s) => s,
        Err(e) => {
            tracing::error!(
                "Invalid parent_comment_sk {}: {}",
                parent_comment_sk,
                e
            );
            return Ok(());
        }
    };

    let thread = match source {
        ReplyCommentSource::Post => fetch_post_comment_thread(cli, &pk, &sk).await,
        ReplyCommentSource::SpaceDiscussion => {
            fetch_space_discussion_thread(cli, &pk, &sk).await
        }
    };

    let Some((parent_content, parent_author_pk, reply_author_pks)) = thread else {
        return Ok(());
    };

    let mut all_pks: Vec<Partition> = Vec::with_capacity(1 + reply_author_pks.len());
    all_pks.push(parent_author_pk);
    all_pks.extend(reply_author_pks);

    let mut seen = std::collections::HashSet::new();
    let mut emails: Vec<String> = Vec::new();

    let comment_preview = build_preview(&parent_content);

    for pk in all_pks {
        let pk_str = pk.to_string();
        if pk_str == replier_pk {
            continue;
        }
        if !seen.insert(pk_str.clone()) {
            continue;
        }

        let user = match crate::common::models::User::get(
            cli,
            &pk,
            Some(EntityType::User),
        )
        .await
        {
            Ok(Some(u)) => u,
            Ok(None) => {
                tracing::warn!("Reply-notification target user not found: {}", pk_str);
                continue;
            }
            Err(e) => {
                tracing::error!(
                    "Failed to look up reply-notification target user {}: {}",
                    pk_str,
                    e
                );
                continue;
            }
        };

        if user.email.is_empty() {
            continue;
        }
        emails.push(user.email);

        // Inbox row — idempotent per (recipient, parent_comment_sk#replier_pk).
        let payload = crate::common::types::InboxPayload::ReplyOnComment {
            space_id: None,
            post_id: None,
            comment_preview: comment_preview.clone(),
            replier_name: replier_name.to_string(),
            replier_profile_url: String::new(),
            cta_url: cta_url.to_string(),
        };
        let dedup_source = format!("{parent_comment_sk}#{replier_pk}");
        if let Err(e) = crate::common::utils::inbox::create_inbox_row_once(
            pk.clone(),
            payload,
            &dedup_source,
        )
        .await
        {
            crate::error!("reply inbox row failed: {e}");
        }
    }

    if emails.is_empty() {
        tracing::info!("Reply-on-comment: no recipients after filtering");
        return Ok(());
    }

    let reply_preview = build_preview(reply_content);

    let operation = EmailOperation::ReplyOnCommentNotification {
        replier_name: replier_name.to_string(),
        comment_preview,
        reply_preview,
        cta_url: cta_url.to_string(),
    };
    let template = EmailTemplate {
        targets: emails,
        operation,
    };
    template.send_email(ses).await?;

    Ok(())
}

#[cfg(feature = "server")]
async fn fetch_post_comment_thread(
    cli: &aws_sdk_dynamodb::Client,
    pk: &crate::common::types::Partition,
    sk: &crate::common::types::EntityType,
) -> Option<(
    String,
    crate::common::types::Partition,
    Vec<crate::common::types::Partition>,
)> {
    use crate::features::posts::models::PostComment;

    let parent = match PostComment::get(cli, pk, Some(sk.clone())).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            tracing::warn!("Parent post comment not found: {}/{}", pk, sk);
            return None;
        }
        Err(e) => {
            tracing::error!("Failed to load parent post comment: {}", e);
            return None;
        }
    };

    let mut author_pks: Vec<crate::common::types::Partition> = Vec::new();
    let mut bookmark: Option<String> = None;
    for _ in 0..5 {
        match PostComment::list_by_comment(cli, pk.clone(), sk.clone(), bookmark.clone()).await {
            Ok((items, next)) => {
                for r in items {
                    author_pks.push(r.author_pk);
                }
                if next.is_none() {
                    break;
                }
                bookmark = next;
            }
            Err(e) => {
                tracing::error!("PostComment::list_by_comment failed: {}", e);
                break;
            }
        }
    }

    Some((parent.content, parent.author_pk, author_pks))
}

#[cfg(feature = "server")]
async fn fetch_space_discussion_thread(
    cli: &aws_sdk_dynamodb::Client,
    pk: &crate::common::types::Partition,
    sk: &crate::common::types::EntityType,
) -> Option<(
    String,
    crate::common::types::Partition,
    Vec<crate::common::types::Partition>,
)> {
    use crate::features::spaces::pages::actions::actions::discussion::SpacePostComment;

    let parent = match SpacePostComment::get(cli, pk, Some(sk.clone())).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            tracing::warn!("Parent space post comment not found: {}/{}", pk, sk);
            return None;
        }
        Err(e) => {
            tracing::error!("Failed to load parent space post comment: {}", e);
            return None;
        }
    };

    let opt = SpacePostComment::opt_all()
        .scan_index_forward(false)
        .limit(100);
    let author_pks: Vec<crate::common::types::Partition> =
        match SpacePostComment::list_by_comment(cli, sk.clone(), opt).await {
            Ok((items, _)) => items.into_iter().map(|r| r.author_pk).collect(),
            Err(e) => {
                tracing::error!("SpacePostComment::list_by_comment failed: {}", e);
                Vec::new()
            }
        };

    Some((parent.content, parent.author_pk, author_pks))
}
