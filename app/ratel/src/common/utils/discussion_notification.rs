//! Unified fan-out for a new comment/reply on a space discussion.
//!
//! Fired as a single `NotificationData::DiscussionCommentPosted` row; recipient
//! resolution happens here at send time so the comment API stays fast. One pass
//! over a shared `seen` set guarantees **one notification per recipient per
//! comment**, with priority: mention > direct reply target > subscriber.
//! Mentions themselves are still created synchronously in the comment
//! controllers; here we only parse the mentioned pks to exclude them.

#[cfg(feature = "server")]
use crate::common::types::{EntityType, Partition, SpacePartition, SpacePostPartition};

#[cfg(feature = "server")]
#[allow(clippy::too_many_arguments)]
pub async fn send_discussion_comment_posted(
    space_id: &SpacePartition,
    discussion_id: &str,
    discussion_title: &str,
    comment_sk: &str,
    parent_comment_sk: Option<&str>,
    commenter_pk: &str,
    commenter_name: &str,
    comment_content: &str,
    cta_url: &str,
) -> crate::Result<()> {
    use crate::features::auth::models::EmailTemplate;
    use crate::features::auth::types::email_operation::EmailOperation;
    use crate::features::spaces::pages::actions::actions::discussion::SpacePostSubscription;
    use std::collections::HashSet;
    use std::str::FromStr;

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let ses = cfg.ses();

    let space_post_pk = SpacePostPartition(discussion_id.to_string());
    let post_pk: Partition = space_post_pk.clone().into();

    let comment_preview = crate::common::utils::reply_notification::build_preview(comment_content);

    // `seen` accumulates every pk already handled (higher-priority bucket wins).
    let mut seen: HashSet<String> = HashSet::new();
    seen.insert(commenter_pk.to_string());

    // Priority 1 — mentions: created synchronously elsewhere, just exclude them.
    for pk in crate::common::utils::mention::extract_mentioned_pks(comment_content) {
        seen.insert(pk);
    }

    // Priority 2 — direct reply targets (replies only).
    let mut reply_emails: Vec<String> = Vec::new();
    if let Some(parent_sk_str) = parent_comment_sk {
        if let Ok(parent_sk) = EntityType::from_str(parent_sk_str) {
            if let Some((_parent_content, parent_author_pk, reply_author_pks)) =
                crate::common::utils::reply_notification::fetch_space_discussion_thread(
                    cli, &post_pk, &parent_sk,
                )
                .await
            {
                let mut targets: Vec<Partition> = Vec::with_capacity(1 + reply_author_pks.len());
                targets.push(parent_author_pk);
                targets.extend(reply_author_pks);

                for pk in targets {
                    let pk_str = pk.to_string();
                    if !seen.insert(pk_str) {
                        continue;
                    }
                    let Some(email) = lookup_email(cli, &pk).await else {
                        continue;
                    };
                    let payload = crate::common::types::InboxPayload::ReplyOnComment {
                        space_id: Some(space_id.clone()),
                        post_id: None,
                        comment_preview: comment_preview.clone(),
                        replier_name: commenter_name.to_string(),
                        replier_profile_url: String::new(),
                        cta_url: cta_url.to_string(),
                    };
                    if let Err(e) = crate::common::utils::inbox::create_inbox_row_once(
                        pk.clone(),
                        payload,
                        comment_sk,
                    )
                    .await
                    {
                        crate::error!("discussion reply inbox row failed: {e}");
                    }
                    reply_emails.push(email);
                }
            }
        }
    }

    // Priority 3 — subscribers (direct partition Query, never a Scan).
    let mut subscriber_emails: Vec<String> = Vec::new();
    let opt = SpacePostSubscription::opt().sk(SpacePostSubscription::sk_prefix());
    let subs = match SpacePostSubscription::query(cli, post_pk.clone(), opt).await {
        Ok((items, _)) => items,
        Err(e) => {
            crate::error!("list discussion subscribers failed: {e}");
            Vec::new()
        }
    };

    for sub in subs {
        let pk = sub.user_pk;
        let pk_str = pk.to_string();
        if !seen.insert(pk_str) {
            continue;
        }
        let Some(email) = lookup_email(cli, &pk).await else {
            continue;
        };
        let payload = crate::common::types::InboxPayload::DiscussionCommentPosted {
            space_id: space_id.clone(),
            discussion_id: discussion_id.to_string(),
            discussion_title: discussion_title.to_string(),
            commenter_name: commenter_name.to_string(),
            commenter_profile_url: String::new(),
            comment_preview: comment_preview.clone(),
            cta_url: cta_url.to_string(),
        };
        if let Err(e) =
            crate::common::utils::inbox::create_inbox_row_once(pk.clone(), payload, comment_sk)
                .await
        {
            crate::error!("discussion subscriber inbox row failed: {e}");
        }
        subscriber_emails.push(email);
    }

    // Emails — one bulk send per bucket; each recipient is in at most one.
    if !reply_emails.is_empty() {
        let reply_preview =
            crate::common::utils::reply_notification::build_preview(comment_content);
        let operation = EmailOperation::ReplyOnCommentNotification {
            replier_name: commenter_name.to_string(),
            comment_preview: comment_preview.clone(),
            reply_preview,
            cta_url: cta_url.to_string(),
        };
        let template = EmailTemplate {
            targets: reply_emails,
            operation,
        };
        template.send_email(ses).await?;
    }

    if !subscriber_emails.is_empty() {
        let operation = EmailOperation::DiscussionCommentNotification {
            commenter_name: commenter_name.to_string(),
            discussion_title: discussion_title.to_string(),
            comment_preview,
            cta_url: cta_url.to_string(),
        };
        let template = EmailTemplate {
            targets: subscriber_emails,
            operation,
        };
        template.send_email(ses).await?;
    }

    Ok(())
}

#[cfg(feature = "server")]
async fn lookup_email(cli: &aws_sdk_dynamodb::Client, pk: &Partition) -> Option<String> {
    match crate::common::models::User::get(cli, pk, Some(EntityType::User)).await {
        Ok(Some(u)) if !u.email.is_empty() => Some(u.email),
        Ok(_) => None,
        Err(e) => {
            crate::error!("discussion notification user lookup failed for {pk}: {e}");
            None
        }
    }
}
