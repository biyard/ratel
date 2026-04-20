/// Helpers for sending "someone replied to your comment" email notifications.
///
/// Recipients are: the parent comment's author plus every user who has
/// previously posted a reply in that thread. The replier themselves is
/// always excluded.

#[cfg(feature = "server")]
use crate::common::types::Partition;

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
pub async fn create_reply_on_comment_notifications(
    cli: &aws_sdk_dynamodb::Client,
    recipient_pks: Vec<Partition>,
    replier_pk: &Partition,
    replier_name: &str,
    comment_preview: &str,
    reply_preview: &str,
    cta_url: &str,
) {
    let replier_pk_str = replier_pk.to_string();
    let mut seen = std::collections::HashSet::new();
    let mut emails = Vec::new();

    for pk in recipient_pks {
        let pk_str = pk.to_string();
        if pk_str == replier_pk_str {
            continue;
        }
        if !seen.insert(pk_str.clone()) {
            continue;
        }

        let user = match crate::common::models::User::get(
            cli,
            &pk,
            Some(crate::common::types::EntityType::User),
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
    }

    if emails.is_empty() {
        return;
    }

    let notification = crate::common::models::notification::Notification::new(
        crate::common::types::NotificationData::ReplyOnComment {
            emails,
            replier_name: replier_name.to_string(),
            comment_preview: comment_preview.to_string(),
            reply_preview: reply_preview.to_string(),
            cta_url: cta_url.to_string(),
        },
    );
    if let Err(e) = notification.create(cli).await {
        tracing::error!("Failed to create reply-on-comment notification: {}", e);
    }
}
