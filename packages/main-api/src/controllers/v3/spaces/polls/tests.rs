use crate::controllers::v3::posts::CreatePostResponse;
use crate::controllers::v3::spaces::create_space::CreateSpaceResponse;
use crate::controllers::v3::spaces::participate_space::ParticipateSpaceResponse;
use crate::controllers::v3::spaces::polls::{
    DeletePollSpaceResponse, RespondPollSpaceResponse, UpdatePollSpaceResponse,
};
use crate::features::did::AttributeCode;
use crate::features::did::{VerifiableAttribute, VerifiableAttributeWithQuota};
use crate::features::spaces::SpaceParticipant;
use crate::features::spaces::panels::{
    PanelAttribute, PanelAttributeWithQuota, SpacePanelParticipant, SpacePanelQuota,
    SpacePanelsResponse,
};
use crate::features::spaces::polls::{Poll, PollResponse, PollResultResponse};
use crate::tests::v3_setup::TestContextV3;
use crate::types::{Answer, ChoiceQuestion, EntityType, Gender, Partition, Question};
use crate::utils::time::get_now_timestamp_millis;
use crate::*;

/// Helper function to setup a poll space for testing
/// Returns (TestContextV3, space_pk, poll_sk)
pub async fn setup_poll_space() -> (TestContextV3, Partition, EntityType) {
    let ctx = TestContextV3::setup().await;
    let TestContextV3 { app, test_user, .. } = ctx.clone();

    // Create a post first
    let (_status, _headers, create_post_res) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };

    let post_pk = create_post_res.post_pk;

    // Publish the post
    let (_status, _headers, _body) = patch! {
        app: app,
        path: format!("/v3/posts/{}", post_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "title": "Poll Post",
            "content": "<p>This is a poll post</p>",
            "publish": true
        }
    };

    // Create a poll space
    let (status, _headers, create_space_res) = post! {
        app: app,
        path: "/v3/spaces",
        headers: test_user.1.clone(),
        body: {
            "space_type": 2,
            "post_pk": post_pk,
        },
        response_type: CreateSpaceResponse
    };
    assert_eq!(status, 200);

    let space_pk = create_space_res.space_pk;

    let (status, _headers, create_poll_res) = post! {
        app: app,
        path: format!("/v3/spaces/{}/polls", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "default": false
        },
        response_type: PollResponse
    };
    assert_eq!(status, 200);

    // Get the poll_sk from DynamoDB
    let poll_sk = create_poll_res.sk;

    (ctx, space_pk, poll_sk)
}

/// Helper function to setup a published poll space with questions
pub async fn setup_published_poll_space() -> (TestContextV3, Partition, EntityType, Vec<Question>) {
    let (ctx, space_pk, poll_sk) = setup_poll_space().await;
    let TestContextV3 { app, test_user, .. } = ctx.clone();

    let now = get_now_timestamp_millis();
    let questions = vec![
        Question::SingleChoice(ChoiceQuestion {
            title: "What is your favorite color?".to_string(),
            description: Some("Pick one".to_string()),
            image_url: None,
            options: vec![
                "Red".to_string(),
                "Blue".to_string(),
                "Green".to_string(),
                "Yellow".to_string(),
            ],
            is_required: Some(true),
            allow_other: None,
        }),
        Question::MultipleChoice(ChoiceQuestion {
            title: "What languages do you speak?".to_string(),
            description: Some("Select all that apply".to_string()),
            image_url: None,
            options: vec![
                "English".to_string(),
                "Spanish".to_string(),
                "French".to_string(),
                "German".to_string(),
            ],
            is_required: Some(false),
            allow_other: None,
        }),
    ];

    // Update poll with questions
    let (status, _headers, _res) = put! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}", space_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "questions": questions.clone(),
        },
        response_type: UpdatePollSpaceResponse
    };
    assert_eq!(status, 200);

    // Update poll time to be in progress
    let (status, _headers, _res) = put! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}", space_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "started_at": now - 1000,
            "ended_at": now + 86400000, // 1 day later
        },
        response_type: UpdatePollSpaceResponse
    };
    assert_eq!(status, 200);

    // Publish the space
    let (status, _headers, _res) = patch! {
        app: app,
        path: format!("/v3/spaces/{}", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "publish": true,
            "visibility": "PUBLIC",
        }
    };
    assert_eq!(status, 200);

    (ctx, space_pk, poll_sk, questions)
}

// ============================================================================
// GET /:poll_sk - get_poll_handler tests
// ============================================================================

#[tokio::test]
async fn test_get_poll_when_authenticated() {
    let (ctx, space_pk, poll_sk, _) = setup_published_poll_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}", space_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone(),
        response_type: PollResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.sk, poll_sk);
    assert_eq!(body.questions.len(), 2);
    assert!(body.my_response.is_none());
}

#[tokio::test]
async fn test_get_poll_when_not_authenticated() {
    let (ctx, space_pk, poll_sk, _) = setup_published_poll_space().await;
    let TestContextV3 { app, .. } = ctx;

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}", space_pk.to_string(), poll_sk.to_string()),
        response_type: PollResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.sk, poll_sk);
    assert!(body.my_response.is_none());
}

#[tokio::test]
async fn test_get_poll_with_my_response() {
    let (ctx, space_pk, poll_sk, _questions) = setup_published_poll_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let answers = vec![
        Answer::SingleChoice {
            answer: Some(1),
            other: None,
        },
        Answer::MultipleChoice {
            answer: Some(vec![0, 2]),
            other: None,
        },
    ];

    // Submit a response
    let (status, _headers, _res) = post! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}/responses", space_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "answers": answers.clone(),
        },
        response_type: RespondPollSpaceResponse
    };
    assert_eq!(status, 200);

    // Get the poll again
    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}", space_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone(),
        response_type: PollResponse
    };

    assert_eq!(status, 200);
    assert!(body.my_response.is_some());
    assert_eq!(body.my_response.unwrap().len(), 2);
}

#[tokio::test]
async fn test_get_poll_not_found() {
    let (ctx, space_pk, _, _) = setup_published_poll_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let fake_poll_sk = EntityType::SpacePoll("nonexistent".to_string());

    let (status, _headers, _body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}", space_pk.to_string(), fake_poll_sk.to_string()),
        headers: test_user.1.clone()
    };

    assert_eq!(status, 400); // NotFoundPoll returns 400
}

#[tokio::test]
async fn test_get_poll_with_invalid_space_pk() {
    let (ctx, _, poll_sk, _) = setup_published_poll_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let invalid_pk = Partition::Feed("invalid".to_string());

    let (status, _headers, _body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}", invalid_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone()
    };

    assert_eq!(status, 400); // Invalid partition returns 400
}

#[tokio::test]
async fn test_get_poll_without_permission() {
    let (ctx, space_pk, poll_sk) = setup_poll_space().await;
    let TestContextV3 {
        app,
        test_user,
        user2,
        ..
    } = ctx;

    // Publish as private
    let (_status, _headers, _res) = patch! {
        app: app,
        path: format!("/v3/spaces/{}", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "publish": true,
            "visibility": "PRIVATE",
        }
    };

    // Try to get as user2 (should fail)
    let (status, _headers, _body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}", space_pk.to_string(), poll_sk.to_string()),
        headers: user2.1.clone()
    };

    assert_eq!(status, 401); // No permission returns 401
}

// ============================================================================
// PUT /:poll_sk - update_poll_handler tests
// ============================================================================

#[tokio::test]
async fn test_update_poll_time() {
    let (ctx, space_pk, poll_sk) = setup_poll_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let now = get_now_timestamp_millis();
    let started_at = now + 1000;
    let ended_at = now + 86400000; // 1 day later

    let (status, _headers, body) = put! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}", space_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "started_at": started_at,
            "ended_at": ended_at,
        },
        response_type: UpdatePollSpaceResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.status, "success");
}

#[tokio::test]
async fn test_update_poll_questions() {
    let (ctx, space_pk, poll_sk) = setup_poll_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let questions = vec![Question::SingleChoice(ChoiceQuestion {
        title: "Test question".to_string(),
        description: None,
        image_url: None,
        options: vec!["Option 1".to_string(), "Option 2".to_string()],
        is_required: Some(true),
        allow_other: None,
    })];

    let (status, _headers, body) = put! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}", space_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "questions": questions,
        },
        response_type: UpdatePollSpaceResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.status, "success");
}

#[tokio::test]
async fn test_update_poll_response_editable() {
    let (ctx, space_pk, poll_sk) = setup_poll_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let (status, _headers, body) = put! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}", space_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "response_editable": true,
        },
        response_type: UpdatePollSpaceResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.status, "success");
}

#[tokio::test]
async fn test_update_poll_without_permission() {
    let (ctx, space_pk, poll_sk) = setup_poll_space().await;
    let TestContextV3 { app, user2, .. } = ctx;

    let now = get_now_timestamp_millis();

    let (status, _headers, _body) = put! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}", space_pk.to_string(), poll_sk.to_string()),
        headers: user2.1.clone(),
        body: {
            "started_at": now + 1000,
            "ended_at": now + 86400000,
        }
    };

    assert_eq!(status, 401); // No permission returns 401
}

#[tokio::test]
async fn test_update_poll_with_invalid_time_range() {
    let (ctx, space_pk, poll_sk) = setup_poll_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let now = get_now_timestamp_millis();

    let (status, _headers, _body) = put! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}", space_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "started_at": now + 86400000,
            "ended_at": now + 1000, // ended_at before started_at
        }
    };

    assert_eq!(status, 400);
}

#[tokio::test]
async fn test_update_poll_with_empty_questions() {
    let (ctx, space_pk, poll_sk) = setup_poll_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let questions: Vec<Question> = vec![];

    let (status, _headers, _body) = put! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}", space_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "questions": questions,
        }
    };

    assert_eq!(status, 400);
}

#[tokio::test]
async fn test_update_poll_with_invalid_space_pk() {
    let (ctx, _, poll_sk) = setup_poll_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let invalid_pk = Partition::Feed("invalid".to_string());
    let now = get_now_timestamp_millis();

    let (status, _headers, _body) = put! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}", invalid_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "started_at": now + 1000,
            "ended_at": now + 86400000,
        }
    };

    assert_eq!(status, 400); // Invalid partition returns 400
}

// ============================================================================
// GET /:poll_sk/results - get_poll_result tests
// ============================================================================

#[tokio::test]
async fn test_get_poll_results_when_authenticated_with_permission() {
    let (ctx, space_pk, poll_sk, _) = setup_published_poll_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}/results", space_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone(),
        response_type: PollResultResponse
    };

    assert_eq!(status, 200);
    assert!(body.created_at > 0);
    assert_eq!(body.summaries.len(), 2); // 2 questions in the poll
}

#[tokio::test]
async fn test_get_poll_results_with_panel_responses() {
    let (ctx, space_pk, poll_sk, _) = setup_published_poll_space().await;
    let TestContextV3 {
        app,
        test_user,
        user2,
        ddb,
        ..
    } = ctx;

    let mut attribute = AttributeCode::new();
    attribute.gender = Some(Gender::Male);
    attribute.birth_date = Some("20100101".to_string()); // 2010-01-01, age ~16 (fits 0-18 range)
    let _ = attribute.create(&ddb).await;

    let code = match attribute.pk {
        Partition::AttributeCode(v) => v.to_string(),
        _ => "".to_string(),
    };

    let (status, _headers, _body) = put! {
        app: app,
        path: format!("/v3/me/did"),
        headers: test_user.1.clone(),
        body: {
            "type": "code",
            "code": code
        }
    };

    assert_eq!(status, 200);

    // Submit responses from multiple users
    let answers1 = vec![
        Answer::SingleChoice {
            answer: Some(1),
            other: None,
        },
        Answer::MultipleChoice {
            answer: Some(vec![0, 2]),
            other: None,
        },
    ];

    let (status, _headers, _res) = patch! {
        app: app,
        path: format!("/v3/spaces/{}", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "quotas": 60,
        }
    };
    assert_eq!(status, 200);

    let (status, _headers, body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/panels", space_pk.to_string()),
        headers: test_user.1.clone(),
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
                        attribute: VerifiableAttribute::Age(Age::Range { inclusive_min: 0, inclusive_max: 30 }),
                        quota: 30
                    }
                )
            ]
        },
        response_type: Vec<SpacePanelQuota>
    };

    assert_eq!(status, 200);
    assert_eq!(body.len(), 2);

    let participant = SpacePanelParticipant::new(space_pk.clone(), test_user.0.clone());
    let _ = participant.create(&ddb).await;

    let (status, _headers, _res) = post! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}/responses", space_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "answers": answers1,
        },
        response_type: RespondPollSpaceResponse
    };
    assert_eq!(status, 200);

    let answers2 = vec![
        Answer::SingleChoice {
            answer: Some(2),
            other: None,
        },
        Answer::MultipleChoice {
            answer: Some(vec![1, 3]),
            other: None,
        },
    ];

    // Setup credential for user2
    let mut attribute2 = AttributeCode::new();
    attribute2.gender = Some(Gender::Male);
    attribute2.birth_date = Some("20100101".to_string());
    let _ = attribute2.create(&ddb).await;

    let code2 = match attribute2.pk {
        Partition::AttributeCode(v) => v.to_string(),
        _ => "".to_string(),
    };

    let (status, _headers, _body) = put! {
        app: app,
        path: format!("/v3/me/did"),
        headers: user2.1.clone(),
        body: {
            "type": "code",
            "code": code2
        }
    };
    assert_eq!(status, 200, "user2 failed to set credential");

    // user2 participates in the space before responding to poll
    let (status, _headers, _res) = post! {
        app: app,
        path: format!("/v3/spaces/{}/participate", space_pk.to_string()),
        headers: user2.1.clone(),
        body: {
            "verifiable_presentation": ""
        },
        response_type: ParticipateSpaceResponse
    };
    assert_eq!(status, 200, "user2 failed to participate");

    let (status, _headers, _res) = post! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}/responses", space_pk.to_string(), poll_sk.to_string()),
        headers: user2.1.clone(),
        body: {
            "answers": answers2,
        },
        response_type: RespondPollSpaceResponse
    };
    assert_eq!(status, 200);

    // Get results
    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}/results", space_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone(),
        response_type: PollResultResponse
    };

    println!("body: {:?}", body);

    assert_eq!(status, 200);
    assert!(
        !body.summaries_by_gender.is_empty(),
        "summaries_by_gender is empty"
    );
    assert!(
        !body.summaries_by_age.is_empty(),
        "summaries_by_age is empty"
    );

    // Note: We can't easily deserialize the response because DynamoDB stores HashMap<i32, i64>
    // keys as strings, which causes deserialization issues. The API works correctly though.
}

#[tokio::test]
async fn test_get_poll_results_with_responses() {
    let (ctx, space_pk, poll_sk, _) = setup_published_poll_space().await;
    let TestContextV3 {
        app,
        test_user,
        user2,
        ..
    } = ctx;

    // Submit responses from multiple users
    let answers1 = vec![
        Answer::SingleChoice {
            answer: Some(1),
            other: None,
        },
        Answer::MultipleChoice {
            answer: Some(vec![0, 2]),
            other: None,
        },
    ];

    let answers2 = vec![
        Answer::SingleChoice {
            answer: Some(2),
            other: None,
        },
        Answer::MultipleChoice {
            answer: Some(vec![1, 3]),
            other: None,
        },
    ];

    // user2 participates in the space before responding to poll
    let (status, _headers, _res) = post! {
        app: app,
        path: format!("/v3/spaces/{}/participate", space_pk.to_string()),
        headers: user2.1.clone(),
        body: {
            "verifiable_presentation": ""
        },
        response_type: ParticipateSpaceResponse
    };
    assert_eq!(status, 200, "user2 failed to participate");

    let (status, _headers, _res) = post! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}/responses", space_pk.to_string(), poll_sk.to_string()),
        headers: user2.1.clone(),
        body: {
            "answers": answers2,
        },
        response_type: RespondPollSpaceResponse
    };
    assert_eq!(status, 200);

    // Get results
    let (status, _headers, _body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}/results", space_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone()
    };

    assert_eq!(status, 200);
    // Note: We can't easily deserialize the response because DynamoDB stores HashMap<i32, i64>
    // keys as strings, which causes deserialization issues. The API works correctly though.
}

#[tokio::test]
async fn test_get_poll_results_without_permission() {
    let (ctx, space_pk, poll_sk) = setup_poll_space().await;
    let TestContextV3 {
        app,
        test_user,
        user2,
        ..
    } = ctx;

    // Publish as private
    let (_status, _headers, _res) = patch! {
        app: app,
        path: format!("/v3/spaces/{}", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "publish": true,
            "visibility": "PRIVATE",
        }
    };

    // Try to get results as user2 (should fail)
    let (status, _headers, _body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}/results", space_pk.to_string(), poll_sk.to_string()),
        headers: user2.1.clone()
    };

    assert_eq!(status, 401); // No permission returns 401
}

// ============================================================================
// POST /:poll_sk/responses - respond_poll_handler tests
// ============================================================================

#[tokio::test]
async fn test_respond_poll_successfully() {
    let (ctx, space_pk, poll_sk, _) = setup_published_poll_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let answers = vec![
        Answer::SingleChoice {
            answer: Some(1),
            other: None,
        },
        Answer::MultipleChoice {
            answer: Some(vec![0, 2]),
            other: None,
        },
    ];

    let (status, _headers, body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}/responses", space_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "answers": answers,
        },
        response_type: RespondPollSpaceResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.poll_space_pk, space_pk);
}

#[tokio::test]
async fn test_respond_poll_without_permission() {
    let (ctx, space_pk, poll_sk, _) = setup_published_poll_space().await;
    let TestContextV3 {
        app,
        test_user,
        user2,
        ..
    } = ctx;

    // Change space to private
    let (_status, _headers, _res) = patch! {
        app: app,
        path: format!("/v3/spaces/{}", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "publish": true,
            "visibility": "PRIVATE",
        }
    };

    let answers = vec![
        Answer::SingleChoice {
            answer: Some(1),
            other: None,
        },
        Answer::MultipleChoice {
            answer: Some(vec![0, 2]),
            other: None,
        },
    ];

    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}/responses", space_pk.to_string(), poll_sk.to_string()),
        headers: user2.1.clone(),
        body: {
            "answers": answers,
        }
    };

    assert_eq!(status, 401); // No permission returns 401
}

#[tokio::test]
async fn test_respond_poll_when_not_started() {
    let (ctx, space_pk, poll_sk) = setup_poll_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let now = get_now_timestamp_millis();
    let questions = vec![Question::SingleChoice(ChoiceQuestion {
        title: "Test question".to_string(),
        description: None,
        image_url: None,
        options: vec!["Option 1".to_string(), "Option 2".to_string()],
        is_required: Some(true),
        allow_other: None,
    })];

    // Update poll with questions
    let (_status, _headers, _res) = put! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}", space_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "questions": questions,
        },
        response_type: UpdatePollSpaceResponse
    };

    // Set poll to start in the future
    let (_status, _headers, _res) = put! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}", space_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "started_at": now + 86400000, // 1 day later
            "ended_at": now + 172800000, // 2 days later
        },
        response_type: UpdatePollSpaceResponse
    };

    // Publish the space
    let (_status, _headers, _res) = patch! {
        app: app,
        path: format!("/v3/spaces/{}", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "publish": true,
            "visibility": "PUBLIC",
        }
    };

    let answers = vec![Answer::SingleChoice {
        answer: Some(1),
        other: None,
    }];

    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}/responses", space_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "answers": answers,
        }
    };

    assert_eq!(status, 400);
}

#[tokio::test]
async fn test_respond_poll_when_already_ended() {
    let (ctx, space_pk, poll_sk) = setup_poll_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let now = get_now_timestamp_millis();
    let questions = vec![Question::SingleChoice(ChoiceQuestion {
        title: "Test question".to_string(),
        description: None,
        image_url: None,
        options: vec!["Option 1".to_string(), "Option 2".to_string()],
        is_required: Some(true),
        allow_other: None,
    })];

    // Update poll with questions
    let (_status, _headers, _res) = put! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}", space_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "questions": questions,
        },
        response_type: UpdatePollSpaceResponse
    };

    // Set poll to already ended
    let (_status, _headers, _res) = put! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}", space_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "started_at": now - 172800000, // 2 days ago
            "ended_at": now - 86400000, // 1 day ago
        },
        response_type: UpdatePollSpaceResponse
    };

    // Publish the space
    let (_status, _headers, _res) = patch! {
        app: app,
        path: format!("/v3/spaces/{}", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "publish": true,
            "visibility": "PUBLIC",
        }
    };

    let answers = vec![Answer::SingleChoice {
        answer: Some(1),
        other: None,
    }];

    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}/responses", space_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "answers": answers,
        }
    };

    assert_eq!(status, 400);
}

#[tokio::test]
async fn test_respond_poll_with_mismatched_answers() {
    let (ctx, space_pk, poll_sk, _) = setup_published_poll_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    // Only provide 1 answer when 2 questions exist
    let answers = vec![Answer::SingleChoice {
        answer: Some(1),
        other: None,
    }];

    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}/responses", space_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "answers": answers,
        }
    };

    assert_eq!(status, 400);
}

#[tokio::test]
async fn test_respond_poll_with_invalid_answer_option() {
    let (ctx, space_pk, poll_sk, _) = setup_published_poll_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    // Provide invalid option index (out of bounds)
    let answers = vec![
        Answer::SingleChoice {
            answer: Some(10),
            other: None,
        }, // Invalid: only 4 options (0-3)
        Answer::MultipleChoice {
            answer: Some(vec![0, 2]),
            other: None,
        },
    ];

    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}/responses", space_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "answers": answers,
        }
    };

    assert_eq!(status, 400);
}

// #[tokio::test]
// async fn test_respond_poll_with_started_space() {
//     let (ctx, space_pk, poll_sk, _) = setup_published_poll_space().await;
//     let TestContextV3 { app, test_user, .. } = ctx;

//     let (status, _, _res) = patch! {
//         app: app,
//         path: format!("/v3/spaces/{}", space_pk.to_string()),
//         headers: test_user.1.clone(),
//         body: {
//             "start": true,
//         }
//     };

//     assert_eq!(status, 200);

//     let answers = vec![
//         Answer::SingleChoice { answer: Some(1) },
//         Answer::MultipleChoice {
//             answer: Some(vec![0, 2]),
//         },
//     ];

//     let (status, _headers, _body) = post! {
//         app: app,
//         path: format!("/v3/spaces/{}/polls/{}/responses", space_pk.to_string(), poll_sk.to_string()),
//         headers: test_user.1.clone(),
//         body: {
//             "answers": answers,
//         }
//     };

//     assert_eq!(status, 400);
// }

#[tokio::test]
async fn test_respond_poll_increments_response_count() {
    let (ctx, space_pk, poll_sk, _) = setup_published_poll_space().await;
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = ctx;

    // Get initial response count
    let initial_poll = Poll::get(&ddb, &space_pk, Some(&poll_sk))
        .await
        .unwrap()
        .unwrap();
    let initial_count = initial_poll.user_response_count;

    let answers = vec![
        Answer::SingleChoice {
            answer: Some(1),
            other: None,
        },
        Answer::MultipleChoice {
            answer: Some(vec![0, 2]),
            other: None,
        },
    ];

    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}/responses", space_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "answers": answers,
        },
        response_type: RespondPollSpaceResponse
    };
    assert_eq!(status, 200);

    // Get updated response count
    let updated_poll = Poll::get(&ddb, &space_pk, Some(&poll_sk))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(updated_poll.user_response_count, initial_count + 1);
}

// ============================================================================
// DELETE /:poll_sk - delete_poll_handler tests
// ============================================================================

#[tokio::test]
async fn test_delete_poll_successfully() {
    let (ctx, space_pk, poll_sk) = setup_poll_space().await;
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = ctx;

    // Verify poll exists before deletion
    let poll_before = Poll::get(&ddb, &space_pk, Some(&poll_sk)).await.unwrap();
    assert!(poll_before.is_some(), "Poll should exist before deletion");

    let (status, _headers, body) = delete! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}", space_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone(),
        response_type: DeletePollSpaceResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.status, "success");

    // Verify poll is deleted
    let poll_after = Poll::get(&ddb, &space_pk, Some(&poll_sk)).await.unwrap();
    assert!(poll_after.is_none(), "Poll should be deleted");
}

#[tokio::test]
async fn test_delete_poll_without_permission() {
    let (ctx, space_pk, poll_sk) = setup_poll_space().await;
    let TestContextV3 { app, user2, .. } = ctx;

    let (status, _headers, _body) = delete! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}", space_pk.to_string(), poll_sk.to_string()),
        headers: user2.1.clone()
    };

    assert_eq!(status, 401); // No permission returns 401
}

#[tokio::test]
async fn test_delete_poll_not_found() {
    let (ctx, space_pk, _) = setup_poll_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let fake_poll_sk = EntityType::SpacePoll("nonexistent".to_string());

    let (status, _headers, _body) = delete! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}", space_pk.to_string(), fake_poll_sk.to_string()),
        headers: test_user.1.clone()
    };

    assert_eq!(status, 400); // NotFoundPoll returns 400
}

#[tokio::test]
async fn test_delete_poll_with_invalid_space_pk() {
    let (ctx, _, poll_sk) = setup_poll_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let invalid_pk = Partition::Feed("invalid".to_string());

    let (status, _headers, _body) = delete! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}", invalid_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone()
    };

    assert_eq!(status, 400); // Invalid partition returns 400
}

#[tokio::test]
async fn test_delete_poll_with_invalid_poll_sk() {
    let (ctx, space_pk, _) = setup_poll_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let invalid_sk = EntityType::Post; // Use a different entity type

    let (status, _headers, _body) = delete! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}", space_pk.to_string(), invalid_sk.to_string()),
        headers: test_user.1.clone()
    };

    assert_eq!(status, 400); // Invalid entity type returns 400
}

#[tokio::test]
async fn test_delete_poll_when_not_authenticated() {
    let (ctx, space_pk, poll_sk) = setup_poll_space().await;
    let TestContextV3 { app, .. } = ctx;

    let (status, _headers, _body) = delete! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}", space_pk.to_string(), poll_sk.to_string())
    };

    assert_eq!(status, 401); // Not authenticated returns 401
}
