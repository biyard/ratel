use crate::{
    controllers::v3::spaces::deliberations::{
        create_deliberation::{CreateDeliberationRequest, create_deliberation_handler},
        responses::{
            create_response_answer::DeliberationResponsePath,
            get_response_answer::{DeliberationResponseByIdPath, get_response_answer_handler},
        },
        update_deliberation::{
            DeliberationPath, UpdateDeliberationRequest, update_deliberation_handler,
        },
    },
    models::space::{DiscussionCreateRequest, SurveyCreateRequest},
    tests::{create_app_state, create_test_user, get_auth},
    types::{ChoiceQuestion, LinearScaleQuestion, SurveyQuestion, SurveyStatus},
};
use dto::{
    File,
    by_axum::axum::{
        Json,
        extract::{Extension, Path, State},
    },
};

use crate::controllers::v3::spaces::deliberations::responses::create_response_answer::{
    CreateResponseAnswerRequest, create_response_answer_handler,
};
use crate::types::SurveyAnswer;

#[tokio::test]
async fn test_create_response_answer_handler() {
    let app_state = create_app_state();
    let cli = app_state.dynamo.client.clone();
    let user = create_test_user(&cli).await;
    let auth = get_auth(&user.clone());
    let uid = uuid::Uuid::new_v4().to_string();
    let create_deliberation = create_deliberation_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Json(CreateDeliberationRequest { feed_id: uid }),
    )
    .await
    .unwrap();

    let space_pk = create_deliberation.0.metadata.deliberation.pk.clone();
    let now = chrono::Utc::now().timestamp();

    let update_deliberation = update_deliberation_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(DeliberationPath {
            space_pk: space_pk.to_string(),
        }),
        Json(UpdateDeliberationRequest {
            title: Some("deliberation title".to_string()),
            html_contents: Some("<div>deliberation description</div>".to_string()),
            files: vec![File {
                name: "deliberation summary file title".to_string(),
                size: "15KB".to_string(),
                ext: dto::FileExtension::PDF,
                url: None,
            }],
            discussions: vec![DiscussionCreateRequest {
                discussion_pk: None,
                started_at: now,
                ended_at: now,
                name: "discussion title".to_string(),
                description: "discussion description".to_string(),
                user_ids: vec![],
            }],
            elearning_files: vec![File {
                name: "deliberation elearning file title".to_string(),
                size: "15KB".to_string(),
                ext: dto::FileExtension::PDF,
                url: None,
            }],
            surveys: vec![SurveyCreateRequest {
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
            recommendation_html_contents: Some(
                "<div>deliberation recommendation description</div>".to_string(),
            ),
            recommendation_files: vec![File {
                name: "deliberation recommendation file title".to_string(),
                size: "15KB".to_string(),
                ext: dto::FileExtension::PDF,
                url: None,
            }],
        }),
    )
    .await
    .unwrap();

    let space_pk = create_deliberation.0.metadata.deliberation.pk;
    let survey_pk = update_deliberation.0.surveys.pk;

    let create_response_answer = create_response_answer_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(DeliberationResponsePath {
            space_pk: space_pk.to_string(),
        }),
        Json(CreateResponseAnswerRequest {
            survey_pk: survey_pk.to_string(),
            survey_type: crate::types::SurveyType::Survey,
            answers: vec![
                SurveyAnswer::SingleChoice { answer: Some(1) },
                SurveyAnswer::MultipleChoice {
                    answer: Some(vec![1]),
                },
            ],
        }),
    )
    .await;

    assert!(
        create_response_answer.is_ok(),
        "Failed to create response answer {:?}",
        create_response_answer.err()
    );

    let resp = create_response_answer.as_ref().expect("request failed"); // &Json<...>
    let meta = &resp.0.metadata;

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
    let app_state = create_app_state();
    let cli = app_state.dynamo.client.clone();
    let user = create_test_user(&cli).await;
    let auth = get_auth(&user.clone());
    let uid = uuid::Uuid::new_v4().to_string();
    let create_deliberation = create_deliberation_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Json(CreateDeliberationRequest { feed_id: uid }),
    )
    .await
    .unwrap();

    let space_pk = create_deliberation.0.metadata.deliberation.pk.clone();
    let now = chrono::Utc::now().timestamp();

    let update_deliberation = update_deliberation_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(DeliberationPath {
            space_pk: space_pk.to_string(),
        }),
        Json(UpdateDeliberationRequest {
            title: Some("deliberation title".to_string()),
            html_contents: Some("<div>deliberation description</div>".to_string()),
            files: vec![File {
                name: "deliberation summary file title".to_string(),
                size: "15KB".to_string(),
                ext: dto::FileExtension::PDF,
                url: None,
            }],
            discussions: vec![DiscussionCreateRequest {
                discussion_pk: None,
                started_at: now,
                ended_at: now,
                name: "discussion title".to_string(),
                description: "discussion description".to_string(),
                user_ids: vec![],
            }],
            elearning_files: vec![File {
                name: "deliberation elearning file title".to_string(),
                size: "15KB".to_string(),
                ext: dto::FileExtension::PDF,
                url: None,
            }],
            surveys: vec![SurveyCreateRequest {
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
                ],
            }],
            recommendation_html_contents: Some(
                "<div>deliberation recommendation description</div>".to_string(),
            ),
            recommendation_files: vec![File {
                name: "deliberation recommendation file title".to_string(),
                size: "15KB".to_string(),
                ext: dto::FileExtension::PDF,
                url: None,
            }],
        }),
    )
    .await
    .unwrap();

    let space_pk = create_deliberation.0.metadata.deliberation.pk;
    let survey_pk = update_deliberation.0.surveys.pk;

    let create_response_answer = create_response_answer_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(DeliberationResponsePath {
            space_pk: space_pk.to_string(),
        }),
        Json(CreateResponseAnswerRequest {
            survey_pk: survey_pk.to_string(),
            survey_type: crate::types::SurveyType::Survey,
            answers: vec![
                SurveyAnswer::SingleChoice { answer: Some(1) },
                SurveyAnswer::MultipleChoice {
                    answer: Some(vec![1]),
                },
            ],
        }),
    )
    .await;

    assert!(
        create_response_answer.is_ok(),
        "Failed to create response answer {:?}",
        create_response_answer.err()
    );

    let resp = create_response_answer.as_ref().expect("request failed"); // &Json<...>
    let meta = &resp.0.metadata;

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

    let response_answer = get_response_answer_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(DeliberationResponseByIdPath {
            deliberation_pk: space_pk.to_string(),
            response_pk: meta.surveys.user_responses[0].pk.to_string(),
        }),
    )
    .await;

    eprintln!("response_answer: {:?}", response_answer);

    let resp = response_answer.as_ref().expect("request failed"); // &Json<...>
    let meta = &resp.0.answers;

    assert_eq!(
        meta.len(),
        2,
        "Failed to match retrieved response answer length"
    );

    assert!(
        matches!(&meta[0], SurveyAnswer::SingleChoice { answer: Some(1) }),
        "Failed to match updated single choice answer"
    );
    assert!(
        matches!(
            &meta[1],
            SurveyAnswer::MultipleChoice { answer: Some(v) } if v.as_slice() == &[1]
        ),
        "Failed to match updated multiple choice answer"
    );
}
