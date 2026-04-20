use super::*;
use crate::common::types::{InboxPayload, ListResponse};
use crate::common::utils::inbox::create_inbox_row;
use crate::features::notifications::types::{InboxNotificationResponse, UnreadCountResponse};

fn reply_payload(content: &str) -> InboxPayload {
    InboxPayload::ReplyOnComment {
        space_id: None,
        post_id: None,
        comment_preview: content.into(),
        replier_name: "bob".into(),
        replier_profile_url: String::new(),
        cta_url: "/posts/xyz".into(),
    }
}

#[tokio::test]
async fn test_list_inbox_returns_rows_for_current_user() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();

    create_inbox_row(user_pk.clone(), reply_payload("hi 1"))
        .await
        .unwrap();
    create_inbox_row(user_pk.clone(), reply_payload("hi 2"))
        .await
        .unwrap();

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/inbox",
        headers: ctx.test_user.1.clone(),
        response_type: ListResponse<InboxNotificationResponse>,
    };
    assert_eq!(status, 200, "list: {:?}", body);
    assert_eq!(body.items.len(), 2);
    assert!(body.items.iter().all(|i| !i.is_read));
}

#[tokio::test]
async fn test_list_inbox_unauthenticated_fails() {
    let ctx = TestContext::setup().await;
    let (status, _, _) = crate::test_get! {
        app: ctx.app,
        path: "/api/inbox",
    };
    assert_ne!(status, 200);
}

#[tokio::test]
async fn test_unread_count_reports_gsi_entries_capped_at_100() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();

    for i in 0..3 {
        create_inbox_row(user_pk.clone(), reply_payload(&format!("m{i}")))
            .await
            .unwrap();
    }

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/inbox/unread-count",
        headers: ctx.test_user.1.clone(),
        response_type: UnreadCountResponse,
    };
    assert_eq!(status, 200, "unread-count: {:?}", body);
    assert_eq!(body.count, 3);
}
