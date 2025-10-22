use crate::features::membership::dto::*;
use crate::{
    controllers::m3::memberships::DeleteMembershipResponse,
    delete,
    features::membership::{Membership, MembershipTier},
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
    let membership1 = Membership::new(MembershipTier::Pro, 10, 100, 30, 1);
    let membership2 = Membership::new(MembershipTier::Max, 20, 200, 365, 2);

    membership1.create(&ddb).await.unwrap();
    membership2.create(&ddb).await.unwrap();

    let (status, _headers, body) = get! {
        app: app,
        path: "/m3/memberships",
        headers: admin_user.1,
        response_type: ListMembershipsResponse
    };

    assert_eq!(status, 200);
    assert!(body.total >= 2);
    assert!(body.memberships.len() >= 2);
}

#[tokio::test]
async fn test_list_memberships_as_non_admin() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let membership = Membership::new(MembershipTier::Pro, 10, 100, 30, 1);
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

    let membership = Membership::new(MembershipTier::Pro, 10, 100, 30, 1);
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
    assert_eq!(body.price_dollers, 10);
}

#[tokio::test]
async fn test_get_membership_as_non_admin() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let membership = Membership::new(MembershipTier::Pro, 10, 100, 30, 1);
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
            "price_dollers": 15,
            "credits": 150,
            "duration_days": 30,
            "display_order": 3
        },
        response_type: MembershipResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.tier, MembershipTier::Pro);
    assert_eq!(body.price_dollers, 15);
    assert_eq!(body.credits, 150);
    assert_eq!(body.duration_days, 30);
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
            "price_dollers": 15,
            "credits": 150,
            "duration_days": 30,
            "display_order": 3
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
            "price_dollers": 15,
            "credits": 150,
            "duration_days": 30,
            "display_order": 3
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

    let membership = Membership::new(MembershipTier::Pro, 10, 100, 30, 1);
    let membership_id = membership.get_id().unwrap();

    membership.create(&ddb).await.unwrap();

    let (status, _headers, body) = patch! {
        app: app,
        path: format!("/m3/memberships/{}", membership_id),
        headers: admin_user.1,
        body: {
            "price_dollers": 25,
            "credits": 250
        },
        response_type: MembershipResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.price_dollers, 25);
    assert_eq!(body.credits, 250);
}

#[tokio::test]
async fn test_update_membership_as_non_admin() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let membership = Membership::new(MembershipTier::Pro, 10, 100, 30, 1);
    let membership_id = membership.get_id().unwrap();

    membership.create(&ddb).await.unwrap();

    let (status, _headers, body) = patch! {
        app: app,
        path: format!("/m3/memberships/{}", membership_id),
        headers: test_user.1,
        body: {
            "price_dollers": 25
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

    let membership = Membership::new(MembershipTier::Pro, 10, 100, 30, 1);
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

    let membership = Membership::new(MembershipTier::Pro, 10, 100, 30, 1);
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
