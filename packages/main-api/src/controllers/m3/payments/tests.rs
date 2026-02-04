use super::*;
use crate::{get, tests::v3_setup::TestContextV3, types::*};

#[tokio::test]
async fn test_list_payments_admin_success() {
    let TestContextV3 {
        app, admin_user, ..
    } = TestContextV3::setup().await;

    let (status, _headers, body) = get! {
        app: app,
        path: "/m3/payments",
        headers: admin_user.1,
        response_type: ListItemsResponse<AdminPaymentDetail>
    };

    assert_eq!(status, 200, "Admin should be able to access payment list");
    assert!(body.items.len() >= 0, "Should return payment items");
    assert!(body.bookmark.is_some(), "Should return bookmark");
}

#[tokio::test]
async fn test_list_payments_non_admin() {
    let TestContextV3 {
        app, test_user, ..
    } = TestContextV3::setup().await;

    let (status, _headers, _body) = get! {
        app: app,
        path: "/m3/payments",
        headers: test_user.1
    };

    assert_eq!(
        status, 401,
        "Non-admin user should receive 401 Unauthorized"
    );
}

#[tokio::test]
async fn test_list_payments_unauthenticated() {
    let TestContextV3 { app, .. } = TestContextV3::setup().await;

    let (status, _headers, _body) = get! {
        app: app,
        path: "/m3/payments"
    };

    assert_eq!(status, 401, "Unauthenticated should receive 401");
}

#[tokio::test]
async fn test_list_payments_bookmark_structure() {
    let TestContextV3 {
        app, admin_user, ..
    } = TestContextV3::setup().await;

    let (status, _headers, body) = get! {
        app: app,
        path: "/m3/payments",
        headers: admin_user.1,
        response_type: ListItemsResponse<AdminPaymentDetail>
    };

    assert_eq!(status, 200);

    let bookmark_str = body.bookmark.expect("Should have bookmark");
    let bookmark: serde_json::Value =
        serde_json::from_str(&bookmark_str).expect("Bookmark should be valid JSON");

    assert!(bookmark.get("page").is_some(), "Bookmark should have page");
    assert!(
        bookmark.get("page_size").is_some(),
        "Bookmark should have page_size"
    );
    assert!(
        bookmark.get("total_count").is_some(),
        "Bookmark should have total_count"
    );
    assert!(
        bookmark.get("total_pages").is_some(),
        "Bookmark should have total_pages"
    );
}

#[tokio::test]
async fn test_list_payments_response_structure() {
    let TestContextV3 {
        app, admin_user, ..
    } = TestContextV3::setup().await;

    let (status, _headers, body) = get! {
        app: app,
        path: "/m3/payments",
        headers: admin_user.1,
        response_type: ListItemsResponse<AdminPaymentDetail>
    };

    assert_eq!(status, 200);

    if let Some(first_item) = body.items.first() {
        assert!(
            !first_item.payment_id.is_empty(),
            "payment_id should not be empty"
        );
        assert!(!first_item.status.is_empty(), "status should not be empty");
        assert!(
            !first_item.currency.is_empty(),
            "currency should not be empty"
        );
        assert!(
            !first_item.order_name.is_empty(),
            "order_name should not be empty"
        );
        assert!(first_item.total >= 0, "total should be non-negative");
    }
}
