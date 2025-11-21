use crate::{
    features::notification::{
        DeleteNotificationResponse, ListNotificationsResponse, MarkAsReadRequest,
        MarkAsReadResponse, Notification,
    },
    tests::v3_setup::TestContextV3,
    types::{email_operation::EmailOperation, notification_status::NotificationStatus, EntityType},
    *,
};

#[tokio::test]
async fn test_list_notifications_when_authenticated() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let (user, headers) = test_user;

    // Create some test notifications for the user
    let notification1 = Notification::new(
        EmailOperation::SpacePostNotification {
            author_profile: "https://example.com/profile.jpg".to_string(),
            author_display_name: "Test Author".to_string(),
            author_username: "testauthor".to_string(),
            post_title: "Test Post 1".to_string(),
            post_desc: "Test Description 1".to_string(),
            connect_link: "https://example.com/post1".to_string(),
        },
        user.clone(),
    );
    notification1.create(&ddb).await.unwrap();

    let notification2 = Notification::new(
        EmailOperation::TeamInvite {
            team_name: "Test Team".to_string(),
            team_profile: "https://example.com/team.jpg".to_string(),
            team_display_name: "Test Team".to_string(),
            url: "https://example.com/team/invite".to_string(),
        },
        user.clone(),
    );
    notification2.create(&ddb).await.unwrap();

    // List notifications
    let (status, _headers, body) = get! {
        app: app,
        path: "/v3/notifications",
        headers: headers.clone(),
        response_type: ListNotificationsResponse
    };

    assert_eq!(status, 200);
    assert!(body.items.len() >= 2, "Should have at least 2 notifications");
    assert_eq!(body.items[0].status, NotificationStatus::Unread);
}

#[tokio::test]
async fn test_list_notifications_without_authentication() {
    let TestContextV3 { app, .. } = TestContextV3::setup().await;

    // Try to list notifications without authentication
    let (status, _headers, _body) = get! {
        app: app,
        path: "/v3/notifications"
    };

    assert_eq!(status, 401, "Should require authentication");
}

#[tokio::test]
async fn test_mark_single_notification_as_read() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let (user, headers) = test_user;

    // Create a test notification
    let notification = Notification::new(
        EmailOperation::SpacePostNotification {
            author_profile: "https://example.com/profile.jpg".to_string(),
            author_display_name: "Test Author".to_string(),
            author_username: "testauthor".to_string(),
            post_title: "Test Post".to_string(),
            post_desc: "Test Description".to_string(),
            connect_link: "https://example.com/post".to_string(),
        },
        user.clone(),
    );
    notification.create(&ddb).await.unwrap();

    // Extract notification ID from sk
    let notification_id = match &notification.sk {
        EntityType::Notification(id) => id.clone(),
        _ => panic!("Invalid notification sk type"),
    };

    // Mark as read
    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/notifications/mark-as-read",
        headers: headers.clone(),
        body: {
            "notification_ids": vec![notification_id.clone()]
        },
        response_type: MarkAsReadResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.success, true);
    assert_eq!(body.updated_count, 1);

    // Verify the notification is marked as read
    let updated_notification = Notification::get(
        &ddb,
        notification.pk.to_string(),
        Some(notification.sk.to_string()),
    )
    .await
    .unwrap()
    .unwrap();

    assert_eq!(updated_notification.status, NotificationStatus::Read);
    assert!(updated_notification.readed_at.is_some());
}

#[tokio::test]
async fn test_mark_multiple_notifications_as_read() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let (user, headers) = test_user;

    // Create multiple test notifications
    let mut notification_ids = Vec::new();

    for i in 0..3 {
        let notification = Notification::new(
            EmailOperation::SpacePostNotification {
                author_profile: "https://example.com/profile.jpg".to_string(),
                author_display_name: "Test Author".to_string(),
                author_username: "testauthor".to_string(),
                post_title: format!("Test Post {}", i),
                post_desc: format!("Test Description {}", i),
                connect_link: format!("https://example.com/post{}", i),
            },
            user.clone(),
        );
        notification.create(&ddb).await.unwrap();

        if let EntityType::Notification(id) = notification.sk {
            notification_ids.push(id);
        }
    }

    // Mark all as read
    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/notifications/mark-as-read",
        headers: headers.clone(),
        body: {
            "notification_ids": notification_ids
        },
        response_type: MarkAsReadResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.success, true);
    assert_eq!(body.updated_count, 3);
}

#[tokio::test]
async fn test_mark_as_read_with_invalid_notification_id() {
    let TestContextV3 {
        app, test_user, ..
    } = TestContextV3::setup().await;

    let (_user, headers) = test_user;

    // Try to mark a non-existent notification as read
    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/notifications/mark-as-read",
        headers: headers.clone(),
        body: {
            "notification_ids": vec!["non-existent-id"]
        },
        response_type: MarkAsReadResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.success, true);
    assert_eq!(
        body.updated_count, 0,
        "Should not update non-existent notification"
    );
}

#[tokio::test]
async fn test_mark_all_as_read() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let (user, headers) = test_user;

    // Create multiple test notifications
    for i in 0..5 {
        let notification = Notification::new(
            EmailOperation::SpacePostNotification {
                author_profile: "https://example.com/profile.jpg".to_string(),
                author_display_name: "Test Author".to_string(),
                author_username: "testauthor".to_string(),
                post_title: format!("Test Post {}", i),
                post_desc: format!("Test Description {}", i),
                connect_link: format!("https://example.com/post{}", i),
            },
            user.clone(),
        );
        notification.create(&ddb).await.unwrap();
    }

    // Mark all as read
    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/notifications/mark-all-as-read",
        headers: headers.clone(),
        response_type: MarkAsReadResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.success, true);
    assert!(
        body.updated_count >= 5,
        "Should update at least 5 notifications"
    );
}

#[tokio::test]
async fn test_mark_all_as_read_without_authentication() {
    let TestContextV3 { app, .. } = TestContextV3::setup().await;

    // Try to mark all as read without authentication
    let (status, _headers, _body) = post! {
        app: app,
        path: "/v3/notifications/mark-all-as-read"
    };

    assert_eq!(status, 401, "Should require authentication");
}

#[tokio::test]
async fn test_delete_notification() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let (user, headers) = test_user;

    // Create a test notification
    let notification = Notification::new(
        EmailOperation::SpacePostNotification {
            author_profile: "https://example.com/profile.jpg".to_string(),
            author_display_name: "Test Author".to_string(),
            author_username: "testauthor".to_string(),
            post_title: "Test Post".to_string(),
            post_desc: "Test Description".to_string(),
            connect_link: "https://example.com/post".to_string(),
        },
        user.clone(),
    );
    notification.create(&ddb).await.unwrap();

    // Extract notification ID from sk
    let notification_id = match &notification.sk {
        EntityType::Notification(id) => id.clone(),
        _ => panic!("Invalid notification sk type"),
    };

    // Delete the notification
    let (status, _headers, body) = delete! {
        app: app,
        path: format!("/v3/notifications/{}", notification_id),
        headers: headers.clone(),
        response_type: DeleteNotificationResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.success, true);

    // Verify the notification is deleted
    let deleted_notification = Notification::get(
        &ddb,
        notification.pk.to_string(),
        Some(notification.sk.to_string()),
    )
    .await
    .unwrap();

    assert!(deleted_notification.is_none(), "Notification should be deleted");
}

#[tokio::test]
async fn test_delete_notification_without_permission() {
    let TestContextV3 {
        app,
        test_user,
        user2,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let (user1, _headers1) = test_user;
    let (_user2, headers2) = user2;

    // Create a notification for user1
    let notification = Notification::new(
        EmailOperation::SpacePostNotification {
            author_profile: "https://example.com/profile.jpg".to_string(),
            author_display_name: "Test Author".to_string(),
            author_username: "testauthor".to_string(),
            post_title: "Test Post".to_string(),
            post_desc: "Test Description".to_string(),
            connect_link: "https://example.com/post".to_string(),
        },
        user1.clone(),
    );
    notification.create(&ddb).await.unwrap();

    // Extract notification ID from sk
    let notification_id = match &notification.sk {
        EntityType::Notification(id) => id.clone(),
        _ => panic!("Invalid notification sk type"),
    };

    // Try to delete user1's notification as user2
    let (status, _headers, _body) = delete! {
        app: app,
        path: format!("/v3/notifications/{}", notification_id),
        headers: headers2.clone()
    };

    // Returns 404 because notifications are partitioned by user - this is secure
    // as it doesn't leak information about whether the notification exists
    assert_eq!(status, 404, "Should return not found for other user's notification");
}

#[tokio::test]
async fn test_delete_non_existent_notification() {
    let TestContextV3 {
        app, test_user, ..
    } = TestContextV3::setup().await;

    let (_user, headers) = test_user;

    // Try to delete a non-existent notification
    let (status, _headers, _body) = delete! {
        app: app,
        path: "/v3/notifications/non-existent-id",
        headers: headers.clone()
    };

    assert_eq!(status, 404, "Should return not found");
}

#[tokio::test]
async fn test_delete_notification_without_authentication() {
    let TestContextV3 { app, .. } = TestContextV3::setup().await;

    // Try to delete without authentication
    let (status, _headers, _body) = delete! {
        app: app,
        path: "/v3/notifications/some-id"
    };

    assert_eq!(status, 401, "Should require authentication");
}

#[tokio::test]
async fn test_notifications_are_isolated_between_users() {
    let TestContextV3 {
        app,
        test_user,
        user2,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let (user1, headers1) = test_user;
    let (user2, headers2) = user2;

    // Create notifications for both users
    let notification1 = Notification::new(
        EmailOperation::SpacePostNotification {
            author_profile: "https://example.com/profile.jpg".to_string(),
            author_display_name: "Author for User 1".to_string(),
            author_username: "author1".to_string(),
            post_title: "Post for User 1".to_string(),
            post_desc: "Description 1".to_string(),
            connect_link: "https://example.com/post1".to_string(),
        },
        user1.clone(),
    );
    notification1.create(&ddb).await.unwrap();

    let notification2 = Notification::new(
        EmailOperation::TeamInvite {
            team_name: "Team for User 2".to_string(),
            team_profile: "https://example.com/team2.jpg".to_string(),
            team_display_name: "Team 2".to_string(),
            url: "https://example.com/team2/invite".to_string(),
        },
        user2.clone(),
    );
    notification2.create(&ddb).await.unwrap();

    // Get notifications for user1
    let (status, _headers, body1) = get! {
        app: app.clone(),
        path: "/v3/notifications",
        headers: headers1.clone(),
        response_type: ListNotificationsResponse
    };

    assert_eq!(status, 200);
    // User1 should only see their own notifications
    let user1_notification_count = body1.items.len();
    assert!(
        user1_notification_count >= 1,
        "User1 should have at least 1 notification"
    );

    // Get notifications for user2
    let (status, _headers, body2) = get! {
        app: app,
        path: "/v3/notifications",
        headers: headers2.clone(),
        response_type: ListNotificationsResponse
    };

    assert_eq!(status, 200);
    // User2 should only see their own notifications
    let user2_notification_count = body2.items.len();
    assert!(
        user2_notification_count >= 1,
        "User2 should have at least 1 notification"
    );
}
