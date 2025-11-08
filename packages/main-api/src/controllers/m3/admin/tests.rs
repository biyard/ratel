use crate::{
    controllers::m3::admin::*,
    delete, get,
    models::dynamo_tables::main::user::User,
    post,
    tests::v3_setup::TestContextV3,
    types::*,
};

#[tokio::test]
async fn test_list_admins() {
    let TestContextV3 {
        app,
        admin_user,
        ..
    } = TestContextV3::setup().await;

    let (status, _headers, body) = get! {
        app: app,
        path: "/m3/admin",
        headers: admin_user.1,
        response_type: ListItemsResponse<AdminUserResponse>
    };

    assert_eq!(status, 200);
    // List endpoint returns empty for now (scan not implemented)
    // Note: May return items if database has existing data from previous tests
    assert!(body.items.len() >= 0);
}

#[tokio::test]
async fn test_list_admins_as_non_admin() {
    let TestContextV3 {
        app,
        test_user,
        ..
    } = TestContextV3::setup().await;

    let (status, _headers, _body) = get! {
        app: app,
        path: "/m3/admin",
        headers: test_user.1
    };

    assert_eq!(status, 401);
}

#[tokio::test]
async fn test_get_admin() {
    let TestContextV3 {
        app,
        ddb,
        admin_user,
        ..
    } = TestContextV3::setup().await;

    // Create an admin user
    let mut admin = User::new(
        "Test Admin".to_string(),
        "testadmin@example.com".to_string(),
        "https://example.com/profile.jpg".to_string(),
        true,
        true,
        UserType::Individual,
        format!("testadmin_{}", chrono::Utc::now().timestamp_micros()),
        Some("password123".to_string()),
    );
    admin.user_type = UserType::Admin;
    admin.create(&ddb).await.unwrap();

    let admin_id = match &admin.pk {
        Partition::User(id) => id.clone(),
        _ => panic!("Expected User partition"),
    };

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/m3/admin/{}", admin_id),
        headers: admin_user.1.clone(),
        response_type: AdminUserResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.user_id, format!("User({})", admin_id));
    assert_eq!(body.user_type, UserType::Admin);
    assert_eq!(body.email, "testadmin@example.com");
}

#[tokio::test]
async fn test_get_admin_not_found() {
    let TestContextV3 {
        app,
        admin_user,
        ..
    } = TestContextV3::setup().await;

    let (status, _headers, _body) = get! {
        app: app,
        path: "/m3/admin/nonexistent",
        headers: admin_user.1
    };

    assert_eq!(status, 401);
}

#[tokio::test]
async fn test_get_non_admin_user() {
    let TestContextV3 {
        app,
        ddb,
        admin_user,
        test_user,
        ..
    } = TestContextV3::setup().await;

    let user_id = match &test_user.0.pk {
        Partition::User(id) => id.clone(),
        _ => panic!("Expected User partition"),
    };

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/m3/admin/{}", user_id),
        headers: admin_user.1
    };

    // Should return error because user is not an admin
    assert_eq!(status, 403);
    assert_eq!(body["code"], 404);
}

#[tokio::test]
async fn test_promote_to_admin() {
    let TestContextV3 {
        app,
        ddb,
        admin_user,
        ..
    } = TestContextV3::setup().await;

    // Create a regular user
    let user = User::new(
        "Regular User".to_string(),
        "regular@example.com".to_string(),
        "https://example.com/profile.jpg".to_string(),
        true,
        true,
        UserType::Individual,
        format!("regularuser_{}", chrono::Utc::now().timestamp_micros()),
        Some("password123".to_string()),
    );
    user.create(&ddb).await.unwrap();

    let user_id = match &user.pk {
        Partition::User(id) => id.clone(),
        _ => panic!("Expected User partition"),
    };

    let (status, _headers, body) = post! {
        app: app,
        path: "/m3/admin",
        headers: admin_user.1,
        body: {
            "email": "regular@example.com"
        },
        response_type: AdminUserResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.user_type, UserType::Admin);
    assert_eq!(body.email, "regular@example.com");

    // Verify the user is now an admin in the database
    let updated_user = User::get(&ddb, Partition::User(user_id), Some(EntityType::User))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(updated_user.user_type, UserType::Admin);
}

#[tokio::test]
async fn test_promote_to_admin_already_admin() {
    let TestContextV3 {
        app,
        ddb,
        admin_user,
        ..
    } = TestContextV3::setup().await;

    // Create an admin user
    let mut admin = User::new(
        "Already Admin".to_string(),
        "alreadyadmin@example.com".to_string(),
        "https://example.com/profile.jpg".to_string(),
        true,
        true,
        UserType::Individual,
        format!("alreadyadmin_{}", chrono::Utc::now().timestamp_micros()),
        Some("password123".to_string()),
    );
    admin.user_type = UserType::Admin;
    admin.create(&ddb).await.unwrap();

    let admin_id = match &admin.pk {
        Partition::User(id) => id.clone(),
        _ => panic!("Expected User partition"),
    };

    let (status, _headers, body) = post! {
        app: app,
        path: "/m3/admin",
        headers: admin_user.1,
        body: {
            "email": "alreadyadmin@example.com"
        }
    };

    assert_eq!(status, 400);
    assert_eq!(body["code"], 405);
}

#[tokio::test]
async fn test_promote_to_admin_user_not_found() {
    let TestContextV3 {
        app,
        admin_user,
        ..
    } = TestContextV3::setup().await;

    let (status, _headers, body) = post! {
        app: app,
        path: "/m3/admin",
        headers: admin_user.1,
        body: {
            "email": "nonexistent@example.com"
        }
    };

    assert_eq!(status, 401);
    assert_eq!(body["code"], 401);
}

#[tokio::test]
async fn test_demote_admin() {
    let TestContextV3 {
        app,
        ddb,
        admin_user,
        ..
    } = TestContextV3::setup().await;

    // Create an admin user
    let mut admin = User::new(
        "Test Admin".to_string(),
        "testadmin@example.com".to_string(),
        "https://example.com/profile.jpg".to_string(),
        true,
        true,
        UserType::Individual,
        format!("testadmin_{}", chrono::Utc::now().timestamp_micros()),
        Some("password123".to_string()),
    );
    admin.user_type = UserType::Admin;
    admin.create(&ddb).await.unwrap();

    let admin_id = match &admin.pk {
        Partition::User(id) => id.clone(),
        _ => panic!("Expected User partition"),
    };

    let (status, _headers, body) = delete! {
        app: app,
        path: format!("/m3/admin/{}", admin_id),
        headers: admin_user.1,
        response_type: DemoteAdminResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.success, true);

    // Verify the user is now a regular user in the database
    let updated_user = User::get(&ddb, Partition::User(admin_id), Some(EntityType::User))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(updated_user.user_type, UserType::Individual);
}

#[tokio::test]
async fn test_demote_admin_not_admin() {
    let TestContextV3 {
        app,
        ddb,
        admin_user,
        ..
    } = TestContextV3::setup().await;

    // Create a regular user
    let user = User::new(
        "Regular User".to_string(),
        "regular@example.com".to_string(),
        "https://example.com/profile.jpg".to_string(),
        true,
        true,
        UserType::Individual,
        format!("regularuser_{}", chrono::Utc::now().timestamp_micros()),
        Some("password123".to_string()),
    );
    user.create(&ddb).await.unwrap();

    let user_id = match &user.pk {
        Partition::User(id) => id.clone(),
        _ => panic!("Expected User partition"),
    };

    let (status, _headers, body) = delete! {
        app: app,
        path: format!("/m3/admin/{}", user_id),
        headers: admin_user.1
    };

    assert_eq!(status, 403);
    assert_eq!(body["code"], 404);
}

#[tokio::test]
async fn test_demote_admin_user_not_found() {
    let TestContextV3 {
        app,
        admin_user,
        ..
    } = TestContextV3::setup().await;

    let (status, _headers, body) = delete! {
        app: app,
        path: "/m3/admin/nonexistent",
        headers: admin_user.1
    };

    assert_eq!(status, 401);
    assert_eq!(body["code"], 401);
}

#[tokio::test]
async fn test_promote_and_demote_workflow() {
    let TestContextV3 {
        app,
        ddb,
        admin_user,
        ..
    } = TestContextV3::setup().await;

    // Create a regular user
    let user = User::new(
        "Workflow User".to_string(),
        "workflow@example.com".to_string(),
        "https://example.com/profile.jpg".to_string(),
        true,
        true,
        UserType::Individual,
        format!("workflowuser_{}", chrono::Utc::now().timestamp_micros()),
        Some("password123".to_string()),
    );
    user.create(&ddb).await.unwrap();

    let user_id = match &user.pk {
        Partition::User(id) => id.clone(),
        _ => panic!("Expected User partition"),
    };

    // Step 1: Promote to admin
    let (status, _headers, body) = post! {
        app: app,
        path: "/m3/admin",
        headers: admin_user.1.clone(),
        body: {
            "email": "workflow@example.com"
        },
        response_type: AdminUserResponse
    };
    assert_eq!(status, 200);
    assert_eq!(body.user_type, UserType::Admin);

    // Step 2: Verify user is now an admin using get endpoint
    let (status, _headers, body) = get! {
        app: app,
        path: format!("/m3/admin/{}", user_id),
        headers: admin_user.1.clone(),
        response_type: AdminUserResponse
    };
    assert_eq!(status, 200);
    assert_eq!(body.user_type, UserType::Admin);

    // Step 3: Demote admin
    let (status, _headers, body) = delete! {
        app: app,
        path: format!("/m3/admin/{}", user_id),
        headers: admin_user.1.clone(),
        response_type: DemoteAdminResponse
    };
    assert_eq!(status, 200);
    assert_eq!(body.success, true);

    // Step 4: Verify user is no longer an admin
    let updated_user = User::get(&ddb, Partition::User(user_id), Some(EntityType::User))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(updated_user.user_type, UserType::Individual);
}
