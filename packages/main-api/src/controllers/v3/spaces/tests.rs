use crate::controllers::v3::posts::CreatePostResponse;
use crate::models::SpaceCommon;
use crate::types::{ListItemsResponse, SpacePublishState, SpaceType};
use crate::*;
use crate::{
    controllers::v3::spaces::create_space::CreateSpaceResponse, tests::v3_setup::TestContextV3,
};

#[tokio::test]
pub async fn test_create_space() {
    let (ctx, post_pk) = setup_post().await;

    let TestContextV3 {
        app,
        test_user: (_user, headers),
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
    let mut last_space_pk = String::new();

    for _ in 0..11 {
        let (ctx, post_pk) = setup_post().await;

        let TestContextV3 {
            app,
            test_user: (_user, headers),
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

        assert_eq!(status, 200);

        let (status, _, _res) = patch! {
            app: app,
            path: format!("/v3/spaces/{}", res.space_pk.to_string()),
            headers: headers.clone(),
            body: {
                "publish": true,
                "visibility": "PUBLIC",
            }
        };
        tracing::info!("Create space response: {:?}", res);
        assert_eq!(status, 200, "error: {:?}", _res);

        last_space_pk = res.space_pk.to_string();
    }

    let (ctx, post_pk) = setup_post().await;

    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = ctx;

    let (status, _, _res) = post! {
        app: app,
        path: "/v3/spaces",
        headers: headers.clone(),
        body: {
            "space_type": 2,
            "post_pk": post_pk,
        },
        response_type: CreateSpaceResponse
    };

    assert_eq!(status, 200);

    let (ctx, post_pk) = setup_post().await;
    let TestContextV3 {
        app,
        test_user: (_user, headers),
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

    assert_eq!(status, 200);

    let (status, _, _res) = patch! {
        app: app,
        path: format!("/v3/spaces/{}", res.space_pk.to_string()),
        headers: headers.clone(),
        body: {
            "publish": true,
            "visibility": "PRIVATE",
        }
    };
    tracing::debug!("Create space response: {:?}", res);
    assert_eq!(status, 200, "error: {:?}", _res);

    let (status, _, list_res) = get! {
        app: app,
        path: "/v3/spaces",
        response_type: ListItemsResponse<SpaceCommon>,
    };

    assert_eq!(status, 200);
    assert_eq!(list_res.items.len(), 10);
    assert!(list_res.bookmark.is_some());
    assert!(
        list_res
            .items
            .iter()
            .all(|item| item.publish_state == SpacePublishState::Published
                && item.visibility == crate::types::SpaceVisibility::Public)
    );
    assert_eq!(
        list_res.items.first().unwrap().pk.to_string(),
        last_space_pk
    )
}

#[tokio::test]
pub async fn test_start_space() {
    let (ctx, post_pk) = setup_post().await;

    let TestContextV3 {
        app,
        test_user: (_user, headers),
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
    let (status, _, _res) = patch! {
        app: app,
        path: format!("/v3/spaces/{}", space_pk.to_string()),
        headers: headers.clone(),
        body: {
            "publish": true,
            "visibility": "PRIVATE",
        }
    };

    assert_eq!(status, 200);

    let (status, _, _res) = patch! {
        app: app,
        path: format!("/v3/spaces/{}", space_pk.to_string()),
        headers: headers.clone(),
        body: {
            "start": true,
        }
    };

    assert_eq!(status, 200);
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
    assert_eq!(_body["user_pk"], test_user.0.pk.to_string());

    return (ctx, post_pk);
}

pub async fn setup_space(space_type: SpaceType) -> (TestContextV3, String) {
    let (ctx, post_pk) = setup_post().await;
    let TestContextV3 { app, test_user, .. } = ctx.clone();

    let (_status, _headers, create_body) = post! {
        app: app,
        path: "/v3/spaces",
        headers: test_user.1.clone(),
        body: {
            "space_type": space_type as u8,
            "post_pk": post_pk,
        },
        response_type: CreateSpaceResponse
    };

    let space_pk = create_body.space_pk.to_string();

    return (ctx, space_pk);
}
