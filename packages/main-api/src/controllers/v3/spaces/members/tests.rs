use crate::controllers::v3::posts::create_post::CreatePostResponse;
use crate::controllers::v3::spaces::CreateSpaceResponse;
use crate::controllers::v3::spaces::files::get_files::GetSpaceFileResponse;
use crate::controllers::v3::spaces::files::update_files::UpdateSpaceFileResponse;

use crate::controllers::v3::spaces::members::{
    ResentInvitationCodeResponse, UpsertInvitationResponse,
};
use crate::features::did::AttributeCode;
use crate::features::did::{VerifiableAttribute, VerifiableAttributeWithQuota};
use crate::features::spaces::members::{SpaceEmailVerification, SpaceInvitationMemberResponse};
use crate::features::spaces::panels::{
    PanelAttribute, PanelAttributeWithQuota, SpacePanelQuota, SpacePanelsResponse,
};
use crate::tests::create_user_session;
use crate::tests::{
    create_app_state,
    v3_setup::{TestContextV3, setup_v3},
};
use crate::types::{EntityType, File, Gender, Partition, SpaceType};
use crate::*;
use axum::AxumRouter;

struct CreatedDeliberationSpace {
    space_pk: Partition,
}

#[tokio::test]
async fn test_upsert_invitation_handler() {
    let TestContextV3 {
        ddb,
        app,
        test_user: (_user, headers),
        ..
    } = setup_v3().await;

    let app_state = create_app_state();
    let _cli = &app_state.dynamo.client;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");

    let (new_user, _headers) = create_user_session(app.clone(), &ddb).await;
    let (new_user_2, _headers) = create_user_session(app.clone(), &ddb).await;
    let (new_user_3, _headers) = create_user_session(app.clone(), &ddb).await;

    let path = format!("/v3/spaces/{}/members", space_pk_encoded);
    let (status, _headers, _body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "new_user_pks": vec![new_user.clone().pk, new_user_2.clone().pk],
            "removed_user_pks": Vec::<Partition>::new()
        },
        response_type: UpsertInvitationResponse
    };

    assert_eq!(status, 200);

    let (status, _headers, _body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "new_user_pks": vec![new_user_3.pk],
            "removed_user_pks": vec![new_user.pk]
        },
        response_type: UpsertInvitationResponse
    };

    assert_eq!(status, 200);
}

#[tokio::test]
async fn test_list_invitation_handler() {
    let TestContextV3 {
        ddb,
        app,
        test_user: (_user, headers),
        ..
    } = setup_v3().await;

    let app_state = create_app_state();
    let _cli = &app_state.dynamo.client;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");

    let (new_user, _headers) = create_user_session(app.clone(), &ddb).await;
    let (new_user_2, _headers) = create_user_session(app.clone(), &ddb).await;

    let path = format!("/v3/spaces/{}/members", space_pk_encoded);
    let (status, _headers, _body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "new_user_pks": vec![new_user.clone().pk, new_user_2.clone().pk],
            "removed_user_pks": Vec::<Partition>::new()
        },
        response_type: UpsertInvitationResponse
    };

    assert_eq!(status, 200);

    // Wait for eventual consistency in localstack DynamoDB
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let (status, _headers, body) = get! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        response_type: Vec<SpaceInvitationMemberResponse>
    };

    assert_eq!(status, 200);
    assert_eq!(body.len(), 2);
}

#[tokio::test]
async fn test_prevent_upsert_invitations() {
    let TestContextV3 {
        app,
        test_user: (user, headers),
        ..
    } = setup_v3().await;

    let app_state = create_app_state();
    let _cli = &app_state.dynamo.client;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");

    let path = format!("/v3/spaces/{}/members", space_pk_encoded);
    let (status, _headers, _body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "new_user_pks": vec![user.clone().pk],
            "removed_user_pks": Vec::<Partition>::new()
        },
        response_type: UpsertInvitationResponse
    };

    assert_eq!(status, 200);

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

    let path = format!("/v3/spaces/{}/members", space_pk_encoded);
    let (status, _headers, _body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "new_user_pks": vec![user.clone().pk],
            "removed_user_pks": Vec::<Partition>::new()
        },
        response_type: UpsertInvitationResponse
    };

    assert_eq!(status, 400);
}

#[tokio::test]
async fn test_prevent_resent_code_handler() {
    let TestContextV3 {
        app,
        test_user: (user, headers),
        ..
    } = setup_v3().await;

    let app_state = create_app_state();
    let _cli = &app_state.dynamo.client;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");

    let path = format!("/v3/spaces/{}/members", space_pk_encoded);
    let (status, _headers, _body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "new_user_pks": vec![user.clone().pk],
            "removed_user_pks": Vec::<Partition>::new()
        },
        response_type: UpsertInvitationResponse
    };

    assert_eq!(status, 200);

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

    tracing::debug!(
        "space url: {:?}",
        format!("/v3/spaces/{}/members", space_pk.to_string())
    );

    let (status, _headers, _body) = patch! {
        app: app,
        path: format!("/v3/spaces/{}/members", space_pk.to_string()),
        headers: headers.clone(),
        body: {
            "email": user.clone().email
        }
    };

    assert_eq!(status, 400);
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
