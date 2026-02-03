use crate::controllers::v3::spaces::tests::setup_space;
use crate::features::spaces::{SpaceDao, SpaceDaoSelectedUser, SpaceParticipant};
use crate::models::user::UserEvmAddress;
use crate::types::*;
use crate::tests::v3_setup::TestContextV3;
use crate::*;
use std::str::FromStr;

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

#[tokio::test]
async fn test_create_and_list_space_dao_selected() {
    let (ctx, space_pk) = setup_space(SpaceType::Poll).await;
    let user2 = ctx.create_another_user().await;
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = ctx;

    let (_status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/dao", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "contract_address": "0x0000000000000000000000000000000000000001",
            "deploy_block": 100
        },
        response_type: SpaceDao
    };

    let space_partition = Partition::from_str(&space_pk).unwrap();

    let participant1 =
        SpaceParticipant::new_non_anonymous(space_partition.clone(), test_user.0.clone());
    let (pk1, sk1) = SpaceParticipant::keys(space_partition.clone(), test_user.0.pk.clone());
    if SpaceParticipant::get(&ddb, pk1, Some(sk1))
        .await
        .unwrap()
        .is_none()
    {
        participant1.create(&ddb).await.unwrap();
    }

    let participant2 =
        SpaceParticipant::new_non_anonymous(space_partition.clone(), user2.0.clone());
    let (pk2, sk2) = SpaceParticipant::keys(space_partition.clone(), user2.0.pk.clone());
    if SpaceParticipant::get(&ddb, pk2, Some(sk2))
        .await
        .unwrap()
        .is_none()
    {
        participant2.create(&ddb).await.unwrap();
    }

    let evm1 = "0x0000000000000000000000000000000000000001".to_string();
    let evm2 = "0x0000000000000000000000000000000000000002".to_string();
    let evm_item1 = UserEvmAddress::new(test_user.0.pk.clone(), evm1.clone());
    evm_item1.create(&ddb).await.unwrap();
    let evm_item2 = UserEvmAddress::new(user2.0.pk.clone(), evm2.clone());
    evm_item2.create(&ddb).await.unwrap();

    let (status, _headers, body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/dao/selected", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "selected_addresses": [evm1.clone(), evm2.clone()]
        },
        response_type: Vec<SpaceDaoSelectedUser>
    };

    assert_eq!(status, 200);
    assert_eq!(body.len(), 2);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/dao/selected?limit=50", space_pk.to_string()),
        headers: test_user.1.clone(),
        response_type: serde_json::Value
    };

    assert_eq!(status, 200);
    assert_eq!(body["items"].as_array().unwrap().len(), 2);
    assert_eq!(body["remaining_count"].as_i64().unwrap(), 2);
    assert_eq!(body["total_count"].as_i64().unwrap(), 2);
}

#[tokio::test]
async fn test_update_space_dao_selected() {
    let (ctx, space_pk) = setup_space(SpaceType::Poll).await;
    let user2 = ctx.create_another_user().await;
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = ctx;

    let (_status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/dao", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "contract_address": "0x0000000000000000000000000000000000000002",
            "deploy_block": 100
        },
        response_type: SpaceDao
    };

    let space_partition = Partition::from_str(&space_pk).unwrap();

    let participant1 =
        SpaceParticipant::new_non_anonymous(space_partition.clone(), test_user.0.clone());
    let (pk1, sk1) = SpaceParticipant::keys(space_partition.clone(), test_user.0.pk.clone());
    if SpaceParticipant::get(&ddb, pk1, Some(sk1))
        .await
        .unwrap()
        .is_none()
    {
        participant1.create(&ddb).await.unwrap();
    }

    let participant2 =
        SpaceParticipant::new_non_anonymous(space_partition.clone(), user2.0.clone());
    let (pk2, sk2) = SpaceParticipant::keys(space_partition.clone(), user2.0.pk.clone());
    if SpaceParticipant::get(&ddb, pk2, Some(sk2))
        .await
        .unwrap()
        .is_none()
    {
        participant2.create(&ddb).await.unwrap();
    }

    let evm1 = "0x0000000000000000000000000000000000000003".to_string();
    let evm2 = "0x0000000000000000000000000000000000000004".to_string();
    let evm_item1 = UserEvmAddress::new(test_user.0.pk.clone(), evm1.clone());
    evm_item1.create(&ddb).await.unwrap();
    let evm_item2 = UserEvmAddress::new(user2.0.pk.clone(), evm2.clone());
    evm_item2.create(&ddb).await.unwrap();

    let (_status, _headers, body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/dao/selected", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "selected_addresses": [evm1.clone(), evm2.clone()]
        },
        response_type: Vec<SpaceDaoSelectedUser>
    };

    let selected_sks: Vec<String> = body.iter().map(|item| item.sk.to_string()).collect();

    let (status, _headers, updated) = patch! {
        app: app,
        path: format!("/v3/spaces/{}/dao/selected", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "selected_sks": selected_sks,
            "reward_distributed": true
        },
        response_type: Vec<SpaceDaoSelectedUser>
    };

    assert_eq!(status, 200);
    assert!(updated.iter().all(|item| item.reward_distributed));

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/dao/selected?limit=50", space_pk.to_string()),
        headers: test_user.1.clone(),
        response_type: serde_json::Value
    };

    assert_eq!(status, 200);
    assert_eq!(body["remaining_count"].as_i64().unwrap(), 0);
    assert_eq!(body["total_count"].as_i64().unwrap(), 2);
}
