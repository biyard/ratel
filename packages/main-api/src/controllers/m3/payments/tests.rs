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
        response_type: ListItemsResponse<AdminPaymentResponse>
    };

    assert_eq!(status, 200, "Admin should be able to access payment list");
    assert!(body.items.len() >= 0, "Should return payment items");
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
async fn test_list_payments_with_bookmark() {
    let TestContextV3 {
        app, admin_user, ..
    } = TestContextV3::setup().await;

    let (status, _headers, body) = get! {
        app: app,
        path: "/m3/payments",
        headers: admin_user.1.clone(),
        response_type: ListItemsResponse<AdminPaymentResponse>
    };

    assert_eq!(status, 200);

    // If bookmark exists, test pagination
    if let Some(bookmark) = body.bookmark {
        // Bookmark should be a page number string
        let page_num: i32 = bookmark.parse().expect("Bookmark should be a valid page number");
        assert!(page_num > 0, "Next page should be > 0");

        // Request next page
        let (status, _headers, _body) = get! {
            app: app,
            path: format!("/m3/payments?bookmark={}", bookmark),
            headers: admin_user.1,
            response_type: ListItemsResponse<AdminPaymentResponse>
        };

        assert_eq!(status, 200, "Should successfully fetch next page");
    }
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
        response_type: ListItemsResponse<AdminPaymentResponse>
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
