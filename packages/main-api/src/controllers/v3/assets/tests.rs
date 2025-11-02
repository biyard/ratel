use crate::tests::v3_setup::TestContextV3;
use crate::*;

#[tokio::test]
async fn test_get_put_object_uri_with_pdf() {
    let TestContextV3 {
        app, test_user, ..
    } = TestContextV3::setup().await;

    let (status, _headers, body) = get! {
        app: app,
        path: "/v3/assets?total_count=1&file_type=pdf".to_string(),
        headers: test_user.1.clone(),
        response_type: serde_json::Value
    };

    assert_eq!(status, 200, "PDF upload URL request should succeed");
    assert!(
        body["presigned_uris"].is_array(),
        "Response should contain presigned_uris array"
    );
    assert!(
        body["uris"].is_array(),
        "Response should contain uris array"
    );
    assert_eq!(
        body["presigned_uris"].as_array().unwrap().len(),
        1,
        "Should return exactly 1 presigned URL"
    );
}

#[tokio::test]
async fn test_get_put_object_uri_with_docx() {
    let TestContextV3 {
        app, test_user, ..
    } = TestContextV3::setup().await;

    let (status, _headers, body) = get! {
        app: app,
        path: "/v3/assets?total_count=1&file_type=docx".to_string(),
        headers: test_user.1.clone(),
        response_type: serde_json::Value
    };

    assert_eq!(status, 200, "DOCX upload URL request should succeed");
    assert!(
        body["presigned_uris"].is_array(),
        "Response should contain presigned_uris array"
    );
    assert_eq!(
        body["presigned_uris"].as_array().unwrap().len(),
        1,
        "Should return exactly 1 presigned URL"
    );
}

#[tokio::test]
async fn test_get_put_object_uri_with_doc() {
    let TestContextV3 {
        app, test_user, ..
    } = TestContextV3::setup().await;

    let (status, _headers, body) = get! {
        app: app,
        path: "/v3/assets?total_count=1&file_type=doc".to_string(),
        headers: test_user.1.clone(),
        response_type: serde_json::Value
    };

    assert_eq!(status, 200, "DOC upload URL request should succeed");
    assert!(
        body["presigned_uris"].is_array(),
        "Response should contain presigned_uris array"
    );
}

#[tokio::test]
async fn test_get_put_object_uri_with_xlsx() {
    let TestContextV3 {
        app, test_user, ..
    } = TestContextV3::setup().await;

    let (status, _headers, body) = get! {
        app: app,
        path: "/v3/assets?total_count=1&file_type=xlsx".to_string(),
        headers: test_user.1.clone(),
        response_type: serde_json::Value
    };

    assert_eq!(status, 200, "XLSX upload URL request should succeed");
    assert!(
        body["presigned_uris"].is_array(),
        "Response should contain presigned_uris array"
    );
}

#[tokio::test]
async fn test_get_put_object_uri_with_xls() {
    let TestContextV3 {
        app, test_user, ..
    } = TestContextV3::setup().await;

    let (status, _headers, body) = get! {
        app: app,
        path: "/v3/assets?total_count=1&file_type=xls".to_string(),
        headers: test_user.1.clone(),
        response_type: serde_json::Value
    };

    assert_eq!(status, 200, "XLS upload URL request should succeed");
    assert!(
        body["presigned_uris"].is_array(),
        "Response should contain presigned_uris array"
    );
}

#[tokio::test]
async fn test_get_put_object_uri_with_jpg() {
    let TestContextV3 {
        app, test_user, ..
    } = TestContextV3::setup().await;

    let (status, _headers, body) = get! {
        app: app,
        path: "/v3/assets?total_count=1&file_type=jpg".to_string(),
        headers: test_user.1.clone(),
        response_type: serde_json::Value
    };

    assert_eq!(status, 200, "JPG upload URL request should succeed");
    assert!(
        body["presigned_uris"].is_array(),
        "Response should contain presigned_uris array"
    );
}

#[tokio::test]
async fn test_get_put_object_uri_with_png() {
    let TestContextV3 {
        app, test_user, ..
    } = TestContextV3::setup().await;

    let (status, _headers, body) = get! {
        app: app,
        path: "/v3/assets?total_count=1&file_type=png".to_string(),
        headers: test_user.1.clone(),
        response_type: serde_json::Value
    };

    assert_eq!(status, 200, "PNG upload URL request should succeed");
    assert!(
        body["presigned_uris"].is_array(),
        "Response should contain presigned_uris array"
    );
}

#[tokio::test]
async fn test_get_put_object_uri_with_gif() {
    let TestContextV3 {
        app, test_user, ..
    } = TestContextV3::setup().await;

    let (status, _headers, body) = get! {
        app: app,
        path: "/v3/assets?total_count=1&file_type=gif".to_string(),
        headers: test_user.1.clone(),
        response_type: serde_json::Value
    };

    assert_eq!(status, 200, "GIF upload URL request should succeed");
    assert!(
        body["presigned_uris"].is_array(),
        "Response should contain presigned_uris array"
    );
}

#[tokio::test]
async fn test_get_put_object_uri_with_zip() {
    let TestContextV3 {
        app, test_user, ..
    } = TestContextV3::setup().await;

    let (status, _headers, body) = get! {
        app: app,
        path: "/v3/assets?total_count=1&file_type=zip".to_string(),
        headers: test_user.1.clone(),
        response_type: serde_json::Value
    };

    assert_eq!(status, 200, "ZIP upload URL request should succeed");
    assert!(
        body["presigned_uris"].is_array(),
        "Response should contain presigned_uris array"
    );
}

#[tokio::test]
async fn test_get_put_object_uri_with_pptx() {
    let TestContextV3 {
        app, test_user, ..
    } = TestContextV3::setup().await;

    let (status, _headers, body) = get! {
        app: app,
        path: "/v3/assets?total_count=1&file_type=pptx".to_string(),
        headers: test_user.1.clone(),
        response_type: serde_json::Value
    };

    assert_eq!(status, 200, "PPTX upload URL request should succeed");
    assert!(
        body["presigned_uris"].is_array(),
        "Response should contain presigned_uris array"
    );
}

#[tokio::test]
async fn test_get_put_object_uri_multiple_files() {
    let TestContextV3 {
        app, test_user, ..
    } = TestContextV3::setup().await;

    let (status, _headers, body) = get! {
        app: app,
        path: "/v3/assets?total_count=3&file_type=pdf".to_string(),
        headers: test_user.1.clone(),
        response_type: serde_json::Value
    };

    assert_eq!(
        status, 200,
        "Multiple PDF upload URLs request should succeed"
    );
    assert_eq!(
        body["presigned_uris"].as_array().unwrap().len(),
        3,
        "Should return exactly 3 presigned URLs"
    );
    assert_eq!(
        body["uris"].as_array().unwrap().len(),
        3,
        "Should return exactly 3 public URLs"
    );
}

#[tokio::test]
async fn test_get_put_object_uri_without_authentication() {
    let TestContextV3 { app, .. } = TestContextV3::setup().await;

    let (status, _headers, _body) = get! {
        app: app,
        path: "/v3/assets?total_count=1&file_type=pdf".to_string(),
        response_type: serde_json::Value
    };

    assert_eq!(
        status, 401,
        "Request without authentication should fail with 401"
    );
}

#[tokio::test]
async fn test_get_put_multi_object_uri_with_docx() {
    let TestContextV3 {
        app, test_user, ..
    } = TestContextV3::setup().await;

    let (status, _headers, body) = get! {
        app: app,
        path: "/v3/assets/multiparts?total_count=5&file_type=docx".to_string(),
        headers: test_user.1.clone(),
        response_type: serde_json::Value
    };

    assert_eq!(
        status, 200,
        "Multipart DOCX upload URL request should succeed"
    );
    assert!(
        body["presigned_uris"].is_array(),
        "Response should contain presigned_uris array"
    );
    assert!(
        body["upload_id"].is_string(),
        "Response should contain upload_id"
    );
    assert!(body["key"].is_string(), "Response should contain key");
    assert_eq!(
        body["presigned_uris"].as_array().unwrap().len(),
        5,
        "Should return exactly 5 presigned URLs for multipart upload"
    );
}

#[tokio::test]
async fn test_get_put_multi_object_uri_with_xlsx() {
    let TestContextV3 {
        app, test_user, ..
    } = TestContextV3::setup().await;

    let (status, _headers, body) = get! {
        app: app,
        path: "/v3/assets/multiparts?total_count=10&file_type=xlsx".to_string(),
        headers: test_user.1.clone(),
        response_type: serde_json::Value
    };

    assert_eq!(
        status, 200,
        "Multipart XLSX upload URL request should succeed"
    );
    assert!(
        body["presigned_uris"].is_array(),
        "Response should contain presigned_uris array"
    );
    assert_eq!(
        body["presigned_uris"].as_array().unwrap().len(),
        10,
        "Should return exactly 10 presigned URLs for multipart upload"
    );
}
