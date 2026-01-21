use crate::controllers::v3::posts::create_post::CreatePostResponse;
use crate::controllers::v3::spaces::CreateSpaceResponse;

use crate::features::spaces::recommendations::SpaceRecommendationResponse;
use crate::tests::{
    create_app_state,
    v3_setup::{TestContextV3, setup_v3},
};
use crate::types::File;
use crate::types::{FileExtension, Partition, SpaceType};
use crate::*;
use axum::AxumRouter;

struct CreatedDeliberationSpace {
    space_pk: Partition,
}

#[tokio::test]
async fn test_update_recommendation_contents() {
    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = setup_v3().await;

    let app_state = create_app_state();
    let _cli = &app_state.dynamo.client;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/{}/recommendations", space_pk_encoded);

    let (status, _headers, body) = patch! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "Content": {
                "html_contents": "recommendation html contents"
            }
        },
        response_type: SpaceRecommendationResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.html_contents, "recommendation html contents");
}

#[tokio::test]
async fn test_update_recommendation_files() {
    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = setup_v3().await;

    let app_state = create_app_state();
    let _cli = &app_state.dynamo.client;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/{}/recommendations", space_pk_encoded);

    let (status, _headers, body) = patch! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "File": {
                "files": vec![File{
                    id: uuid::Uuid::new_v4().to_string(),
                    name: "recommendation file name".to_string(),
                    size: "15KB".to_string(),
                    ext: FileExtension::PDF,
                    url: None
                }]
            }
        },
        response_type: SpaceRecommendationResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.files.len(), 1);
}

#[tokio::test]
async fn test_get_recommendation_handler() {
    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = setup_v3().await;

    let app_state = create_app_state();
    let _cli = &app_state.dynamo.client;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/{}/recommendations", space_pk_encoded);

    let (status, _headers, _body) = patch! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "Content": {
                "html_contents": "recommendation html contents"
            }
        },
        response_type: SpaceRecommendationResponse
    };

    assert_eq!(status, 200);

    let (status, _headers, body) = get! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        response_type: SpaceRecommendationResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.html_contents, "recommendation html contents");
}

async fn bootstrap_deliberation_space(
    app: &AxumRouter,
    headers: axum::http::HeaderMap,
) -> CreatedDeliberationSpace {
    let (_status, _headers, post) = post! {
        app: app,
        path: "/v3/posts",
        headers: headers.clone(),
        response_type: CreatePostResponse
    };

    let feed_pk = post.post_pk.clone();

    let (_status, _headers, space) = post! {
        app: app,
        path: "/v3/spaces",
        headers: headers.clone(),
        body: {
            "space_type": SpaceType::Deliberation,
            "post_pk": feed_pk
        },
        response_type: CreateSpaceResponse
    };

    CreatedDeliberationSpace {
        space_pk: space.space_pk,
    }
}
