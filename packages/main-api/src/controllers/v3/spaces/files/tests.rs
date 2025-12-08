use crate::controllers::v3::posts::create_post::CreatePostResponse;
use crate::controllers::v3::spaces::CreateSpaceResponse;
use crate::controllers::v3::spaces::files::get_files::GetSpaceFileResponse;
use crate::controllers::v3::spaces::files::update_files::UpdateSpaceFileResponse;

use crate::tests::{
    create_app_state,
    v3_setup::{TestContextV3, setup_v3},
};
use crate::types::{file_location::FileLocation, File, Partition, SpaceType};
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
                id: None,
                name: "deliberation overview file title".to_string(),
                size: "15KB".to_string(),
                ext: crate::types::FileExtension::PDF,
                url: None,
                locations: vec![FileLocation::Files],
                description: None,
                uploaded_at: None,
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
                id: None,
                name: "updated deliberation overview file title".to_string(),
                size: "15KB".to_string(),
                ext: crate::types::FileExtension::PDF,
                url: None,
                locations: vec![FileLocation::Files],
                description: None,
                uploaded_at: None,
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

#[tokio::test]
async fn test_files_with_location_filtering() {
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

    // Upload files with different locations
    let (status, _headers, _body) = patch! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "files": vec![
                File {
                    id: None,
                    name: "overview_file.pdf".to_string(),
                    size: "10KB".to_string(),
                    ext: crate::types::FileExtension::PDF,
                    url: Some("https://example.com/overview.pdf".to_string()),
                    locations: vec![FileLocation::Overview],
                    description: Some("Overview document".to_string()),
                    uploaded_at: None,
                },
                File {
                    id: None,
                    name: "board_image.png".to_string(),
                    size: "500KB".to_string(),
                    ext: crate::types::FileExtension::PNG,
                    url: Some("https://example.com/board.png".to_string()),
                    locations: vec![FileLocation::Board],
                    description: Some("Board image".to_string()),
                    uploaded_at: None,
                },
                File {
                    id: None,
                    name: "shared_file.docx".to_string(),
                    size: "25KB".to_string(),
                    ext: crate::types::FileExtension::WORD,
                    url: Some("https://example.com/shared.docx".to_string()),
                    locations: vec![FileLocation::Overview, FileLocation::Board, FileLocation::Files],
                    description: Some("Shared across all tabs".to_string()),
                    uploaded_at: None,
                },
                File {
                    id: None,
                    name: "files_only.zip".to_string(),
                    size: "1MB".to_string(),
                    ext: crate::types::FileExtension::ZIP,
                    url: Some("https://example.com/archive.zip".to_string()),
                    locations: vec![FileLocation::Files],
                    description: None,
                    uploaded_at: None,
                },
            ],
        },
        response_type: UpdateSpaceFileResponse
    };

    assert_eq!(status, 200);

    // Get all files - should return 4
    let (status, _headers, body) = get! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        response_type: GetSpaceFileResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.files.len(), 4);
    
    // Verify all files have IDs and timestamps
    for file in &body.files {
        assert!(file.id.is_some(), "File should have an ID");
        assert!(file.uploaded_at.is_some(), "File should have upload timestamp");
    }

    // Filter by Overview location - should return 2 files (overview_file and shared_file)
    let (status, _headers, body) = get! {
        app: app,
        path: format!("{}?location=overview", path),
        headers: headers.clone(),
        response_type: GetSpaceFileResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.files.len(), 2);
    assert!(body.files.iter().any(|f| f.name == "overview_file.pdf"));
    assert!(body.files.iter().any(|f| f.name == "shared_file.docx"));

    // Filter by Board location - should return 2 files (board_image and shared_file)
    let (status, _headers, body) = get! {
        app: app,
        path: format!("{}?location=board", path),
        headers: headers.clone(),
        response_type: GetSpaceFileResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.files.len(), 2);
    assert!(body.files.iter().any(|f| f.name == "board_image.png"));
    assert!(body.files.iter().any(|f| f.name == "shared_file.docx"));

    // Filter by Files location - should return 2 files (shared_file and files_only)
    let (status, _headers, body) = get! {
        app: app,
        path: format!("{}?location=files", path),
        headers: headers.clone(),
        response_type: GetSpaceFileResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.files.len(), 2);
    assert!(body.files.iter().any(|f| f.name == "shared_file.docx"));
    assert!(body.files.iter().any(|f| f.name == "files_only.zip"));
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
