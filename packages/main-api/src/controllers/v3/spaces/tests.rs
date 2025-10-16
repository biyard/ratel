use crate::controllers::v3::posts::CreatePostResponse;
use crate::*;
use crate::{
    controllers::v3::spaces::create_space::CreateSpaceResponse,
    models::feed::Post,
    tests::v3_setup::{TestContextV3, setup_v3},
    types::PostType,
};

#[tokio::test]
async fn test_create_space() {
    let (ctx, post_pk) = setup_post().await;

    let TestContextV3 {
        app,
        test_user: (user, headers),
        ddb,
        ..
    } = ctx;

    let (status, _, res) = post! {
        app: app,
        path: "/v3/spaces",
        headers: headers.clone(),
        body: {
            "space_type": 2,
            "post_pk": post_pk,
        },
        response_type: CreateSpaceResponse
    };
    tracing::debug!("Create space response: {:?}", res);
    assert_eq!(status, 200);

    let space_pk = res.space_pk;
    let encoded_pk = percent_encoding::percent_encode(
        space_pk.to_string().as_bytes(),
        percent_encoding::NON_ALPHANUMERIC,
    )
    .to_string();
    let path = format!("/v3/spaces/{}", encoded_pk);

    let (status, _, res) = delete! {
        app: app,
        path: path,
        headers: headers.clone()
    };
    tracing::debug!("Delete space response: {:?}", res);
    assert_eq!(status, 200);
}

#[tokio::test]
async fn test_list_spaces() {
    let (ctx, post_pk) = setup_post().await;

    let TestContextV3 {
        app,
        test_user: (user, headers),
        ddb,
        now,
        ..
    } = ctx;

    let title_base = format!("List Space Post {now}");
    let content_base = format!("This is content for List Space Post {now}");

    for i in 0..11 {
        let (ctx, post_pk) = setup_post().await;

        let TestContextV3 {
            app,
            test_user: (user, headers),
            ddb,
            ..
        } = ctx;

        let title = format!("{} {}", title_base, i);
        let content = format!("{} {}", content_base, i);

        let (status, _, res) = post! {
            app: app,
            path: "/v3/spaces",
            headers: headers.clone(),
            body: {
                "space_type": 2,
                "post_pk": post_pk,
            },
            response_type: CreateSpaceResponse
        };
        tracing::debug!("Create space response: {:?}", res);
        assert_eq!(status, 200);
    }
}

#[tokio::test]
async fn test_get_space() {}

pub async fn setup_post() -> (TestContextV3, String) {
    let ctx = TestContextV3::setup().await;
    let TestContextV3 { app, test_user, .. } = ctx.clone();

    let (_status, _headers, create_body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };

    let (_status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", create_body.post_pk.to_string()),
        headers: test_user.1.clone()
    };

    let post_pk = body["post"]["pk"].as_str().unwrap_or_default().to_string();

    let (_status, _headers, _body) = patch! {
        app: app,
        path: format!("/v3/posts/{}", post_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "title": "Post for Space",
            "content": "<p>post for space contents</p>",
            "publish": true
        }
    };

    return (ctx, post_pk);
}
