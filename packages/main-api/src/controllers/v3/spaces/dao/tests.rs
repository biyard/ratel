use crate::controllers::v3::spaces::tests::setup_space;
use crate::features::spaces::SpaceDao;
use crate::types::*;
use crate::tests::v3_setup::TestContextV3;
use crate::*;

#[tokio::test]
async fn test_create_space_dao() {
    let (ctx, space_pk) = setup_space(SpaceType::Poll).await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let (status, _headers, body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/dao", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "contract_address": "0x0000000000000000000000000000000000000001",
            "deploy_block": 100
        },
        response_type: SpaceDao
    };

    assert_eq!(status, 200);
    assert_eq!(
        body.contract_address,
        "0x0000000000000000000000000000000000000001"
    );
    assert_eq!(body.deploy_block, 100);
}

#[tokio::test]
async fn test_get_space_dao() {
    let (ctx, space_pk) = setup_space(SpaceType::Poll).await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let (_status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/dao", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "contract_address": "0x0000000000000000000000000000000000000002",
            "deploy_block": 0
        },
        response_type: SpaceDao
    };

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/dao", space_pk.to_string()),
        headers: test_user.1.clone(),
        response_type: SpaceDao
    };

    assert_eq!(status, 200);
    assert_eq!(
        body.contract_address,
        "0x0000000000000000000000000000000000000002"
    );
    assert_eq!(body.deploy_block, 0);
}

#[tokio::test]
async fn test_get_space_dao_not_found() {
    let (ctx, space_pk) = setup_space(SpaceType::Poll).await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let (status, _headers, _body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/dao", space_pk.to_string()),
        headers: test_user.1.clone(),
        response_type: serde_json::Value
    };

    assert_eq!(status, 404);
}
