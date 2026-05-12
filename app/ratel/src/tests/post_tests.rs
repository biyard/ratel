use super::*;
use crate::common::types::ContentBody;

// #[tokio::test]
// async fn test_create_post_by_user() {
//     let TestContext { app, test_user, .. } = TestContext::setup().await;

//     let (status, _headers, body) = crate::test_post! {
//         app: app,
//         path: "/api/posts",
//         headers: test_user.1.clone(),
//     };

//     assert_eq!(status, 200, "create post response: {:?}", body);
// }

#[tokio::test]
async fn test_create_post_without_auth() {
    let TestContext { app, .. } = TestContext::setup().await;

    let (status, _headers, _body) = crate::test_post! {
        app: app,
        path: "/api/posts",
    };

    assert_ne!(status, 200, "unauthenticated request should fail");
}

/// Backward-compat test: rows persisted under the legacy `html_contents: String`
/// schema must still deserialize cleanly into the new `body: ContentBody` field.
/// `#[serde(alias = "html_contents")]` on `Post.body` + `ContentBody`'s custom
/// `Deserialize` (which accepts a bare JSON string) carry the legacy shape.
#[tokio::test]
async fn legacy_html_contents_string_loads_as_html_content_body() {
    let ctx = TestContext::setup().await;
    let cli = &ctx.ddb;
    let table = std::env::var("DYNAMO_TABLE_PREFIX").unwrap() + "-main";
    let pk = format!("POST#{}", uuid::Uuid::new_v4());

    let item = serde_dynamo::to_item(serde_json::json!({
        "pk": pk,
        "sk": "POST",
        "title": "Legacy",
        "html_contents": "<p>legacy body</p>",
        "post_type": "Post",
        "status": "Draft",
        "user_pk": "USER#legacy",
        "shares": 0, "likes": 0, "comments": 0, "reports": 0,
        "created_at": 0, "updated_at": 0,
        "author_display_name": "x",
        "author_profile_url": "x",
        "author_username": "x",
        "author_type": "Individual",
        "urls": [],
        "categories": [],
    }))
    .unwrap();
    cli.put_item()
        .table_name(&table)
        .set_item(Some(item))
        .send()
        .await
        .unwrap();

    let res = cli
        .get_item()
        .table_name(&table)
        .key("pk", aws_sdk_dynamodb::types::AttributeValue::S(pk.clone()))
        .key("sk", aws_sdk_dynamodb::types::AttributeValue::S("POST".into()))
        .send()
        .await
        .unwrap();
    let post: crate::features::posts::models::Post =
        serde_dynamo::from_item(res.item.unwrap()).unwrap();
    assert_eq!(
        post.body,
        ContentBody::HtmlContent("<p>legacy body</p>".into())
    );
}
