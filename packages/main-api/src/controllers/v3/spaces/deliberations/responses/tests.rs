use crate::controllers::v3::posts::create_post::CreatePostResponse;
use crate::types::File;
use crate::{
    controllers::v3::{
        posts::create_post::{CreatePostRequest, create_post_handler},
        spaces::deliberations::responses::create_response_answer::CreateDeliberationResponse,
    },
    get,
    models::space::{
        DeliberationDetailResponse, DeliberationSpaceResponse, DiscussionCreateRequest,
        SurveyCreateRequest,
    },
    post,
    tests::{
        create_app_state, get_auth,
        v3_setup::{TestContextV3, setup_v3},
    },
    types::{ChoiceQuestion, LinearScaleQuestion, SpaceVisibility, SurveyQuestion, SurveyStatus},
};

use bdk::prelude::axum::{Extension, Json, extract::State};

use crate::types::SurveyAnswer;

#[tokio::test]
async fn test_create_response_answer_handler() {
    let TestContextV3 {
        app,
        test_user: (user, headers),
        ..
    } = setup_v3().await;

    //FIXME: fix by session and one test code
    let app_state = create_app_state();
    let auth = get_auth(&user);

    let (status, _headers, post) = crate::post! {
        app: app,
        path: "/v3/posts",
        headers: headers.clone(),
        response_type: CreatePostResponse,
    };

    let feed_pk = post.post_pk.clone();

    // SPACE
    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/spaces/deliberation",
        headers: headers.clone(),
        body: {
            "feed_pk": feed_pk
        },
        response_type: CreateDeliberationResponse
    };

    assert_eq!(status, 200);

    let now = chrono::Utc::now().timestamp();
    let space_pk = body.metadata.deliberation.pk.clone();
    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/deliberation/{}", space_pk_encoded);

    let (status, _headers, body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "title": Some("deliberation title".to_string()),
            "html_contents": Some("<div>deliberation description</div>".to_string()),
            "visibility": SpaceVisibility::Public,
            "started_at": now,
            "ended_at": now + 86400,
            "files": vec![File {
                name: "deliberation summary file title".to_string(),
                size: "15KB".to_string(),
                ext: crate::types::FileExtension::PDF,
                url: None,
            }],
            "discussions": vec![DiscussionCreateRequest {
                discussion_pk: None,
                started_at: now,
                ended_at: now,
                name: "discussion title".to_string(),
                description: "discussion description".to_string(),
                user_ids: vec![],
            }],
            "elearning_files": vec![File {
                name: "deliberation elearning file title".to_string(),
                size: "15KB".to_string(),
                ext: crate::types::FileExtension::PDF,
                url: None,
            }],
            "surveys": vec![SurveyCreateRequest {
                survey_pk: None,
                started_at: now,
                ended_at: now + 10_000,
                status: SurveyStatus::Ready,
                questions: vec![
                    SurveyQuestion::SingleChoice(ChoiceQuestion {
                        title: "How did you hear about us?".into(),
                        description: Some("Pick one".into()),
                        image_url: None,
                        options: vec![
                            "Search".into(),
                            "Friend".into(),
                            "Social".into(),
                            "Other".into(),
                        ],
                        is_required: Some(true),
                    }),
                    SurveyQuestion::MultipleChoice(ChoiceQuestion {
                        title: "Which topics interest you?".into(),
                        description: None,
                        image_url: None,
                        options: vec![
                            "DeFi".into(),
                            "NFTs".into(),
                            "Governance".into(),
                            "Education".into(),
                        ],
                        is_required: Some(false),
                    }),
                    SurveyQuestion::LinearScale(LinearScaleQuestion {
                        title: "Rate your onboarding experience".into(),
                        description: Some("1 = Poor, 5 = Excellent".into()),
                        image_url: None,
                        min_value: 1,
                        max_value: 5,
                        min_label: "Poor".into(),
                        max_label: "Excellent".into(),
                        is_required: Some(true),
                    }),
                ],
            }],
            "recommendation_html_contents": Some(
                "<div>deliberation recommendation description</div>".to_string(),
            ),
            "recommendation_files": vec![File {
                name: "deliberation recommendation file title".to_string(),
                size: "15KB".to_string(),
                ext: crate::types::FileExtension::PDF,
                url: None,
            }],
        },
        response_type: DeliberationDetailResponse,
    };

    assert_eq!(status, 200);

    let space_pk = body.clone().deliberation.pk;
    let survey_pk = body.clone().surveys.pk;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/deliberation/{}/responses", space_pk_encoded);

    let (status, _headers, body) = post! (
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "survey_pk": survey_pk,
            "survey_type": crate::types::SurveyType::Survey,
            "answers": vec![
                SurveyAnswer::SingleChoice { answer: Some(1) },
                SurveyAnswer::MultipleChoice {
                    answer: Some(vec![1]),
                },
            ],
        },
        response_type: CreateDeliberationResponse,
    );

    assert_eq!(status, 200);

    let meta = &body.metadata;

    eprintln!("meta: {:?}", meta);

    assert_eq!(
        meta.surveys.user_responses.len(),
        1,
        "Failed to match user response answer length"
    );
    assert_eq!(
        meta.surveys.responses.len(),
        1,
        "Failed to match response answer length"
    );
}

#[tokio::test]
async fn test_get_response_answer_handler() {
    let TestContextV3 {
        app,
        test_user: (user, headers),
        ..
    } = setup_v3().await;

    //FIXME: fix by session and one test code
    let app_state = create_app_state();
    let auth = get_auth(&user);

    let (status, _headers, post) = crate::post! {
        app: app,
        path: "/v3/posts",
        headers: headers.clone(),
        response_type: CreatePostResponse,
    };

    let feed_pk = post.post_pk.clone();

    // SPACE

    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/spaces/deliberation",
        headers: headers.clone(),
        body: {
            "feed_pk": feed_pk
        },
        response_type: CreateDeliberationResponse
    };

    assert_eq!(status, 200);

    let now = chrono::Utc::now().timestamp();
    let space_pk = body.metadata.deliberation.pk.clone();
    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/deliberation/{}", space_pk_encoded);

    let (status, _headers, body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "title": Some("deliberation title".to_string()),
            "html_contents": Some("<div>deliberation description</div>".to_string()),
            "files": vec![File {
                name: "deliberation summary file title".to_string(),
                size: "15KB".to_string(),
                ext: crate::types::FileExtension::PDF,
                url: None,
            }],
            "visibility": SpaceVisibility::Public,
            "started_at": now,
            "ended_at": now + 86400,
            "discussions": vec![DiscussionCreateRequest {
                discussion_pk: None,
                started_at: now,
                ended_at: now,
                name: "discussion title".to_string(),
                description: "discussion description".to_string(),
                user_ids: vec![],
            }],
            "elearning_files": vec![File {
                name: "deliberation elearning file title".to_string(),
                size: "15KB".to_string(),
                ext: crate::types::FileExtension::PDF,
                url: None,
            }],
            "surveys": vec![SurveyCreateRequest {
                survey_pk: None,
                started_at: now,
                ended_at: now + 10_000,
                status: SurveyStatus::Ready,
                questions: vec![
                    SurveyQuestion::SingleChoice(ChoiceQuestion {
                        title: "How did you hear about us?".into(),
                        description: Some("Pick one".into()),
                        image_url: None,
                        options: vec![
                            "Search".into(),
                            "Friend".into(),
                            "Social".into(),
                            "Other".into(),
                        ],
                        is_required: Some(true),
                    }),
                    SurveyQuestion::MultipleChoice(ChoiceQuestion {
                        title: "Which topics interest you?".into(),
                        description: None,
                        image_url: None,
                        options: vec![
                            "DeFi".into(),
                            "NFTs".into(),
                            "Governance".into(),
                            "Education".into(),
                        ],
                        is_required: Some(false),
                    }),
                    SurveyQuestion::LinearScale(LinearScaleQuestion {
                        title: "Rate your onboarding experience".into(),
                        description: Some("1 = Poor, 5 = Excellent".into()),
                        image_url: None,
                        min_value: 1,
                        max_value: 5,
                        min_label: "Poor".into(),
                        max_label: "Excellent".into(),
                        is_required: Some(true),
                    }),
                ],
            }],
            "recommendation_html_contents": Some(
                "<div>deliberation recommendation description</div>".to_string(),
            ),
            "recommendation_files": vec![File {
                name: "deliberation recommendation file title".to_string(),
                size: "15KB".to_string(),
                ext: crate::types::FileExtension::PDF,
                url: None,
            }],
        },
        response_type: DeliberationDetailResponse,
    };

    assert_eq!(status, 200);

    let space_pk = body.deliberation.pk;
    let survey_pk = body.surveys.pk;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/deliberation/{}/responses", space_pk_encoded);

    let (status, _headers, body) = post! (
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "survey_pk": survey_pk,
            "survey_type": crate::types::SurveyType::Survey,
            "answers": vec![
                SurveyAnswer::SingleChoice { answer: Some(1) },
                SurveyAnswer::MultipleChoice {
                    answer: Some(vec![1]),
                },
            ],
        },
        response_type: CreateDeliberationResponse,
    );

    assert_eq!(status, 200);

    let meta = &body.metadata;

    assert_eq!(
        meta.surveys.user_responses.len(),
        1,
        "Failed to match user response answer length"
    );
    assert_eq!(
        meta.surveys.responses.len(),
        1,
        "Failed to match response answer length"
    );

    let response_pk = body.metadata.surveys.user_responses[0].pk.clone();

    let response_pk_encoded = response_pk.to_string().replace('#', "%23");
    eprintln!("response pk encoded: {:?}", response_pk_encoded.clone());
    let path = format!(
        "/v3/spaces/deliberation/{}/responses/{}",
        space_pk_encoded, response_pk_encoded
    );

    let (_status, _headers, body) = get! (
        app: app,
        path: path.clone(),
        headers: headers
        // response_type: DeliberationSpaceResponse
    );

    eprintln!("response_answer: {:?}", body);

    // let meta = &body.answers;

    // assert_eq!(
    //     meta.len(),
    //     2,
    //     "Failed to match retrieved response answer length"
    // );

    // assert!(
    //     matches!(&meta[0], SurveyAnswer::SingleChoice { answer: Some(1) }),
    //     "Failed to match updated single choice answer"
    // );
    // assert!(
    //     matches!(
    //         &meta[1],
    //         SurveyAnswer::MultipleChoice { answer: Some(v) } if v.as_slice() == &[1]
    //     ),
    //     "Failed to match updated multiple choice answer"
    // );
}
