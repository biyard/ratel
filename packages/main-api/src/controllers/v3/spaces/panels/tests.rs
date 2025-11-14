use crate::controllers::v3::spaces::CreateSpaceResponse;
use crate::features::spaces::panels::{
    ListParticipantResponse, PanelAttribute, PanelAttributeWithQuota, SpacePanelParticipantResponse,
    SpacePanelQuota, SpacePanelsResponse,
};
use crate::features::did::{VerifiableAttribute, VerifiableAttributeWithQuota};
use crate::types::{Age, Attribute, Gender, Partition, SpaceType};
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

    let (status, _headers, body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/panels", space_pk.to_string()),
        headers: headers.clone(),
        body: {
            "attributes": vec![
                PanelAttributeWithQuota::VerifiableAttribute(
                    VerifiableAttributeWithQuota {
                        attribute: VerifiableAttribute::Gender(Gender::Male),
                        quota: 30
                    }
                )
            ]
        },
        response_type: Vec<SpacePanelQuota>
    };

    assert_eq!(status, 200);
    assert_eq!(body.len(), 1);
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

    let (status, _headers, body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/panels", space_pk.to_string()),
        headers: headers.clone(),
        body: {
            "attributes": vec![
                PanelAttributeWithQuota::VerifiableAttribute(
                    VerifiableAttributeWithQuota {
                        attribute: VerifiableAttribute::Gender(Gender::Male),
                        quota: 30
                    }
                ),
                PanelAttributeWithQuota::VerifiableAttribute(
                    VerifiableAttributeWithQuota {
                        attribute: VerifiableAttribute::Age(Age::Range { inclusive_min: 0, inclusive_max: 18 }),
                        quota: 30
                    }
                )
            ]
        },
        response_type: Vec<SpacePanelQuota>
    };

    assert_eq!(status, 200);
    assert_eq!(body.len(), 2);

    // Find the panel with Age attribute to delete
    let age_panel = body.iter().find(|p| {
        matches!(p.attributes, PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(_)))
    }).expect("Age panel should exist");

    #[derive(Serialize)]
    struct DeleteKey {
        pk: CompositePartition,
        sk: EntityType,
    }

    let (status, _headers, _body) = delete! {
        app: app,
        path: format!("/v3/spaces/{}/panels", space_pk.to_string()),
        headers: headers.clone(),
        body: {
            "keys": vec![DeleteKey { pk: age_panel.pk.clone(), sk: age_panel.sk.clone() }]
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
        path: format!("/v3/spaces/{}/panels", space_pk.to_string()),
        headers: headers.clone(),
        body: {
            "attributes": vec![
                PanelAttributeWithQuota::VerifiableAttribute(
                    VerifiableAttributeWithQuota {
                        attribute: VerifiableAttribute::Gender(Gender::Male),
                        quota: 30
                    }
                ),
                PanelAttributeWithQuota::VerifiableAttribute(
                    VerifiableAttributeWithQuota {
                        attribute: VerifiableAttribute::Age(Age::Range { inclusive_min: 0, inclusive_max: 18 }),
                        quota: 30
                    }
                )
            ]
        },
        response_type: Vec<SpacePanelQuota>
    };

    assert_eq!(status, 200);
    assert_eq!(body.len(), 2);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/panels", space_pk.to_string()),
        headers: headers.clone(),
        response_type: ListItemsResponse<SpacePanelQuota>
    };

    assert_eq!(status, 200);
    assert_eq!(body.items.len(), 2);
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
        path: format!("/v3/spaces/{}/panels", space_pk.to_string()),
        headers: headers.clone(),
        body: {
            "attributes": vec![
                PanelAttributeWithQuota::VerifiableAttribute(
                    VerifiableAttributeWithQuota {
                        attribute: VerifiableAttribute::Gender(Gender::Male),
                        quota: 30
                    }
                )
            ]
        },
        response_type: Vec<SpacePanelQuota>
    };

    assert_eq!(status, 200);
    assert_eq!(body.len(), 1);

    let panel_sk = &body[0].sk;

    let (status, _headers, updated_body) = patch! {
        app: app,
        path: format!("/v3/spaces/{}/panels/{}", space_pk.to_string(), panel_sk),
        headers: headers.clone(),
        body: {
            "quota": 50
        },
        response_type: SpacePanelQuota
    };

    assert_eq!(status, 200);
    assert_eq!(updated_body.quotas, 50);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/panels", space_pk.to_string()),
        headers: headers.clone(),
        response_type: ListItemsResponse<SpacePanelQuota>
    };

    assert_eq!(status, 200);
    assert_eq!(body.items.len(), 1);
    assert_eq!(body.items[0].quotas, 50);
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
