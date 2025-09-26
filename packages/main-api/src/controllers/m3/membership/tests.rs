#[cfg(test)]
mod tests {
    use crate::{
        Error2,
        controllers::m3::membership::{
            get_membership::get_user_membership,
            promote_to_admin::promote_user_to_admin,
            set_membership::{self, set_user_membership},
        },
        models::dynamo_tables::main::user::UserMembership,
        tests::{create_app_state, create_test_user, get_auth},
        types::Membership,
    };
    use dto::by_axum::axum::{
        Json,
        extract::{Extension, Path, State},
    };
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_get_user_membership_success() {
        let app_state = create_app_state();
        let cli = app_state.dynamo.client.clone();

        // Create a test user
        let user = create_test_user(&cli).await;
        let user_id = user
            .pk
            .to_string()
            .strip_prefix("USER#")
            .unwrap()
            .to_string();

        // Create admin user and auth
        let admin_user = create_test_user(&cli).await;
        let admin_auth = get_auth(&admin_user);

        // Create admin membership for the admin user
        let admin_membership = UserMembership::builder(admin_user.pk.to_string())
            .with_admin()
            .build();
        admin_membership.create(&cli).await.unwrap();

        // Create a membership for the target user
        let membership = UserMembership::builder(user_id.clone()).with_pro().build();
        membership.create(&cli).await.unwrap();

        // Test getting the membership
        let result = get_user_membership(
            State(app_state),
            Path(user_id.clone()),
            Extension(Some(admin_auth)),
        )
        .await;

        assert!(result.is_ok(), "Should successfully get user membership");
        let user_info = result.unwrap().0;
        assert_eq!(user_info.user_id, user_id);
        assert_eq!(user_info.membership_type, Membership::Pro);
        assert_eq!(user_info.email, user.email);
        assert_eq!(user_info.display_name, user.display_name);
    }

    #[tokio::test]
    async fn test_get_user_membership_default_free() {
        let app_state = create_app_state();
        let cli = app_state.dynamo.client.clone();

        // Create a test user without membership
        let user = create_test_user(&cli).await;
        let user_id = user
            .pk
            .to_string()
            .strip_prefix("USER#")
            .unwrap()
            .to_string();

        // Create admin user and auth
        let admin_user = create_test_user(&cli).await;
        let admin_auth = get_auth(&admin_user);

        // Create admin membership for the admin user
        let admin_membership = UserMembership::builder(admin_user.pk.to_string())
            .with_admin()
            .build();
        admin_membership.create(&cli).await.unwrap();

        // Test getting the membership (should default to Free)
        let result = get_user_membership(
            State(app_state),
            Path(user_id.clone()),
            Extension(Some(admin_auth)),
        )
        .await;

        assert!(result.is_ok(), "Should successfully get user membership");
        let user_info = result.unwrap().0;
        assert_eq!(user_info.user_id, user_id);
        assert_eq!(user_info.membership_type, Membership::Free);
    }

    #[tokio::test]
    async fn test_get_user_membership_unauthorized() {
        let app_state = create_app_state();
        let cli = app_state.dynamo.client.clone();

        // Create a test user
        let user = create_test_user(&cli).await;
        let user_id = user
            .pk
            .to_string()
            .strip_prefix("USER#")
            .unwrap()
            .to_string();

        // Create non-admin user and auth
        let regular_user = create_test_user(&cli).await;
        let regular_auth = get_auth(&regular_user);

        // Create free membership for the regular user (not admin)
        let regular_membership = UserMembership::builder(regular_user.pk.to_string())
            .with_free()
            .build();
        regular_membership.create(&cli).await.unwrap();

        // Test getting the membership with non-admin auth
        let result = get_user_membership(
            State(app_state),
            Path(user_id),
            Extension(Some(regular_auth)),
        )
        .await;

        assert!(result.is_err(), "Should fail with unauthorized error");
        match result.unwrap_err() {
            Error2::Unauthorized(_) => {}
            other => panic!("Expected Unauthorized error, got: {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_get_user_membership_user_not_found() {
        let app_state = create_app_state();
        let cli = app_state.dynamo.client.clone();

        // Create admin user and auth
        let admin_user = create_test_user(&cli).await;
        let admin_auth = get_auth(&admin_user);

        // Create admin membership for the admin user
        let admin_membership = UserMembership::builder(admin_user.pk.to_string())
            .with_admin()
            .build();
        admin_membership.create(&cli).await.unwrap();

        // Use a non-existent user ID
        let non_existent_user_id = "nonexistent-user-id".to_string();

        // Test getting the membership for non-existent user
        let result = get_user_membership(
            State(app_state),
            Path(non_existent_user_id),
            Extension(Some(admin_auth)),
        )
        .await;

        assert!(result.is_err(), "Should fail with not found error");
        match result.unwrap_err() {
            Error2::NotFound(_) => {}
            other => panic!("Expected NotFound error, got: {:?}", other),
        }
    }

    // Tests for set_user_membership
    #[tokio::test]
    async fn test_set_user_membership_success() {
        let app_state = create_app_state();
        let cli = app_state.dynamo.client.clone();

        // Create a test user
        let user = create_test_user(&cli).await;
        let user_id = user
            .pk
            .to_string()
            .strip_prefix("USER#")
            .unwrap()
            .to_string();

        // Create admin user and auth
        let admin_user = create_test_user(&cli).await;
        let admin_auth = get_auth(&admin_user);

        // Create admin membership for the admin user
        let admin_membership = UserMembership::builder(admin_user.pk.to_string())
            .with_admin()
            .build();
        admin_membership.create(&cli).await.unwrap();

        // Create request to set Pro membership
        let request = set_membership::SetMembershipRequest {
            membership_type: Membership::Pro,
            custom_capabilities: None,
        };

        // Test setting the membership
        let result = set_user_membership(
            State(app_state),
            Path(user_id.clone()),
            Extension(Some(admin_auth)),
            Json(request),
        )
        .await;

        assert!(result.is_ok(), "Should successfully set user membership");
        let response = result.unwrap().0;
        assert!(response.success);
        assert_eq!(response.user_id, user_id);
        assert_eq!(response.new_membership, Membership::Pro);
    }

    #[tokio::test]
    async fn test_set_user_membership_enterprise_with_custom_capabilities() {
        let app_state = create_app_state();
        let cli = app_state.dynamo.client.clone();

        // Create a test user
        let user = create_test_user(&cli).await;
        let user_id = user
            .pk
            .to_string()
            .strip_prefix("USER#")
            .unwrap()
            .to_string();

        // Create admin user and auth
        let admin_user = create_test_user(&cli).await;
        let admin_auth = get_auth(&admin_user);

        // Create admin membership for the admin user
        let admin_membership = UserMembership::builder(admin_user.pk.to_string())
            .with_admin()
            .build();
        admin_membership.create(&cli).await.unwrap();

        // Create custom capabilities for Enterprise
        let mut custom_capabilities = HashMap::new();
        custom_capabilities.insert(1u32, 100i32); // 1x booster: 100 spaces
        custom_capabilities.insert(2u32, 50i32); // 2x booster: 50 spaces

        // Create request to set Enterprise membership with custom capabilities
        let request = set_membership::SetMembershipRequest {
            membership_type: Membership::Enterprise,
            custom_capabilities: Some(custom_capabilities),
        };

        // Test setting the membership
        let result = set_user_membership(
            State(app_state),
            Path(user_id.clone()),
            Extension(Some(admin_auth)),
            Json(request),
        )
        .await;

        assert!(
            result.is_ok(),
            "Should successfully set Enterprise membership"
        );
        let response = result.unwrap().0;
        assert!(response.success);
        assert_eq!(response.user_id, user_id);
        assert_eq!(response.new_membership, Membership::Enterprise);
    }

    #[tokio::test]
    async fn test_set_user_membership_unauthorized() {
        let app_state = create_app_state();
        let cli = app_state.dynamo.client.clone();

        // Create a test user
        let user = create_test_user(&cli).await;
        let user_id = user
            .pk
            .to_string()
            .strip_prefix("USER#")
            .unwrap()
            .to_string();

        // Create non-admin user and auth
        let regular_user = create_test_user(&cli).await;
        let regular_auth = get_auth(&regular_user);

        // Create free membership for the regular user (not admin)
        let regular_membership = UserMembership::builder(regular_user.pk.to_string())
            .with_free()
            .build();
        regular_membership.create(&cli).await.unwrap();

        // Create request to set Pro membership
        let request = set_membership::SetMembershipRequest {
            membership_type: Membership::Pro,
            custom_capabilities: None,
        };

        // Test setting the membership with non-admin auth
        let result = set_user_membership(
            State(app_state),
            Path(user_id),
            Extension(Some(regular_auth)),
            Json(request),
        )
        .await;

        assert!(result.is_err(), "Should fail with unauthorized error");
        match result.unwrap_err() {
            Error2::Unauthorized(_) => {}
            other => panic!("Expected Unauthorized error, got: {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_set_user_membership_user_not_found() {
        let app_state = create_app_state();
        let cli = app_state.dynamo.client.clone();

        // Create admin user and auth
        let admin_user = create_test_user(&cli).await;
        let admin_auth = get_auth(&admin_user);

        // Create admin membership for the admin user
        let admin_membership = UserMembership::builder(admin_user.pk.to_string())
            .with_admin()
            .build();
        admin_membership.create(&cli).await.unwrap();

        // Use a non-existent user ID
        let non_existent_user_id = "nonexistent-user-id".to_string();

        // Create request to set Pro membership
        let request = set_membership::SetMembershipRequest {
            membership_type: Membership::Pro,
            custom_capabilities: None,
        };

        // Test setting the membership for non-existent user
        let result = set_user_membership(
            State(app_state),
            Path(non_existent_user_id),
            Extension(Some(admin_auth)),
            Json(request),
        )
        .await;

        assert!(result.is_err(), "Should fail with not found error");
        match result.unwrap_err() {
            Error2::NotFound(_) => {}
            other => panic!("Expected NotFound error, got: {:?}", other),
        }
    }

    // Tests for promote_user_to_admin
    #[tokio::test]
    async fn test_promote_user_to_admin_success() {
        let app_state = create_app_state();
        let cli = app_state.dynamo.client.clone();

        // Create a test user
        let user = create_test_user(&cli).await;
        let user_id = user
            .pk
            .to_string()
            .strip_prefix("USER#")
            .unwrap()
            .to_string();

        // Create existing admin user and auth
        let admin_user = create_test_user(&cli).await;
        let admin_auth = get_auth(&admin_user);

        // Create admin membership for the admin user
        let admin_membership = UserMembership::builder(admin_user.pk.to_string())
            .with_admin()
            .build();
        admin_membership.create(&cli).await.unwrap();

        // Create a Pro membership for the target user (to be promoted)
        let membership = UserMembership::builder(user_id.clone()).with_pro().build();
        membership.create(&cli).await.unwrap();

        // Test promoting the user to admin
        let result = promote_user_to_admin(
            State(app_state),
            Path(user_id.clone()),
            Extension(Some(admin_auth)),
        )
        .await;

        assert!(result.is_ok(), "Should successfully promote user to admin");
        let response = result.unwrap().0;
        assert!(response.success);
        assert_eq!(response.user_id, user_id);
        assert_eq!(response.new_membership, Membership::Admin);
    }

    #[tokio::test]
    async fn test_promote_user_to_admin_from_free() {
        let app_state = create_app_state();
        let cli = app_state.dynamo.client.clone();

        // Create a test user without existing membership
        let user = create_test_user(&cli).await;
        let user_id = user
            .pk
            .to_string()
            .strip_prefix("USER#")
            .unwrap()
            .to_string();

        // Create admin user and auth
        let admin_user = create_test_user(&cli).await;
        let admin_auth = get_auth(&admin_user);

        // Create admin membership for the admin user
        let admin_membership = UserMembership::builder(admin_user.pk.to_string())
            .with_admin()
            .build();
        admin_membership.create(&cli).await.unwrap();

        // Test promoting the user to admin (should work even without existing membership)
        let result = promote_user_to_admin(
            State(app_state),
            Path(user_id.clone()),
            Extension(Some(admin_auth)),
        )
        .await;

        assert!(
            result.is_ok(),
            "Should successfully promote user to admin from free"
        );
        let response = result.unwrap().0;
        assert!(response.success);
        assert_eq!(response.user_id, user_id);
        assert_eq!(response.new_membership, Membership::Admin);
    }

    #[tokio::test]
    async fn test_promote_user_to_admin_unauthorized() {
        let app_state = create_app_state();
        let cli = app_state.dynamo.client.clone();

        // Create a test user
        let user = create_test_user(&cli).await;
        let user_id = user
            .pk
            .to_string()
            .strip_prefix("USER#")
            .unwrap()
            .to_string();

        // Create non-admin user and auth
        let regular_user = create_test_user(&cli).await;
        let regular_auth = get_auth(&regular_user);

        // Create free membership for the regular user (not admin)
        let regular_membership = UserMembership::builder(regular_user.pk.to_string())
            .with_free()
            .build();
        regular_membership.create(&cli).await.unwrap();

        // Test promoting the user with non-admin auth
        let result = promote_user_to_admin(
            State(app_state),
            Path(user_id),
            Extension(Some(regular_auth)),
        )
        .await;

        assert!(result.is_err(), "Should fail with unauthorized error");
        match result.unwrap_err() {
            Error2::Unauthorized(_) => {}
            other => panic!("Expected Unauthorized error, got: {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_promote_user_to_admin_user_not_found() {
        let app_state = create_app_state();
        let cli = app_state.dynamo.client.clone();

        // Create admin user and auth
        let admin_user = create_test_user(&cli).await;
        let admin_auth = get_auth(&admin_user);

        // Create admin membership for the admin user
        let admin_membership = UserMembership::builder(admin_user.pk.to_string())
            .with_admin()
            .build();
        admin_membership.create(&cli).await.unwrap();

        // Use a non-existent user ID
        let non_existent_user_id = "nonexistent-user-id".to_string();

        // Test promoting non-existent user
        let result = promote_user_to_admin(
            State(app_state),
            Path(non_existent_user_id),
            Extension(Some(admin_auth)),
        )
        .await;

        assert!(result.is_err(), "Should fail with not found error");
        match result.unwrap_err() {
            Error2::NotFound(_) => {}
            other => panic!("Expected NotFound error, got: {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_promote_user_to_admin_no_auth() {
        let app_state = create_app_state();
        let cli = app_state.dynamo.client.clone();

        // Create a test user
        let user = create_test_user(&cli).await;
        let user_id = user
            .pk
            .to_string()
            .strip_prefix("USER#")
            .unwrap()
            .to_string();

        // Test promoting user without authentication
        let result = promote_user_to_admin(State(app_state), Path(user_id), Extension(None)).await;

        assert!(result.is_err(), "Should fail without authentication");
        match result.unwrap_err() {
            Error2::Unauthorized(_) => {}
            other => panic!("Expected Unauthorized error, got: {:?}", other),
        }
    }
}
