use super::*;
use crate::common::models::notification::UserInboxNotification;
use crate::common::types::InboxPayload;
use crate::common::utils::inbox::{create_inbox_row, create_inbox_row_once};

fn dummy_payload() -> InboxPayload {
    InboxPayload::MentionInComment {
        comment_preview: "hello".into(),
        mentioned_by_name: "alice".into(),
        cta_url: "/posts/abc".into(),
    }
}

#[tokio::test]
async fn test_create_inbox_row_persists_row() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();

    create_inbox_row(user_pk.clone(), dummy_payload())
        .await
        .unwrap();

    let (rows, _) = UserInboxNotification::query(
        &ctx.ddb,
        user_pk.clone(),
        UserInboxNotification::opt().sk("USER_INBOX_NOTIFICATION".to_string()),
    )
    .await
    .unwrap();
    assert_eq!(rows.len(), 1);
    assert!(!rows[0].is_read);
    assert!(rows[0].unread_created_at.starts_with("U#"));
}

#[tokio::test]
async fn test_create_inbox_row_once_dedups_on_same_source() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();

    create_inbox_row_once(user_pk.clone(), dummy_payload(), "comment-123")
        .await
        .unwrap();
    create_inbox_row_once(user_pk.clone(), dummy_payload(), "comment-123")
        .await
        .unwrap();

    let (rows, _) = UserInboxNotification::query(
        &ctx.ddb,
        user_pk.clone(),
        UserInboxNotification::opt().sk("USER_INBOX_NOTIFICATION".to_string()),
    )
    .await
    .unwrap();
    assert_eq!(rows.len(), 1, "second call should be deduped");
}

#[tokio::test]
async fn test_create_inbox_row_once_distinct_sources_produce_rows() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();

    create_inbox_row_once(user_pk.clone(), dummy_payload(), "comment-1")
        .await
        .unwrap();
    create_inbox_row_once(user_pk.clone(), dummy_payload(), "comment-2")
        .await
        .unwrap();

    let (rows, _) = UserInboxNotification::query(
        &ctx.ddb,
        user_pk.clone(),
        UserInboxNotification::opt().sk("USER_INBOX_NOTIFICATION".to_string()),
    )
    .await
    .unwrap();
    assert_eq!(rows.len(), 2);
}
