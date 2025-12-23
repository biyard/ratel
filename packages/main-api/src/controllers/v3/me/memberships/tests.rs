use super::*;
use crate::features::membership::*;
use crate::features::payment::*;
use crate::tests::v3_setup::*;
use change_membership::ChangeMembershipResponse;

pub async fn seed_test_user_payment(cli: &aws_sdk_dynamodb::Client, user_pk: &Partition) {
    // Create a test UserPayment with a fake billing_key for testing
    let mut user_payment = UserPayment::new(
        user_pk.clone(),
        format!("test_customer_{}", user_pk.to_string()),
        "Test User".to_string(),
        "900101".to_string(),
    );
    user_payment.billing_key = Some(format!("test_billing_key_{}", user_pk.to_string()));
    user_payment.create(cli).await.unwrap();
}

#[tokio::test]
async fn test_change_membership_upgrade_from_free_to_pro() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;
    let cli = &ddb;

    seed_test_user_payment(cli, &test_user.0.pk).await;

    // Upgrade to Pro
    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/me/memberships",
        headers: test_user.1.clone(),
        body: {
            "membership": "Pro",
            "currency": "USD"
        },
        response_type: serde_json::Value
    };
    if status != 200 {
        println!("Error response: {:?}", body);
    }
    assert_eq!(
        status, 200,
        "Failed to upgrade membership. Response: {:?}",
        body
    );

    // Verify the membership was upgraded
    let user_membership =
        UserMembership::get(cli, &test_user.0.pk, Some(EntityType::UserMembership))
            .await
            .unwrap()
            .expect("UserMembership should exist");

    let membership_pk: Partition = user_membership.membership_pk.clone().into();
    let membership = Membership::get(cli, membership_pk, Some(EntityType::Membership))
        .await
        .unwrap()
        .expect("Membership should exist");

    assert_eq!(membership.tier, MembershipTier::Pro);
    assert_eq!(user_membership.status, MembershipStatus::Active);
    assert!(user_membership.next_membership.is_none());
}

#[tokio::test]
async fn test_change_membership_downgrade_from_pro_to_free() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;
    let cli = &ddb;

    seed_test_user_payment(cli, &test_user.0.pk).await;

    // First upgrade to Pro
    let (status, _headers, _body) = post! {
        app: app,
        path: "/v3/me/memberships",
        headers: test_user.1.clone(),
        body: {
            "membership": "Pro",
            "currency": "USD"
        },
        response_type: ChangeMembershipResponse
    };
    assert_eq!(status, 200);

    // Now downgrade to Free
    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/me/memberships",
        headers: test_user.1.clone(),
        body: {
            "membership": "Free",
            "currency": "USD"
        },
        response_type: serde_json::Value
    };
    if status != 200 {
        println!("ERROR: Downgrade failed with status {}: {:?}", status, body);
    }
    assert_eq!(status, 200, "Downgrade request failed: {:?}", body);

    let body: ChangeMembershipResponse = serde_json::from_value(body).unwrap();
    assert_eq!(body.membership.as_ref().unwrap().tier, MembershipTier::Free);

    // Verify the downgrade was scheduled (not applied immediately)
    let user_membership =
        UserMembership::get(cli, &test_user.0.pk, Some(EntityType::UserMembership))
            .await
            .unwrap()
            .expect("UserMembership should exist");

    let membership_pk: Partition = user_membership.membership_pk.clone().into();
    let membership = Membership::get(cli, membership_pk, Some(EntityType::Membership))
        .await
        .unwrap()
        .expect("Membership should exist");

    // Current membership should still be Pro
    assert_eq!(membership.tier, MembershipTier::Pro);
    // But next_membership should be set to Free
    assert!(
        user_membership.next_membership.is_some(),
        "next_membership should be set to Free, but got: {:?}",
        user_membership.next_membership
    );
}

#[tokio::test]
async fn test_change_membership_to_same_tier_returns_error() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    // Try to change to Free (which is already the default)
    let (status, _headers, _body) = post! {
        app: app,
        path: "/v3/me/memberships",
        headers: test_user.1.clone(),
        body: {
            "membership": "Free",
            "currency": "USD"
        },
        response_type: serde_json::Value
    };
    assert_eq!(status, 400); // Should return error
}

#[tokio::test]
async fn test_change_membership_upgrade_adds_credits() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;
    let cli = &ddb;

    seed_test_user_payment(cli, &test_user.0.pk).await;

    // First, trigger the creation of Free membership by trying to change to Free
    // This will create a UserMembership if it doesn't exist
    let (status, _headers, _body) = post! {
        app: app,
        path: "/v3/me/memberships",
        headers: test_user.1.clone(),
        body: {
            "membership": "Pro",
            "currency": "USD"
        },
        response_type: ChangeMembershipResponse
    };

    // First upgrade should succeed (Free -> Pro)
    assert_eq!(status, 200);

    // Get credits after first upgrade
    let initial_membership =
        UserMembership::get(cli, &test_user.0.pk, Some(EntityType::UserMembership))
            .await
            .unwrap()
            .expect("UserMembership should exist after first upgrade");
    let initial_credits = initial_membership.remaining_credits;

    // Upgrade to Max to test credits are added again
    let (status, _headers, _body) = post! {
        app: app,
        path: "/v3/me/memberships",
        headers: test_user.1.clone(),
        body: {
            "membership": "Max",
            "currency": "USD"
        },
        response_type: ChangeMembershipResponse
    };
    assert_eq!(status, 200);

    // Verify credits were added
    let updated_membership =
        UserMembership::get(cli, &test_user.0.pk, Some(EntityType::UserMembership))
            .await
            .unwrap()
            .expect("UserMembership should exist");

    assert!(updated_membership.remaining_credits > initial_credits);
}

#[tokio::test]
async fn test_change_membership_upgrade_clears_scheduled_downgrade() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;
    let cli = &ddb;

    seed_test_user_payment(cli, &test_user.0.pk).await;

    // Upgrade to Pro
    let (status, _headers, _body) = post! {
        app: app,
        path: "/v3/me/memberships",
        headers: test_user.1.clone(),
        body: {
            "membership": "Pro",
            "currency": "USD"
        },
        response_type: ChangeMembershipResponse
    };
    assert_eq!(status, 200);

    // Schedule downgrade to Free
    let (status, _headers, _body) = post! {
        app: app,
        path: "/v3/me/memberships",
        headers: test_user.1.clone(),
        body: {
            "membership": "Free",
            "currency": "USD"
        },
        response_type: ChangeMembershipResponse
    };
    assert_eq!(status, 200);

    // Verify downgrade was scheduled
    let membership_with_downgrade =
        UserMembership::get(cli, &test_user.0.pk, Some(EntityType::UserMembership))
            .await
            .unwrap()
            .expect("UserMembership should exist");
    assert!(membership_with_downgrade.next_membership.is_some());

    // Now upgrade to Max
    let (status, _headers, _body) = post! {
        app: app,
        path: "/v3/me/memberships",
        headers: test_user.1.clone(),
        body: {
            "membership": "Max",
            "currency": "USD"
        },
        response_type: ChangeMembershipResponse
    };
    assert_eq!(status, 200);

    // Verify scheduled downgrade was cleared
    let final_membership =
        UserMembership::get(cli, &test_user.0.pk, Some(EntityType::UserMembership))
            .await
            .unwrap()
            .expect("UserMembership should exist");

    let membership_pk: Partition = final_membership.membership_pk.clone().into();
    let membership = Membership::get(cli, membership_pk, Some(EntityType::Membership))
        .await
        .unwrap()
        .expect("Membership should exist");

    assert_eq!(membership.tier, MembershipTier::Max);
    assert!(final_membership.next_membership.is_none());
}

#[tokio::test]
async fn test_change_membership_creates_purchase_record() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;

    // Seed memberships and user payment
    seed_test_user_payment(&ddb, &test_user.0.pk).await;

    // Upgrade to Pro
    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/me/memberships",
        headers: test_user.1.clone(),
        body: {
            "membership": "Pro",
            "currency": "USD"
        },
        response_type: ChangeMembershipResponse
    };
    assert_eq!(status, 200, "{body:?}");

    // Verify purchase record was created by checking purchase history
    let (status, _headers, body) = get! {
        app: app,
        path: "/v3/me/memberships/history",
        headers: test_user.1.clone(),
        response_type: serde_json::Value
    };
    assert_eq!(status, 200, "{body:?}");

    // Should have at least one purchase
    let purchases = body["items"]
        .as_array()
        .expect("Should have items field as array");
    assert!(!purchases.is_empty());
}

#[tokio::test]
async fn test_change_membership_without_auth_returns_error() {
    let TestContextV3 { app, .. } = TestContextV3::setup().await;

    // Try to change membership without authentication
    let (status, _headers, _body) = post! {
        app: app,
        path: "/v3/me/memberships",
        body: {
            "membership": "Pro",
            "currency": "USD"
        },
        response_type: serde_json::Value
    };
    assert_eq!(status, 401); // Unauthorized
}
