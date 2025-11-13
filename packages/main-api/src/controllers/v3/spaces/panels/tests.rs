use crate::controllers::v3::spaces::CreateSpaceResponse;
use crate::controllers::v3::spaces::panels::{CreatePanelQuotaResponse, UpdatePanelQuotaResponse};
use crate::features::spaces::panels::{
    ListParticipantResponse, PanelAttribute, SpacePanelParticipantResponse, SpacePanelsResponse,
};
use crate::features::did::VerifiableAttribute;
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
async fn test_create_panel_quota_handler() {
    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = setup_v3().await;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/{}/panels/quotas", space_pk_encoded);

    let (status, _headers, body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "quotas": vec![30], "attributes": vec![PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(Gender::Male))]
        },
        response_type: CreatePanelQuotaResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.attributes.len(), 1);
}

#[tokio::test]
async fn test_delete_panel_quota_handler() {
    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = setup_v3().await;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/{}/panels/quotas", space_pk_encoded);

    let (status, _headers, body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "quotas": vec![30, 30], "attributes": vec![PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(Gender::Male)), PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(Age::Range { inclusive_min: 0, inclusive_max: 18 }))]
        },
        response_type: CreatePanelQuotaResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.attributes.len(), 2);

    let (status, _headers, _body) = delete! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "attribute": PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(Age::Range { inclusive_min: 0, inclusive_max: 18 }))
        },
        response_type: Partition
    };

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

    let (status, _headers, body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/panels/quotas", space_pk.to_string()),
        headers: headers.clone(),
        body: {
            "quotas": vec![30, 30], "attributes": vec![PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(Gender::Male)), PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(Age::Range { inclusive_min: 0, inclusive_max: 18 }))]
        },
        response_type: CreatePanelQuotaResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.attributes.len(), 2);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/panels", space_pk.to_string()),
        headers: headers.clone(),
        response_type: SpacePanelsResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.panel_quotas.len(), 2);
}

#[tokio::test]
async fn test_update_panel_quota_handler() {
    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = setup_v3().await;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let (status, _headers, body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/panels/quotas", space_pk.to_string()),
        headers: headers.clone(),
        body: {
            "quotas": vec![30], "attributes": vec![PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(Gender::Male))]
        },
        response_type: CreatePanelQuotaResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.attributes.len(), 1);

    let (status, _headers, _body) = patch! {
        app: app,
        path: format!("/v3/spaces/{}/panels/quotas", space_pk.to_string()),
        headers: headers.clone(),
        body: {
            "quotas": 50, "attribute": PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(Gender::Male))
        },
        response_type: UpdatePanelQuotaResponse
    };

    assert_eq!(status, 200);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/panels", space_pk.to_string()),
        headers: headers.clone(),
        response_type: SpacePanelsResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.panel_quotas.len(), 1);
    assert_eq!(body.panel_quotas[0].quotas, 50);
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

    let (status, _headers, body) = patch! {
        app: app,
        path: format!("/v3/spaces/{}/panels", space_pk.to_string()),
        headers: headers.clone(),
        body: {
            "quotas": 50, "attributes": vec![PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(Gender::Male))]
        },
        response_type: SpacePanelsResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.quotas, 50);
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
