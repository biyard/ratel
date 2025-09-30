use crate::{
    controllers::v3::{
        posts::create_post::{CreatePostRequest, create_post_handler},
        spaces::deliberations::{
            create_deliberation::CreateDeliberationResponse,
            delete_deliberation::DeleteDeliberationResponse,
        },
    },
    get,
    models::space::{DeliberationDetailResponse, DiscussionCreateRequest, SurveyCreateRequest},
    post,
    tests::{
        create_app_state, create_test_user, get_auth,
        v3_setup::{TestContextV3, setup_v3},
    },
    types::{
        ChoiceQuestion, LinearScaleQuestion, Partition, SpaceVisibility, SurveyQuestion,
        SurveyStatus,
    },
};
use dto::{
    File,
    axum::{Extension, Json, extract::State},
};

#[tokio::test]
async fn test_create_space_handler() {
    let TestContextV3 {
        app,
        test_user: (user, headers),
        ..
    } = setup_v3().await;

    //FIXME: fix by session and one test code
    let app_state = create_app_state();
    let auth = get_auth(&user);

    let post = create_post_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Json(CreatePostRequest { team_pk: None }),
    )
    .await;
    assert!(post.is_ok(), "Failed to create post: {:?}", post);

    let feed_pk = post.unwrap().post_pk.clone();

    // SPACE
    let (status, _headers, _body) = post! {
        app: app,
        path: "/v3/spaces/deliberation",
        headers: headers,
        body: {
            "feed_pk": feed_pk
        },
        response_type: CreateDeliberationResponse
    };

    assert_eq!(status, 200);
}

#[tokio::test]
async fn test_update_space_handler() {
    let TestContextV3 {
        app,
        test_user: (user, headers),
        ..
    } = setup_v3().await;

    //FIXME: fix by session and one test code
    let app_state = create_app_state();
    let cli = &app_state.dynamo.client;
    let auth = get_auth(&user);

    let post = create_post_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Json(CreatePostRequest { team_pk: None }),
    )
    .await;
    assert!(post.is_ok(), "Failed to create post: {:?}", post);

    let feed_pk = post.unwrap().post_pk.clone();

    // SPACE

    let (_status, _headers, body) = post! {
        app: app,
        path: "/v3/spaces/deliberation",
        headers: headers.clone(),
        body: {
            "feed_pk": feed_pk
        },
        response_type: CreateDeliberationResponse
    };

    let space_pk = body.metadata.deliberation.pk.clone();

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
    let space_pk = body.metadata.deliberation.pk;
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
                ext: dto::FileExtension::PDF,
                url: None,
            }],
            "discussions": vec![DiscussionCreateRequest {
                discussion_pk: None,
                started_at: now,
                ended_at: now,
                name: "discussion title".to_string(),
                description: "discussion description".to_string(),
                user_ids: users.clone(),
            }],
            "elearning_files": vec![File {
                name: "deliberation elearning file title".to_string(),
                size: "15KB".to_string(),
                ext: dto::FileExtension::PDF,
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
                ext: dto::FileExtension::PDF,
                url: None,
            }],
        },
        response_type: DeliberationDetailResponse,
    };

    assert_eq!(status, 200);

    assert_eq!(
        body.summary.html_contents,
        "<div>deliberation description</div>".to_string()
    );
    assert_eq!(body.discussions.len(), 1);
    assert_eq!(body.discussions[0].members.len(), 2);
    assert_eq!(
        body.elearnings.files[0].name,
        "deliberation elearning file title".to_string()
    );
    assert_eq!(body.surveys.questions.len(), 3);
    assert_eq!(body.surveys.started_at, now);
    assert_eq!(body.surveys.ended_at, now + 10_000);
    assert_eq!(
        body.recommendation.html_contents,
        "<div>deliberation recommendation description</div>"
    );

    let discussion_id = body.discussions[0].pk.clone();
    let survey_id = body.surveys.pk.clone();

    let updated_users = vec![team_1];

    let (status, _headers, body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "title": Some("deliberation title".to_string()),
            "html_contents": Some("<div>deliberation description 11</div>".to_string()),
            "files": vec![File {
                name: "deliberation summary file title".to_string(),
                size: "15KB".to_string(),
                ext: dto::FileExtension::PDF,
                url: None,
            }],
            "visibility": SpaceVisibility::Public,
            "started_at": now,
            "ended_at": now + 86400,
            "discussions": vec![DiscussionCreateRequest {
                discussion_pk: Some(discussion_id.to_string()),
                started_at: now,
                ended_at: now,
                name: "discussion title".to_string(),
                description: "discussion description".to_string(),
                user_ids: updated_users,
            }],
            "elearning_files": vec![File {
                name: "deliberation elearning update file title".to_string(),
                size: "15KB".to_string(),
                ext: dto::FileExtension::PDF,
                url: None,
            }],
            "surveys": vec![SurveyCreateRequest {
                survey_pk: Some(survey_id.to_string()),
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
            "recommendation_html_contents": Some(
                "<div>deliberation recommendation description 11</div>".to_string(),
            ),
            "recommendation_files": vec![File {
                name: "deliberation recommendation file title 11".to_string(),
                size: "15KB".to_string(),
                ext: dto::FileExtension::PDF,
                url: None,
            }],
        },
        response_type: DeliberationDetailResponse,
    };

    assert_eq!(status, 200);

    assert_eq!(
        body.summary.html_contents,
        "<div>deliberation description 11</div>".to_string()
    );
    assert_eq!(body.discussions.len(), 1);
    assert_eq!(body.discussions[0].members.len(), 1);
    assert_eq!(
        body.elearnings.files[0].name,
        "deliberation elearning update file title".to_string()
    );
    assert_eq!(body.surveys.questions.len(), 2);
    assert_eq!(body.surveys.started_at, now);
    assert_eq!(body.surveys.ended_at, now + 20_000);
    assert_eq!(
        body.recommendation.html_contents,
        "<div>deliberation recommendation description 11</div>"
    );
}

#[tokio::test]
async fn test_delete_space_handler() {
    let TestContextV3 {
        app,
        test_user: (user, headers),
        ..
    } = setup_v3().await;

    //FIXME: fix by session and one test code
    let app_state = create_app_state();
    let cli = &app_state.dynamo.client;
    let auth = get_auth(&user);

    let post = create_post_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Json(CreatePostRequest { team_pk: None }),
    )
    .await;
    assert!(post.is_ok(), "Failed to create post: {:?}", post);

    let feed_pk = post.unwrap().post_pk.clone();

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
    let space_pk = body.metadata.deliberation.pk;
    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/deliberation/{}", space_pk_encoded);

    let (status, _headers, _body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "title": Some("deliberation title".to_string()),
            "html_contents": Some("<div>deliberation description</div>".to_string()),
            "files": vec![File {
                name: "deliberation summary file title".to_string(),
                size: "15KB".to_string(),
                ext: dto::FileExtension::PDF,
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
                user_ids: users,
            }],
            "elearning_files": vec![File {
                name: "deliberation elearning file title".to_string(),
                size: "15KB".to_string(),
                ext: dto::FileExtension::PDF,
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
                ext: dto::FileExtension::PDF,
                url: None,
            }],
        },
        response_type: DeliberationDetailResponse,
    };

    assert_eq!(status, 200);

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/deliberation/{}/delete", space_pk_encoded);

    let (status, _headers, _body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {},
        response_type: DeleteDeliberationResponse
    };

    assert_eq!(status, 200);
}

#[tokio::test]
async fn test_get_space_handler() {
    let TestContextV3 {
        app,
        test_user: (user, headers),
        ..
    } = setup_v3().await;

    //FIXME: fix by session and one test code
    let app_state = create_app_state();
    let auth = get_auth(&user);

    let post = create_post_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Json(CreatePostRequest { team_pk: None }),
    )
    .await;
    assert!(post.is_ok(), "Failed to create post: {:?}", post);

    let feed_pk = post.unwrap().post_pk.clone();

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

    let space_pk = body.metadata.deliberation.pk;
    let space_pk_encoded = space_pk.to_string().replace('#', "%23");

    eprintln!("Created deliberation with space_pk: {}", space_pk_encoded);

    let path = format!("/v3/spaces/deliberation/{}", space_pk_encoded);

    let (status, _headers, body) = get! {
        app: app,
        path: &path,
        headers: headers
    };

    eprintln!("Get deliberation response body: {:?}", body);

    assert_eq!(status, 200);
}
