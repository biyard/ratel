use crate::{
    controllers::v3::spaces::deliberations::{
        create_deliberation::{CreateDeliberationRequest, create_deliberation_handler},
        delete_deliberation::{DeliberationDeletePath, delete_deliberation_handler},
        get_deliberation::{DeliberationGetPath, get_deliberation_handler},
        update_deliberation::{
            DeliberationPath, UpdateDeliberationRequest, update_deliberation_handler,
        },
    },
    models::space::{DiscussionCreateRequest, SurveyCreateRequest},
    tests::{create_app_state, create_test_user, get_auth},
    types::{ChoiceQuestion, LinearScaleQuestion, Partition, SurveyQuestion, SurveyStatus},
};
use dto::{
    File,
    by_axum::axum::{
        Json,
        extract::{Extension, Path, State},
    },
};

#[tokio::test]
async fn test_create_space_handler() {
    let app_state = create_app_state();
    let cli = app_state.dynamo.client.clone();
    let user = create_test_user(&cli).await;
    let auth = get_auth(&user.clone());
    let uid = uuid::Uuid::new_v4().to_string();
    let create_res = create_deliberation_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Json(CreateDeliberationRequest { feed_id: uid }),
    )
    .await;

    assert!(
        create_res.is_ok(),
        "Failed to create deliberation {:?}",
        create_res.err()
    );
}

#[tokio::test]
async fn test_update_space_handler() {
    let app_state = create_app_state();
    let cli = app_state.dynamo.client.clone();
    let user = create_test_user(&cli).await;
    let auth = get_auth(&user.clone());
    let uid = uuid::Uuid::new_v4().to_string();
    let create_res = create_deliberation_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Json(CreateDeliberationRequest { feed_id: uid }),
    )
    .await;

    assert!(
        create_res.is_ok(),
        "Failed to create deliberation {:?}",
        create_res.err()
    );

    let space_pk = create_res.unwrap().0.metadata.deliberation.pk;

    eprintln!("space_pk: {:?}", space_pk);

    // create user
    let team_1 = match create_test_user(&cli).await.pk {
        Partition::User(v) => v,
        _ => "".to_string(),
    };
    let team_2 = match create_test_user(&cli).await.pk {
        Partition::User(v) => v,
        _ => "".to_string(),
    };

    let users = vec![team_1.clone(), team_2];

    let now = chrono::Utc::now().timestamp();

    let update_res = update_deliberation_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(DeliberationPath {
            id: space_pk.clone(),
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
                id: None,
                started_at: now,
                ended_at: now,
                name: "discussion title".to_string(),
                description: "discussion description".to_string(),
                user_ids: users,
            }],
            elearning_files: vec![File {
                name: "deliberation elearning file title".to_string(),
                size: "15KB".to_string(),
                ext: dto::FileExtension::PDF,
                url: None,
            }],
            surveys: vec![SurveyCreateRequest {
                id: None,
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
    .await;

    assert!(
        update_res.is_ok(),
        "Failed to update deliberation {:?}",
        update_res.err()
    );

    let res = update_res.unwrap().0;

    assert_eq!(
        res.summary.html_contents,
        "<div>deliberation description</div>".to_string()
    );
    assert_eq!(res.discussions.len(), 1);
    assert_eq!(res.discussions[0].members.len(), 2);
    assert_eq!(
        res.elearnings.files[0].name,
        "deliberation elearning file title".to_string()
    );
    assert_eq!(res.surveys.questions.len(), 3);
    assert_eq!(res.surveys.started_at, now);
    assert_eq!(res.surveys.ended_at, now + 10_000);
    assert_eq!(
        res.recommendation.html_contents,
        "<div>deliberation recommendation description</div>"
    );

    let discussion_id = res.discussions[0].pk.clone();
    let survey_id = res.surveys.pk.clone();

    let updated_users = vec![team_1];

    let update_res = update_deliberation_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(DeliberationPath {
            id: space_pk.clone(),
        }),
        Json(UpdateDeliberationRequest {
            title: Some("deliberation title".to_string()),
            html_contents: Some("<div>deliberation description 11</div>".to_string()),
            files: vec![File {
                name: "deliberation summary file title".to_string(),
                size: "15KB".to_string(),
                ext: dto::FileExtension::PDF,
                url: None,
            }],
            discussions: vec![DiscussionCreateRequest {
                id: Some(discussion_id),
                started_at: now,
                ended_at: now,
                name: "discussion title".to_string(),
                description: "discussion description".to_string(),
                user_ids: updated_users,
            }],
            elearning_files: vec![File {
                name: "deliberation elearning update file title".to_string(),
                size: "15KB".to_string(),
                ext: dto::FileExtension::PDF,
                url: None,
            }],
            surveys: vec![SurveyCreateRequest {
                id: Some(survey_id),
                started_at: now,
                ended_at: now + 20_000,
                status: SurveyStatus::Ready,
                questions: vec![
                    SurveyQuestion::SingleChoice(ChoiceQuestion {
                        title: "How did you hear about us 11?".into(),
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
                        title: "Which topics interest you 22?".into(),
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
                "<div>deliberation recommendation description 11</div>".to_string(),
            ),
            recommendation_files: vec![File {
                name: "deliberation recommendation file title 11".to_string(),
                size: "15KB".to_string(),
                ext: dto::FileExtension::PDF,
                url: None,
            }],
        }),
    )
    .await;

    assert!(
        update_res.is_ok(),
        "Failed to update deliberation {:?}",
        update_res.err()
    );
    let res = update_res.unwrap().0;
    assert_eq!(
        res.summary.html_contents,
        "<div>deliberation description 11</div>".to_string()
    );
    assert_eq!(res.discussions.len(), 1);
    assert_eq!(res.discussions[0].members.len(), 1);
    assert_eq!(
        res.elearnings.files[0].name,
        "deliberation elearning update file title".to_string()
    );
    assert_eq!(res.surveys.questions.len(), 2);
    assert_eq!(res.surveys.started_at, now);
    assert_eq!(res.surveys.ended_at, now + 20_000);
    assert_eq!(
        res.recommendation.html_contents,
        "<div>deliberation recommendation description 11</div>"
    );
}

#[tokio::test]
async fn test_delete_space_handler() {
    let app_state = create_app_state();
    let cli = app_state.dynamo.client.clone();
    let user = create_test_user(&cli).await;
    let auth = get_auth(&user.clone());
    let uid = uuid::Uuid::new_v4().to_string();
    let create_res = create_deliberation_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Json(CreateDeliberationRequest { feed_id: uid }),
    )
    .await;

    assert!(
        create_res.is_ok(),
        "Failed to create deliberation {:?}",
        create_res.err()
    );

    let space_pk = create_res.unwrap().0.metadata.deliberation.pk;

    // create user
    let team_1 = match create_test_user(&cli).await.pk {
        Partition::User(v) => v,
        _ => "".to_string(),
    };
    let team_2 = match create_test_user(&cli).await.pk {
        Partition::User(v) => v,
        _ => "".to_string(),
    };

    let users = vec![team_1.clone(), team_2];

    let now = chrono::Utc::now().timestamp();

    let res = update_deliberation_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(DeliberationPath {
            id: space_pk.clone(),
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
                id: None,
                started_at: now,
                ended_at: now,
                name: "discussion title".to_string(),
                description: "discussion description".to_string(),
                user_ids: users,
            }],
            elearning_files: vec![File {
                name: "deliberation elearning file title".to_string(),
                size: "15KB".to_string(),
                ext: dto::FileExtension::PDF,
                url: None,
            }],
            surveys: vec![SurveyCreateRequest {
                id: None,
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
    .await;

    assert!(res.is_ok(), "Failed to update deliberation {:?}", res.err());

    let res = delete_deliberation_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(DeliberationDeletePath {
            id: space_pk.clone(),
        }),
    )
    .await;

    assert!(res.is_ok(), "Failed to delete deliberation {:?}", res.err());
}

#[tokio::test]
async fn test_get_space_handler() {
    let app_state = create_app_state();
    let cli = app_state.dynamo.client.clone();
    let user = create_test_user(&cli).await;
    let auth = get_auth(&user.clone());
    let uid = uuid::Uuid::new_v4().to_string();
    let create_res = create_deliberation_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Json(CreateDeliberationRequest { feed_id: uid }),
    )
    .await;

    assert!(
        create_res.is_ok(),
        "Failed to create deliberation {:?}",
        create_res.err()
    );

    let space_pk = create_res.unwrap().0.metadata.deliberation.pk;

    let res = get_deliberation_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(DeliberationGetPath { id: space_pk }),
    )
    .await;

    assert!(res.is_ok(), "Failed to get deliberation {:?}", res.err());
}
