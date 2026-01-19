use crate::controllers::v3::posts::create_post::CreatePostResponse;
use crate::controllers::v3::spaces::CreateSpaceResponse;
use crate::controllers::v3::spaces::files::get_files::GetSpaceFileResponse;
use crate::controllers::v3::spaces::files::update_files::UpdateSpaceFileResponse;

use crate::tests::{
    create_app_state,
    v3_setup::{TestContextV3, setup_v3},
};
use crate::types::{File, Partition, SpaceType};
use crate::*;
use axum::AxumRouter;

struct CreatedDeliberationSpace {
    space_pk: Partition,
}

#[tokio::test]
async fn test_update_files_handler() {
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
    let path = format!("/v3/spaces/{}/files", space_pk_encoded);

    let (status, _headers, _body) = patch! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "files": vec![File {
                id: uuid::Uuid::new_v4().to_string(),
                name: "deliberation overview file title".to_string(),
                size: "15KB".to_string(),
                ext: crate::types::FileExtension::PDF,
                url: None,
            }],
        },
        response_type: UpdateSpaceFileResponse
    };

    assert_eq!(status, 200);

    let (status, _headers, body) = get! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        response_type: GetSpaceFileResponse
    };

    assert_eq!(status, 200);

    assert_eq!(body.files.len(), 1);
    assert_eq!(body.files[0].name, "deliberation overview file title");

    let (status, _headers, _body) = patch! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "files": vec![File {
                id: uuid::Uuid::new_v4().to_string(),
                name: "updated deliberation overview file title".to_string(),
                size: "15KB".to_string(),
                ext: crate::types::FileExtension::PDF,
                url: None,
            }],
        },
        response_type: UpdateSpaceFileResponse
    };

    assert_eq!(status, 200);

    let (status, _headers, body) = get! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        response_type: GetSpaceFileResponse
    };

    assert_eq!(status, 200);

    assert_eq!(body.files.len(), 1);
    assert_eq!(
        body.files[0].name,
        "updated deliberation overview file title"
    );
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
