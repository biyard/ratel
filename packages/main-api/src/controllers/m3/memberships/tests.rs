use crate::features::membership::dto::*;
use crate::{
    controllers::m3::memberships::DeleteMembershipResponse,
    delete,
    features::membership::{Membership, MembershipTier, UserMembership},
    get, patch, post,
    tests::v3_setup::TestContextV3,
    types::*,
};

#[tokio::test]
async fn test_list_memberships() {
    let TestContextV3 {
        app,
        ddb,
        admin_user,
        ..
    } = TestContextV3::setup().await;

    // Create some memberships
    let membership1 = Membership::new(MembershipTier::Pro, 10, 100, 30, 1, -1);
    let membership2 = Membership::new(MembershipTier::Max, 20, 200, 365, 2, 1000);

    membership1.create(&ddb).await.unwrap();
    membership2.create(&ddb).await.unwrap();

    let (status, _headers, body) = get! {
        app: app,
        path: "/m3/memberships",
        headers: admin_user.1,
        response_type: ListItemsResponse<MembershipResponse>
    };

    assert_eq!(status, 200);
    assert!(body.items.len() >= 2);
}

#[tokio::test]
async fn test_list_memberships_as_non_admin() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let membership = Membership::new(MembershipTier::Pro, 10, 100, 30, 1, -1);
    membership.create(&ddb).await.unwrap();

    let (status, _headers, body) = get! {
        app: app,
        path: "/m3/memberships",
        headers: test_user.1
    };

    assert_eq!(status, 401);
    assert_eq!(body["code"], 403);
}

#[tokio::test]
async fn test_get_membership() {
    let TestContextV3 {
        app,
        ddb,
        admin_user,
        ..
    } = TestContextV3::setup().await;

    let membership = Membership::new(MembershipTier::Pro, 10, 100, 30, 1, -1);
    let membership_id = membership.get_id().unwrap();

    membership.create(&ddb).await.unwrap();

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/m3/memberships/{}", membership_id),
        headers: admin_user.1,
        response_type: MembershipResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.id, membership_id);
    assert_eq!(body.tier, MembershipTier::Pro);
    assert_eq!(body.price_dollars, 10);
    assert_eq!(body.max_credits_per_space, -1);
}

#[tokio::test]
async fn test_get_membership_as_non_admin() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let membership = Membership::new(MembershipTier::Pro, 10, 100, 30, 1, -1);
    let membership_id = membership.get_id().unwrap();

    membership.create(&ddb).await.unwrap();

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/m3/memberships/{}", membership_id),
        headers: test_user.1
    };

    assert_eq!(status, 401);
    assert_eq!(body["code"], 403);
}

#[tokio::test]
async fn test_get_nonexistent_membership() {
    let TestContextV3 {
        app, admin_user, ..
    } = TestContextV3::setup().await;

    let (status, _headers, _body) = get! {
        app: app,
        path: "/m3/memberships/nonexistent-id",
        headers: admin_user.1
    };

    assert_eq!(status, 404);
}

#[tokio::test]
async fn test_create_membership_as_admin() {
    let TestContextV3 {
        app, admin_user, ..
    } = TestContextV3::setup().await;

    let (status, _headers, body) = post! {
        app: app,
        path: "/m3/memberships",
        headers: admin_user.1,
        body: {
            "tier": "Pro",
            "price_dollars": 15,
            "credits": 150,
            "duration_days": 30,
            "display_order": 3,
            "max_credits_per_space": 500
        },
        response_type: MembershipResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.tier, MembershipTier::Pro);
    assert_eq!(body.price_dollars, 15);
    assert_eq!(body.credits, 150);
    assert_eq!(body.duration_days, 30);
    assert_eq!(body.max_credits_per_space, 500);
    assert!(body.is_active);
}

#[tokio::test]
async fn test_create_membership_as_non_admin() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    let (status, _headers, body) = post! {
        app: app,
        path: "/m3/memberships",
        headers: test_user.1,
        body: {
            "tier": "Pro",
            "price_dollars": 15,
            "credits": 150,
            "duration_days": 30,
            "display_order": 3,
            "max_credits_per_space": -1
        }
    };

    assert_eq!(status, 401);
    assert_eq!(body["code"], 403);
}

#[tokio::test]
async fn test_create_membership_without_auth() {
    let TestContextV3 { app, .. } = TestContextV3::setup().await;

    let (status, _headers, body) = post! {
        app: app,
        path: "/m3/memberships",
        body: {
            "tier": "Pro",
            "price_dollars": 15,
            "credits": 150,
            "duration_days": 30,
            "display_order": 3,
            "max_credits_per_space": -1
        }
    };

    assert_eq!(status, 401);
    assert_eq!(body["code"], 401);
}

#[tokio::test]
async fn test_update_membership_as_admin() {
    let TestContextV3 {
        app,
        admin_user,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let membership = Membership::new(MembershipTier::Pro, 10, 100, 30, 1, -1);
    let membership_id = membership.get_id().unwrap();

    membership.create(&ddb).await.unwrap();

    let (status, _headers, body) = patch! {
        app: app,
        path: format!("/m3/memberships/{}", membership_id),
        headers: admin_user.1,
        body: {
            "price_dollars": 25,
            "credits": 250,
            "max_credits_per_space": 1000
        },
        response_type: MembershipResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.price_dollars, 25);
    assert_eq!(body.credits, 250);
    assert_eq!(body.max_credits_per_space, 1000);
}

#[tokio::test]
async fn test_update_membership_as_non_admin() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let membership = Membership::new(MembershipTier::Pro, 10, 100, 30, 1, -1);
    let membership_id = membership.get_id().unwrap();

    membership.create(&ddb).await.unwrap();

    let (status, _headers, body) = patch! {
        app: app,
        path: format!("/m3/memberships/{}", membership_id),
        headers: test_user.1,
        body: {
            "price_dollars": 25
        }
    };

    assert_eq!(status, 401);
    assert_eq!(body["code"], 403);
}

#[tokio::test]
async fn test_delete_membership_as_admin() {
    let TestContextV3 {
        app,
        admin_user,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let membership = Membership::new(MembershipTier::Pro, 10, 100, 30, 1, -1);
    let membership_id = membership.get_id().unwrap();

    membership.create(&ddb).await.unwrap();

    let (status, _headers, body) = delete! {
        app: app,
        path: format!("/m3/memberships/{}", membership_id),
        headers: admin_user.1,
        response_type: DeleteMembershipResponse
    };

    assert_eq!(status, 200);
    assert!(body.success);

    // Verify deletion
    let result = Membership::get(
        &ddb,
        Partition::Membership(membership_id),
        Some(EntityType::Membership),
    )
    .await
    .unwrap();
    assert!(result.is_none());
}

#[tokio::test]
async fn test_delete_membership_as_non_admin() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let membership = Membership::new(MembershipTier::Pro, 10, 100, 30, 1, -1);
    let membership_id = membership.get_id().unwrap();

    membership.create(&ddb).await.unwrap();

    let (status, _headers, body) = delete! {
        app: app,
        path: format!("/m3/memberships/{}", membership_id),
        headers: test_user.1
    };

    assert_eq!(status, 401);
    assert_eq!(body["code"], 403);
}

#[tokio::test]
async fn test_create_membership_with_infinite_duration() {
    let TestContextV3 {
        app, admin_user, ..
    } = TestContextV3::setup().await;

    let (status, _headers, body) = post! {
        app: app,
        path: "/m3/memberships",
        headers: admin_user.1,
        body: {
            "tier": "Pro",
            "price_dollars": 99,
            "credits": 1000,
            "duration_days": -1,
            "display_order": 1,
            "max_credits_per_space": -1
        },
        response_type: MembershipResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.tier, MembershipTier::Pro);
    assert_eq!(body.duration_days, -1);
    assert_eq!(body.max_credits_per_space, -1);
    assert!(body.is_active);
}

#[tokio::test]
async fn test_create_membership_with_limited_credits_per_space() {
    let TestContextV3 {
        app, admin_user, ..
    } = TestContextV3::setup().await;

    let (status, _headers, body) = post! {
        app: app,
        path: "/m3/memberships",
        headers: admin_user.1,
        body: {
            "tier": "Max",
            "price_dollars": 199,
            "credits": 5000,
            "duration_days": 365,
            "display_order": 2,
            "max_credits_per_space": 500
        },
        response_type: MembershipResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.tier, MembershipTier::Max);
    assert_eq!(body.duration_days, 365);
    assert_eq!(body.credits, 5000);
    assert_eq!(body.max_credits_per_space, 500);
}

#[tokio::test]
async fn test_update_membership_to_infinite_duration() {
    let TestContextV3 {
        app,
        admin_user,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let membership = Membership::new(MembershipTier::Pro, 10, 100, 30, 1, -1);
    let membership_id = membership.get_id().unwrap();

    membership.create(&ddb).await.unwrap();

    let (status, _headers, body) = patch! {
        app: app,
        path: format!("/m3/memberships/{}", membership_id),
        headers: admin_user.1,
        body: {
            "duration_days": -1,
            "max_credits_per_space": 1000
        },
        response_type: MembershipResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.duration_days, -1);
    assert_eq!(body.max_credits_per_space, 1000);
}

#[tokio::test]
async fn test_user_membership_infinite_duration_logic() {
    let TestContextV3 { ddb, test_user, .. } = TestContextV3::setup().await;

    let membership = Membership::new(MembershipTier::Pro, 10, 100, -1, 1, -1);
    membership.create(&ddb).await.unwrap();

    let user_membership = UserMembership::new(
        test_user.0.pk.clone(),
        membership.pk.clone(),
        -1, // infinite duration
        100,
    )
    .unwrap();

    assert!(user_membership.is_infinite());
    assert!(!user_membership.is_expired());
    assert_eq!(user_membership.expired_at, i64::MAX);
}
