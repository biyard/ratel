use crate::controllers::v3::posts::create_post::CreatePostResponse;
use crate::controllers::v3::spaces::CreateSpaceResponse;
use crate::controllers::v3::spaces::files::get_files::GetSpaceFileResponse;
use crate::controllers::v3::spaces::files::update_files::UpdateSpaceFileResponse;
use crate::features::spaces::files::{GetFilesByTargetResponse, ListFileLinksResponse};

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

#[tokio::test]
async fn test_update_files_with_link_targets() {
    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = setup_v3().await;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");

    // Update space files (this will automatically link them to Files and Overview)
    let (status, _headers, _body) = patch! {
        app: app,
        path: format!("/v3/spaces/{}", space_pk_encoded),
        headers: headers.clone(),
        body: {
            "files": vec![File {
                name: "linked-file.pdf".to_string(),
                size: "20KB".to_string(),
                ext: crate::types::FileExtension::PDF,
                url: Some("https://example.com/linked-file.pdf".to_string()),
            }]
        },
        response_type: crate::controllers::v3::spaces::SpaceCommonResponse
    };

    assert_eq!(status, 200);

    // Verify file links were created automatically
    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/files/links", space_pk_encoded),
        headers: headers.clone(),
        response_type: ListFileLinksResponse
    };

    assert_eq!(status, 200);
    assert!(!body.file_links.is_empty());
}

#[tokio::test]
async fn test_get_files_by_target() {
    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = setup_v3().await;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let test_file_url = "https://example.com/overview-file.pdf";

    // Update space with file (automatically links to Overview and Files)
    let (_status, _headers, _body) = patch! {
        app: app,
        path: format!("/v3/spaces/{}", space_pk_encoded),
        headers: headers.clone(),
        body: {
            "files": vec![File {
                name: "overview-file.pdf".to_string(),
                size: "20KB".to_string(),
                ext: crate::types::FileExtension::PDF,
                url: Some(test_file_url.to_string()),
            }]
        },
        response_type: crate::controllers::v3::spaces::SpaceCommonResponse
    };

    // Query files by Overview target
    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/files/links/target?target=Overview", space_pk_encoded),
        headers: headers.clone(),
        response_type: GetFilesByTargetResponse
    };

    assert_eq!(status, 200);
    assert!(body.file_urls.contains(&test_file_url.to_string()));
}

#[tokio::test]
async fn test_list_all_file_links() {
    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = setup_v3().await;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");

    // Update space with files (automatically links them)
    let (_status, _headers, _body) = patch! {
        app: app,
        path: format!("/v3/spaces/{}", space_pk_encoded),
        headers: headers.clone(),
        body: {
            "files": vec![
                File {
                    name: "file1.pdf".to_string(),
                    size: "10KB".to_string(),
                    ext: crate::types::FileExtension::PDF,
                    url: Some("https://example.com/file1.pdf".to_string()),
                },
                File {
                    name: "file2.pdf".to_string(),
                    size: "15KB".to_string(),
                    ext: crate::types::FileExtension::PDF,
                    url: Some("https://example.com/file2.pdf".to_string()),
                }
            ]
        },
        response_type: crate::controllers::v3::spaces::SpaceCommonResponse
    };

    // List all file links
    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/files/links", space_pk_encoded),
        headers: headers.clone(),
        response_type: ListFileLinksResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.file_links.len(), 2);
}

#[tokio::test]
async fn test_unlink_all_targets_deletes_file_link() {
    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = setup_v3().await;

    let app_state = create_app_state();
    let cli = &app_state.dynamo.client;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let test_file_url = "https://example.com/delete-test.pdf";

    // Add file via space update (automatically creates link)
    let (_status, _headers, _body) = patch! {
        app: app,
        path: format!("/v3/spaces/{}", space_pk_encoded),
        headers: headers.clone(),
        body: {
            "files": vec![File {
                name: "delete-test.pdf".to_string(),
                size: "10KB".to_string(),
                ext: crate::types::FileExtension::PDF,
                url: Some(test_file_url.to_string()),
            }]
        },
        response_type: crate::controllers::v3::spaces::SpaceCommonResponse
    };

    // Manually remove all link targets using the model directly
    use crate::features::spaces::files::{FileLink, FileLinkTarget};
    
    let _ = FileLink::remove_link_target(cli, &space_pk, test_file_url, &FileLinkTarget::Files)
        .await;
    let _ = FileLink::remove_link_target(cli, &space_pk, test_file_url, &FileLinkTarget::Overview)
        .await;

    // Verify file link is deleted
    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/files/links", space_pk_encoded),
        headers: headers.clone(),
        response_type: ListFileLinksResponse
    };

    assert_eq!(status, 200);
    assert!(
        body.file_links
            .iter()
            .all(|link| link.file_url != test_file_url)
    );
}
