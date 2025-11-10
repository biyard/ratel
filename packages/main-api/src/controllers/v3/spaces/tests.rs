use crate::controllers::v3::posts::CreatePostResponse;
use crate::models::SpaceCommon;
use crate::types::{ListItemsResponse, SpacePublishState, SpaceType};
use crate::*;
use crate::{
    controllers::v3::spaces::create_space::CreateSpaceResponse, tests::v3_setup::TestContextV3,
};

#[tokio::test]
pub async fn test_create_space() {
    let (ctx, post_pk) = setup_post().await;

    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = ctx;

    let (status, _, res) = post! {
        app: app,
        path: "/v3/spaces",
        headers: headers.clone(),
        body: {
            "space_type": 2,
            "post_pk": post_pk,
        },
        response_type: CreateSpaceResponse
    };
    tracing::debug!("Create space response: {:?}", res);
    assert_eq!(status, 200);

    let space_pk = res.space_pk;
    let encoded_pk = percent_encoding::percent_encode(
        space_pk.to_string().as_bytes(),
        percent_encoding::NON_ALPHANUMERIC,
    )
    .to_string();
    let path = format!("/v3/spaces/{}", encoded_pk);

    let (status, _, res) = delete! {
        app: app,
        path: path,
        headers: headers.clone()
    };
    tracing::debug!("Delete space response: {:?}", res);
    assert_eq!(status, 200);
}

#[tokio::test]
async fn test_list_spaces() {
    let mut last_space_pk = String::new();

    for _ in 0..11 {
        let (ctx, post_pk) = setup_post().await;

        let TestContextV3 {
            app,
            test_user: (_user, headers),
            ..
        } = ctx;

        let (status, _, res) = post! {
            app: app,
            path: "/v3/spaces",
            headers: headers.clone(),
            body: {
                "space_type": 2,
                "post_pk": post_pk,
            },
            response_type: CreateSpaceResponse
        };

        assert_eq!(status, 200);

        let (status, _, _res) = patch! {
            app: app,
            path: format!("/v3/spaces/{}", res.space_pk.to_string()),
            headers: headers.clone(),
            body: {
                "publish": true,
                "visibility": "PUBLIC",
            }
        };
        tracing::info!("Create space response: {:?}", res);
        assert_eq!(status, 200, "error: {:?}", _res);

        last_space_pk = res.space_pk.to_string();
    }

    let (ctx, post_pk) = setup_post().await;

    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = ctx;

    let (status, _, _res) = post! {
        app: app,
        path: "/v3/spaces",
        headers: headers.clone(),
        body: {
            "space_type": 2,
            "post_pk": post_pk,
        },
        response_type: CreateSpaceResponse
    };

    assert_eq!(status, 200);

    let (ctx, post_pk) = setup_post().await;
    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = ctx;

    let (status, _, res) = post! {
        app: app,
        path: "/v3/spaces",
        headers: headers.clone(),
        body: {
            "space_type": 2,
            "post_pk": post_pk,
        },
        response_type: CreateSpaceResponse
    };

    assert_eq!(status, 200);

    let (status, _, _res) = patch! {
        app: app,
        path: format!("/v3/spaces/{}", res.space_pk.to_string()),
        headers: headers.clone(),
        body: {
            "publish": true,
            "visibility": "PRIVATE",
        }
    };
    tracing::debug!("Create space response: {:?}", res);
    assert_eq!(status, 200, "error: {:?}", _res);

    let (status, _, list_res) = get! {
        app: app,
        path: "/v3/spaces",
        response_type: ListItemsResponse<SpaceCommon>,
    };

    assert_eq!(status, 200);
    assert_eq!(list_res.items.len(), 10);
    assert!(list_res.bookmark.is_some());
    assert!(
        list_res
            .items
            .iter()
            .all(|item| item.publish_state == SpacePublishState::Published
                && item.visibility == crate::types::SpaceVisibility::Public)
    );
    assert_eq!(
        list_res.items.first().unwrap().pk.to_string(),
        last_space_pk
    )
}

#[tokio::test]
pub async fn test_start_space() {
    let (ctx, post_pk) = setup_post().await;

    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = ctx;

    let (status, _, res) = post! {
        app: app,
        path: "/v3/spaces",
        headers: headers.clone(),
        body: {
            "space_type": 2,
            "post_pk": post_pk,
        },
        response_type: CreateSpaceResponse
    };
    tracing::debug!("Create space response: {:?}", res);
    assert_eq!(status, 200);

    let space_pk = res.space_pk;
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
}

#[tokio::test]
async fn test_get_space() {}

pub async fn setup_post() -> (TestContextV3, String) {
    let ctx = TestContextV3::setup().await;
    let TestContextV3 { app, test_user, .. } = ctx.clone();

    let (_status, _headers, create_body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };

    let (_status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", create_body.post_pk.to_string()),
        headers: test_user.1.clone()
    };

    let post_pk = body["post"]["pk"].as_str().unwrap_or_default().to_string();

    let (_status, _headers, _body) = patch! {
        app: app,
        path: format!("/v3/posts/{}", post_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "title": "Post for Space",
            "content": "<p>post for space contents</p>",
            "publish": true
        }
    };
    assert_eq!(_body["user_pk"], test_user.0.pk.to_string());

    return (ctx, post_pk);
}

pub async fn setup_space(space_type: SpaceType) -> (TestContextV3, String) {
    let (ctx, post_pk) = setup_post().await;
    let TestContextV3 { app, test_user, .. } = ctx.clone();

    let (_status, _headers, create_body) = post! {
        app: app,
        path: "/v3/spaces",
        headers: test_user.1.clone(),
        body: {
            "space_type": space_type as u8,
            "post_pk": post_pk,
        },
        response_type: CreateSpaceResponse
    };

    let space_pk = create_body.space_pk.to_string();

    return (ctx, space_pk);
}

#[tokio::test]
async fn test_check_prerequisites_without_poll() {
    use crate::controllers::v3::spaces::check_prerequisites::CheckPrerequisitesResponse;

    let (ctx, space_pk) = setup_space(SpaceType::Deliberation).await;

    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = ctx;

    let encoded_space_pk = space_pk.replace('#', "%23");

    // Check prerequisites for space without default poll
    let (status, _, res) = get! {
        app: app,
        path: format!("/v3/spaces/{}/prerequisites", encoded_space_pk),
        headers: headers.clone(),
        response_type: CheckPrerequisitesResponse
    };

    assert_eq!(status, 200);
    assert_eq!(
        res.completed, true,
        "Prerequisites should be completed when there's no default poll"
    );
    assert_eq!(res.prerequisite_type, None);
    assert_eq!(res.poll_pk, None);
}

#[tokio::test]
async fn test_check_prerequisites_with_empty_poll() {
    use crate::controllers::v3::spaces::check_prerequisites::CheckPrerequisitesResponse;
    use crate::features::spaces::polls::PollResponse;

    let (ctx, space_pk) = setup_space(SpaceType::Deliberation).await;

    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = ctx;

    let encoded_space_pk = space_pk.replace('#', "%23");

    // Create a default poll with no questions
    let (status, _, _poll_res) = post! {
        app: app,
        path: format!("/v3/spaces/{}/polls", encoded_space_pk),
        headers: headers.clone(),
        body: {
            "default": true
        },
        response_type: PollResponse
    };

    assert_eq!(status, 200, "Failed to create poll");

    // Check prerequisites - should be completed since poll has no questions
    let (status, _, res) = get! {
        app: app,
        path: format!("/v3/spaces/{}/prerequisites", encoded_space_pk),
        headers: headers.clone(),
        response_type: CheckPrerequisitesResponse
    };

    assert_eq!(status, 200);
    assert_eq!(
        res.completed, true,
        "Prerequisites should be completed when poll has no questions"
    );
}

#[tokio::test]
async fn test_check_prerequisites_workflow() {
    use crate::controllers::v3::spaces::check_prerequisites::CheckPrerequisitesResponse;
    use crate::controllers::v3::spaces::polls::UpdatePollSpaceResponse;
    use crate::features::spaces::polls::PollResponse;
    use crate::types::{Answer, ChoiceQuestion, Question};

    let (ctx, space_pk) = setup_space(SpaceType::Deliberation).await;

    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = ctx;

    let encoded_space_pk = space_pk.replace('#', "%23");

    // Create a default poll
    let (status, _, poll_res) = post! {
        app: app,
        path: format!("/v3/spaces/{}/polls", encoded_space_pk),
        headers: headers.clone(),
        body: {
            "default": true
        },
        response_type: PollResponse
    };

    assert_eq!(status, 200, "Failed to create poll");
    let poll_sk = poll_res.sk;

    // Add questions to the poll
    let questions = vec![
        Question::SingleChoice(ChoiceQuestion {
            title: "What is your age?".to_string(),
            description: Some("Pick one".to_string()),
            image_url: None,
            options: vec!["18-25".to_string(), "26-35".to_string(), "36+".to_string()],
            is_required: Some(true),
        }),
        Question::Subjective(crate::types::SubjectiveQuestion {
            title: "What is your opinion?".to_string(),
            description: "Share your thoughts".to_string(),
            is_required: Some(false),
        }),
    ];

    let (status, _, _res) = put! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}", encoded_space_pk, poll_sk.to_string()),
        headers: headers.clone(),
        body: {
            "questions": questions,
        },
        response_type: UpdatePollSpaceResponse
    };
    assert_eq!(status, 200, "Failed to update poll with questions");

    // Check prerequisites before answering - should NOT be completed
    let (status, _, res) = get! {
        app: app,
        path: format!("/v3/spaces/{}/prerequisites", encoded_space_pk),
        headers: headers.clone(),
        response_type: CheckPrerequisitesResponse
    };

    assert_eq!(status, 200);
    assert_eq!(
        res.completed, false,
        "Prerequisites should NOT be completed before answering poll"
    );
    assert_eq!(res.prerequisite_type, Some("default_poll".to_string()));
    assert!(res.poll_pk.is_some(), "poll_pk should be provided");
    assert!(res.message.is_some(), "message should be provided");

    // Submit poll answers
    let poll_sk_encoded = poll_sk.to_string().replace('#', "%23");
    let (status, _, _) = post! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}/responses", encoded_space_pk, poll_sk_encoded),
        headers: headers.clone(),
        body: {
            "answers": [
                Answer::SingleChoice { answer: Some(0) },
                Answer::Subjective { answer: Some("My opinion".to_string()) }
            ]
        }
    };

    assert_eq!(status, 200, "Failed to submit poll answers");

    // Wait for eventual consistency
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Check prerequisites after answering - should be completed
    let (status, _, res) = get! {
        app: app,
        path: format!("/v3/spaces/{}/prerequisites", encoded_space_pk),
        headers: headers.clone(),
        response_type: CheckPrerequisitesResponse
    };

    assert_eq!(status, 200);
    assert_eq!(
        res.completed, true,
        "Prerequisites should be completed after answering poll"
    );
    assert_eq!(res.prerequisite_type, Some("default_poll".to_string()));
}
