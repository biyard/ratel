/// Mention markup format: @[display_name](user:PARTITION_KEY)
/// Example: "Hello @[John](user:USER#abc123) nice post"

#[derive(Debug, Clone, PartialEq)]
pub enum ContentSegment {
    Text(String),
    Mention {
        display_name: String,
        user_pk: String,
    },
}

pub fn parse_mention_segments(content: &str) -> Vec<ContentSegment> {
    let mut segments = Vec::new();
    let mut last_end = 0;
    let len = content.len();
    let mut i = 0;

    while i < len {
        if content.as_bytes()[i] == b'@' && i + 1 < len && content.as_bytes()[i + 1] == b'[' {
            if let Some((display_name, user_pk, end)) = try_parse_mention(content, i) {
                if i > last_end {
                    segments.push(ContentSegment::Text(content[last_end..i].to_string()));
                }
                segments.push(ContentSegment::Mention {
                    display_name,
                    user_pk,
                });
                last_end = end;
                i = end;
                continue;
            }
        }
        i += 1;
    }

    if last_end < len {
        segments.push(ContentSegment::Text(content[last_end..].to_string()));
    }

    segments
}

fn try_parse_mention(content: &str, start: usize) -> Option<(String, String, usize)> {
    let rest = &content[start..];
    if !rest.starts_with("@[") {
        return None;
    }
    let close_bracket = rest.find(']')?;
    let display_name = rest[2..close_bracket].to_string();
    if display_name.is_empty() {
        return None;
    }
    let after_bracket = &rest[close_bracket + 1..];
    if !after_bracket.starts_with("(user:") {
        return None;
    }
    let close_paren = after_bracket.find(')')?;
    let user_pk = after_bracket[6..close_paren].to_string();
    if user_pk.is_empty() {
        return None;
    }
    let total_len = close_bracket + 1 + close_paren + 1;
    Some((display_name, user_pk, start + total_len))
}

pub fn extract_mentioned_pks(content: &str) -> Vec<String> {
    let mut pks = Vec::new();
    for segment in parse_mention_segments(content) {
        if let ContentSegment::Mention { user_pk, .. } = segment {
            if !pks.contains(&user_pk) {
                pks.push(user_pk);
            }
        }
    }
    pks
}

pub fn strip_mention_markup(content: &str) -> String {
    let segments = parse_mention_segments(content);
    let mut result = String::new();
    for segment in segments {
        match segment {
            ContentSegment::Text(t) => result.push_str(&t),
            ContentSegment::Mention { display_name, .. } => {
                result.push('@');
                result.push_str(&display_name);
            }
        }
    }
    result
}

pub fn mention_markup(display_name: &str, user_pk: &str) -> String {
    format!("@[{display_name}](user:{user_pk}) ")
}

/// Display text to insert in TextArea (visible to user).
pub fn mention_display(display_name: &str) -> String {
    format!("@{display_name} ")
}

/// Convert display text back to markup before submission.
/// Replaces each `@display_name` with `@[display_name](user:pk)` using the tracked mentions list.
pub fn apply_mention_markup(display_text: &str, mentions: &[(String, String)]) -> String {
    let mut result = display_text.to_string();
    for (display_name, user_pk) in mentions {
        let display_pattern = format!("@{display_name}");
        let markup = format!("@[{display_name}](user:{user_pk})");
        result = result.replacen(&display_pattern, &markup, 1);
    }
    result
}

#[cfg(feature = "server")]
pub async fn create_mention_notifications(
    cli: &aws_sdk_dynamodb::Client,
    content: &str,
    author_pk: &crate::common::types::Partition,
    author_name: &str,
    cta_url: &str,
) {
    let mentioned_pks = extract_mentioned_pks(content);
    let author_pk_str = author_pk.to_string();
    let preview = strip_mention_markup(content);
    let preview = if preview.len() > 100 {
        format!("{}...", &preview[..100])
    } else {
        preview
    };

    for pk_str in mentioned_pks {
        if pk_str == author_pk_str {
            continue;
        }
        let notification = crate::common::models::notification::Notification::new(
            crate::common::types::NotificationData::MentionInComment {
                mentioned_by_name: author_name.to_string(),
                comment_preview: preview.clone(),
                cta_url: cta_url.to_string(),
            },
        );
        if let Err(e) = notification.create(cli).await {
            tracing::error!("Failed to create mention notification for {}: {}", pk_str, e);
        }
    }
}
