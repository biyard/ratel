use crate::controllers::v3::spaces::CreateSpaceResponse;
use crate::features::spaces::panels::{ListPanelResponse, SpacePanelResponse};
use crate::types::{Attribute, Partition, SpaceType};
use crate::*;
use crate::{
    controllers::v3::posts::CreatePostResponse,
    tests::v3_setup::{TestContextV3, setup_v3},
};
use axum::AxumRouter;

struct CreatedDeliberationSpace {
    space_pk: Partition,
}

#[tokio::test]
async fn test_create_panel_handler() {
    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = setup_v3().await;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/{}/panels", space_pk_encoded);

    let (status, _headers, body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "name": "Panel 1".to_string(), "quotas": 10, "attributes": vec![Attribute::Age(types::Age::Range { inclusive_min: 0, inclusive_max: 19 }), Attribute::Gender(types::Gender::Female)],
        },
        response_type: SpacePanelResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.attributes.len(), 2);
}

#[tokio::test]
async fn test_update_panel_handler() {
    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = setup_v3().await;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/{}/panels", space_pk_encoded);

    let (status, _headers, body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "name": "Panel 1".to_string(), "quotas": 10, "attributes": vec![Attribute::Age(types::Age::Range { inclusive_min: 0, inclusive_max: 19 }), Attribute::Gender(types::Gender::Female)],
        },
        response_type: SpacePanelResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.attributes.len(), 2);

    let panel_pk = body.pk;
    let panel_pk_encoded = panel_pk.to_string().replace('#', "%23");

    let path = format!(
        "/v3/spaces/{}/panels/{}",
        space_pk_encoded, panel_pk_encoded
    );

    let (status, _headers, body) = patch! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "name": "Panel 1".to_string(), "quotas": 10, "attributes": vec![Attribute::Age(types::Age::Range { inclusive_min: 0, inclusive_max: 19 })],
        },
        response_type: SpacePanelResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.attributes.len(), 1);
}

#[tokio::test]
async fn test_delete_panel_handler() {
    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = setup_v3().await;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/{}/panels", space_pk_encoded);

    let (status, _headers, body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "name": "Panel 1".to_string(), "quotas": 10, "attributes": vec![Attribute::Age(types::Age::Range { inclusive_min: 0, inclusive_max: 19 }), Attribute::Gender(types::Gender::Female)],
        },
        response_type: SpacePanelResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.attributes.len(), 2);

    let panel_pk = body.pk;
    let panel_pk_encoded = panel_pk.to_string().replace('#', "%23");

    let path = format!(
        "/v3/spaces/{}/panels/{}",
        space_pk_encoded, panel_pk_encoded
    );

    let (status, _headers, _body) = delete! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {},
        response_type: Partition
    };

    assert_eq!(status, 200);
}

#[tokio::test]
async fn test_list_panels_handler() {
    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = setup_v3().await;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/{}/panels", space_pk_encoded);

    let (status, _headers, _body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "name": "Panel 1".to_string(), "quotas": 10, "attributes": vec![Attribute::Age(types::Age::Range { inclusive_min: 0, inclusive_max: 19 }), Attribute::Gender(types::Gender::Female)],
        },
        response_type: SpacePanelResponse
    };

    assert_eq!(status, 200);

    let (status, _headers, _body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "name": "Panel 2".to_string(), "quotas": 10, "attributes": vec![Attribute::Age(types::Age::Range { inclusive_min: 0, inclusive_max: 19 }), Attribute::Gender(types::Gender::Female)],
        },
        response_type: SpacePanelResponse
    };

    assert_eq!(status, 200);

    let (status, _headers, body) = get! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        response_type: ListPanelResponse
    };

    tracing::debug!("list panels body: {:?}", body);

    assert_eq!(status, 200);
}

#[tokio::test]
async fn test_get_panel_handler() {
    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = setup_v3().await;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/{}/panels", space_pk_encoded);

    let (status, _headers, body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "name": "Panel 1".to_string(), "quotas": 10, "attributes": vec![Attribute::Age(types::Age::Range { inclusive_min: 0, inclusive_max: 19 }), Attribute::Gender(types::Gender::Female)],
        },
        response_type: SpacePanelResponse
    };

    assert_eq!(status, 200);
    tracing::debug!("panel body: {:?}", body);

    let panel_pk = body.pk;
    let panel_pk_encoded = panel_pk.to_string().replace('#', "%23");
    let path = format!(
        "/v3/spaces/{}/panels/{}",
        space_pk_encoded, panel_pk_encoded
    );

    let (status, _headers, body) = get! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        response_type: SpacePanelResponse
    };

    tracing::debug!("get panel body: {:?}", body);

    assert_eq!(status, 200);
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
