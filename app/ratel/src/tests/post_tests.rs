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
///
/// Round-trips a Post through DynamoDB but **overwrites** `body` with the
/// legacy `html_contents` attribute name + raw JSON string, then reads it back.
/// Exercises both `#[serde(alias = "html_contents")]` on `Post.body` and
/// `ContentBody`'s custom `Deserialize` (which accepts a bare JSON string).
#[tokio::test]
async fn legacy_html_contents_string_loads_as_html_content_body() {
    use aws_sdk_dynamodb::types::AttributeValue;
    use crate::common::DynamoEntity;
    use crate::features::posts::models::Post;
    use crate::features::posts::types::{Author, PostType};

    let ctx = TestContext::setup().await;
    let cli = &ctx.ddb;
    let table = std::env::var("DYNAMO_TABLE_PREFIX").unwrap() + "-main";

    // 1) Create a normal post via the model so all GSI fields land correctly.
    let author = Author {
        pk: ctx.test_user.0.pk.clone(),
        display_name: ctx.test_user.0.display_name.clone(),
        profile_url: ctx.test_user.0.profile_url.clone(),
        username: ctx.test_user.0.username.clone(),
        user_type: ctx.test_user.0.user_type.clone(),
    };
    let post = Post::new("Legacy title", "irrelevant initial body", PostType::Post, author);
    post.create(cli).await.unwrap();

    // 2) Surgically rewrite the row to the LEGACY schema: drop `body`, add
    //    `html_contents: String` (the old shape DynamoDB rows had before this
    //    migration). The model must still read this row as
    //    `ContentBody::HtmlContent` thanks to the alias + custom Deserialize.
    cli.update_item()
        .table_name(&table)
        .key("pk", AttributeValue::S(post.pk.to_string()))
        .key("sk", AttributeValue::S(post.sk.to_string()))
        .update_expression("REMOVE body SET html_contents = :v")
        .expression_attribute_values(":v", AttributeValue::S("<p>legacy body</p>".into()))
        .send()
        .await
        .unwrap();

    // 3) Read via the model.
    let loaded = Post::get(cli, &post.pk, Some(post.sk.clone()))
        .await
        .unwrap()
        .expect("post should be present");
    assert_eq!(
        loaded.body,
        ContentBody::HtmlContent("<p>legacy body</p>".into()),
    );
}
