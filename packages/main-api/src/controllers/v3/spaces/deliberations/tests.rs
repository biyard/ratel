// use crate::controllers::v3::posts::create_post::CreatePostResponse;
// use crate::controllers::v3::spaces::CreateSpaceResponse;
// use crate::controllers::v3::spaces::deliberations::get_deliberation_deliberation::GetDeliberationDeliberationResponse;
// use crate::controllers::v3::spaces::deliberations::update_deliberation_deliberation::UpdateDeliberationDeliberationResponse;
// use crate::controllers::v3::spaces::deliberations::update_deliberation_recommendation::UpdateDeliberationRecommendationResponse;
// use crate::controllers::v3::spaces::deliberations::update_deliberation_summary::UpdateDeliberationSummaryResponse;
// use crate::models::DeliberationContentResponse;
// use crate::types::{File, Partition, SpaceType};
// use crate::*;
// use crate::{
//     controllers::v3::spaces::deliberations::delete_deliberation::DeleteDeliberationResponse,
//     models::space::{DeliberationDetailResponse, DiscussionCreateRequest},
//     tests::{
//         create_app_state, create_test_user, get_auth,
//         v3_setup::{TestContextV3, setup_v3},
//     },
//     types::{ChoiceQuestion, LinearScaleQuestion, SpaceVisibility},
// };
// use axum::AxumRouter;

// struct CreatedDeliberationSpace {
//     space_pk: Partition,
// }

// #[tokio::test]
// async fn test_update_summary_handler() {
//     let TestContextV3 {
//         app,
//         test_user: (_user, headers),
//         ..
//     } = setup_v3().await;

//     let app_state = create_app_state();
//     let _cli = &app_state.dynamo.client;

//     let CreatedDeliberationSpace { space_pk, .. } =
//         bootstrap_deliberation_space(&app, headers.clone()).await;

//     let now = chrono::Utc::now().timestamp();
//     let space_pk_encoded = space_pk.to_string().replace('#', "%23");
//     let path = format!("/v3/spaces/{}/deliberation/summary", space_pk_encoded);

//     let (status, _headers, _body) = patch! {
//         app: app,
//         path: path.clone(),
//         headers: headers.clone(),
//         body: {
//             "title": Some("deliberation title".to_string()),
//             "html_contents": Some("<div>deliberation description</div>".to_string()),
//             "visibility": SpaceVisibility::Public,
//             "started_at": now,
//             "ended_at": now + 86400,
//             "files": vec![File {
//                 name: "deliberation summary file title".to_string(),
//                 size: "15KB".to_string(),
//                 ext: crate::types::FileExtension::PDF,
//                 url: None,
//             }],
//         },
//         response_type: UpdateDeliberationSummaryResponse
//     };

//     assert_eq!(status, 200);

//     let (status, _headers, body) = get! {
//         app: app,
//         path: &path,
//         headers: headers.clone(),
//         response_type: DeliberationContentResponse
//     };

//     assert_eq!(status, 200);

//     assert_eq!(body.files.len(), 1);
//     assert_eq!(body.html_contents, "<div>deliberation description</div>");

//     let (status, _headers, _body) = patch! {
//         app: app,
//         path: path.clone(),
//         headers: headers.clone(),
//         body: {
//             "title": Some("deliberation title".to_string()),
//             "html_contents": Some("<div>updated deliberation description</div>".to_string()),
//             "visibility": SpaceVisibility::Public,
//             "started_at": now,
//             "ended_at": now + 86400,
//             "files": vec![File {
//                 name: "deliberation summary file title".to_string(),
//                 size: "15KB".to_string(),
//                 ext: crate::types::FileExtension::PDF,
//                 url: None,
//             }],
//         },
//         response_type: UpdateDeliberationSummaryResponse
//     };

//     assert_eq!(status, 200);

//     let (status, _headers, body) = get! {
//         app: app,
//         path: &path,
//         headers: headers.clone(),
//         response_type: DeliberationContentResponse
//     };

//     assert_eq!(status, 200);

//     assert_eq!(
//         body.html_contents,
//         "<div>updated deliberation description</div>"
//     );
// }

// #[tokio::test]
// async fn test_update_deliberation_handler() {
//     let TestContextV3 {
//         app,
//         test_user: (_user, headers),
//         ..
//     } = setup_v3().await;

//     let app_state = create_app_state();
//     let cli = &app_state.dynamo.client;

//     let CreatedDeliberationSpace { space_pk, .. } =
//         bootstrap_deliberation_space(&app, headers.clone()).await;

//     // create user
//     let team_1 = create_test_user(&cli).await.pk;
//     let team_2 = create_test_user(&cli).await.pk;

//     let users = vec![team_1.clone(), team_2.clone()];

//     let now = chrono::Utc::now().timestamp();
//     let space_pk_encoded = space_pk.to_string().replace('#', "%23");
//     let path = format!("/v3/spaces/{}/deliberation/deliberation", space_pk_encoded);

//     let (status, _headers, body) = patch! {
//         app: app,
//         path: path.clone(),
//         headers: headers.clone(),
//         body: {
//             "html_contents": Some("<div>deliberation description</div>".to_string()),
//             "visibility": SpaceVisibility::Public,
//             "started_at": now,
//             "ended_at": now + 86400,
//             "discussions": vec![DiscussionCreateRequest {
//                 discussion_pk: None,
//                 started_at: now,
//                 ended_at: now,
//                 name: "discussion title".to_string(),
//                 description: "discussion description".to_string(),
//                 user_ids: users.clone(),
//             }],
//             "elearning_files": vec![File {
//                 name: "deliberation elearning file title".to_string(),
//                 size: "15KB".to_string(),
//                 ext: crate::types::FileExtension::PDF,
//                 url: None,
//             }],
//         },
//         response_type: UpdateDeliberationDeliberationResponse
//     };

//     tracing::debug!("deliberation body: {:?}", body);

//     assert_eq!(status, 200);

//     let (status, _headers, body) = get! {
//         app: app,
//         path: &path,
//         headers: headers.clone(),
//         response_type: GetDeliberationDeliberationResponse
//     };

//     assert_eq!(status, 200);

//     assert_eq!(
//         body.elearnings.files[0].name,
//         "deliberation elearning file title".to_string()
//     );
//     assert_eq!(body.discussions.len(), 1);
//     assert_eq!(body.discussions[0].name, "discussion title".to_string());

//     let discussion_id = body.discussions[0].pk.clone();

//     let (status, _headers, _body) = patch! {
//         app: app,
//         path: path.clone(),
//         headers: headers.clone(),
//         body: {
//             "html_contents": Some("<div>deliberation description</div>".to_string()),
//             "visibility": SpaceVisibility::Public,
//             "started_at": now,
//             "ended_at": now + 86400,
//             "discussions": vec![DiscussionCreateRequest {
//                 discussion_pk: Some(discussion_id.to_string()),
//                 started_at: now,
//                 ended_at: now,
//                 name: "updated discussion title".to_string(),
//                 description: "discussion description".to_string(),
//                 user_ids: users.clone(),
//             }],
//             "elearning_files": vec![File {
//                 name: "updated deliberation elearning file title".to_string(),
//                 size: "15KB".to_string(),
//                 ext: crate::types::FileExtension::PDF,
//                 url: None,
//             }],
//         },
//         response_type: UpdateDeliberationDeliberationResponse
//     };

//     assert_eq!(status, 200);

//     let (status, _headers, body) = get! {
//         app: app,
//         path: &path,
//         headers: headers.clone(),
//         response_type: GetDeliberationDeliberationResponse
//     };

//     assert_eq!(status, 200);

//     assert_eq!(
//         body.elearnings.files[0].name,
//         "updated deliberation elearning file title".to_string()
//     );
//     assert_eq!(body.discussions.len(), 1);
//     assert_eq!(
//         body.discussions[0].name,
//         "updated discussion title".to_string()
//     );
// }

// #[tokio::test]
// async fn test_update_poll_handler() {
//     let TestContextV3 {
//         app,
//         test_user: (_user, headers),
//         ..
//     } = setup_v3().await;

//     let app_state = create_app_state();
//     let _cli = &app_state.dynamo.client;

//     let CreatedDeliberationSpace { space_pk, .. } =
//         bootstrap_deliberation_space(&app, headers.clone()).await;

//     let now = chrono::Utc::now().timestamp();
//     let space_pk_encoded = space_pk.to_string().replace('#', "%23");
//     let path = format!("/v3/spaces/{}/deliberation/poll", space_pk_encoded);

//     let (status, _headers, _body) = patch! {
//         app: app,
//         path: path.clone(),
//         headers: headers.clone(),
//         body: {
//             "html_contents": Some("<div>deliberation description</div>".to_string()),
//             "visibility": SpaceVisibility::Public,
//             "started_at": now,
//             "ended_at": now + 86400,
//             "surveys": vec![SurveyCreateRequest {
//                 survey_pk: None,
//                 started_at: now,
//                 ended_at: now + 10_000,
//                 status: SurveyStatus::Ready,
//                 questions: vec![
//                     SurveyQuestion::SingleChoice(ChoiceQuestion {
//                         title: "How did you hear about us?".into(),
//                         description: Some("Pick one".into()),
//                         image_url: None,
//                         options: vec![
//                             "Search".into(),
//                             "Friend".into(),
//                             "Social".into(),
//                             "Other".into(),
//                         ],
//                         is_required: Some(true),
//                     }),
//                     SurveyQuestion::MultipleChoice(ChoiceQuestion {
//                         title: "Which topics interest you?".into(),
//                         description: None,
//                         image_url: None,
//                         options: vec![
//                             "DeFi".into(),
//                             "NFTs".into(),
//                             "Governance".into(),
//                             "Education".into(),
//                         ],
//                         is_required: Some(false),
//                     }),
//                     SurveyQuestion::LinearScale(LinearScaleQuestion {
//                         title: "Rate your onboarding experience".into(),
//                         description: Some("1 = Poor, 5 = Excellent".into()),
//                         image_url: None,
//                         min_value: 1,
//                         max_value: 5,
//                         min_label: "Poor".into(),
//                         max_label: "Excellent".into(),
//                         is_required: Some(true),
//                     }),
//                 ],
//             }],
//         },
//         response_type: UpdateDeliberationPollResponse
//     };

//     assert_eq!(status, 200);

//     let (status, _headers, body) = get! {
//         app: app,
//         path: &path,
//         headers: headers.clone(),
//         response_type: DeliberationSurveyResponse
//     };

//     assert_eq!(status, 200);

//     assert_eq!(body.questions.len(), 3);
//     assert_eq!(body.started_at, now);
//     assert_eq!(body.ended_at, now + 10_000);

//     let survey_id = body.pk.clone();

//     let (status, _headers, _body) = patch! {
//         app: app,
//         path: path.clone(),
//         headers: headers.clone(),
//         body: {
//             "html_contents": Some("<div>deliberation description</div>".to_string()),
//             "visibility": SpaceVisibility::Public,
//             "started_at": now,
//             "ended_at": now + 86400,
//             "surveys": vec![SurveyCreateRequest {
//                 survey_pk: Some(survey_id.to_string()),
//                 started_at: now,
//                 ended_at: now + 20_000,
//                 status: SurveyStatus::Ready,
//                 questions: vec![
//                     SurveyQuestion::SingleChoice(ChoiceQuestion {
//                         title: "How did you hear about us 11?".into(),
//                         description: Some("Pick one".into()),
//                         image_url: None,
//                         options: vec![
//                             "Search".into(),
//                             "Friend".into(),
//                             "Social".into(),
//                             "Other".into(),
//                         ],
//                         is_required: Some(true),
//                     }),
//                     SurveyQuestion::MultipleChoice(ChoiceQuestion {
//                         title: "Which topics interest you 22?".into(),
//                         description: None,
//                         image_url: None,
//                         options: vec![
//                             "DeFi".into(),
//                             "NFTs".into(),
//                             "Governance".into(),
//                             "Education".into(),
//                         ],
//                         is_required: Some(false),
//                     }),
//                 ],
//             }],
//         },
//         response_type: UpdateDeliberationPollResponse
//     };

//     assert_eq!(status, 200);

//     let (status, _headers, body) = get! {
//         app: app,
//         path: &path,
//         headers: headers.clone(),
//         response_type: DeliberationSurveyResponse
//     };

//     assert_eq!(status, 200);

//     assert_eq!(body.questions.len(), 2);
//     assert_eq!(body.started_at, now);
//     assert_eq!(body.ended_at, now + 20_000);
// }

// #[tokio::test]
// async fn test_update_recommendation_handler() {
//     let TestContextV3 {
//         app,
//         test_user: (_user, headers),
//         ..
//     } = setup_v3().await;

//     let app_state = create_app_state();
//     let _cli = &app_state.dynamo.client;

//     let CreatedDeliberationSpace { space_pk, .. } =
//         bootstrap_deliberation_space(&app, headers.clone()).await;

//     let now = chrono::Utc::now().timestamp();
//     let space_pk_encoded = space_pk.to_string().replace('#', "%23");
//     let path = format!(
//         "/v3/spaces/{}/deliberation/recommendation",
//         space_pk_encoded
//     );

//     let (status, _headers, _body) = patch! {
//         app: app,
//         path: path.clone(),
//         headers: headers.clone(),
//         body: {
//             "title": Some("deliberation title".to_string()),
//             "visibility": SpaceVisibility::Public,
//             "started_at": now,
//             "ended_at": now + 86400,
//             "recommendation_html_contents": Some(
//                 "<div>deliberation recommendation description</div>".to_string(),
//             ),
//             "recommendation_files": vec![File {
//                 name: "deliberation recommendation file title".to_string(),
//                 size: "15KB".to_string(),
//                 ext: crate::types::FileExtension::PDF,
//                 url: None,
//             }],
//         },
//         response_type: UpdateDeliberationRecommendationResponse
//     };

//     assert_eq!(status, 200);

//     let (status, _headers, body) = get! {
//         app: app,
//         path: &path,
//         headers: headers.clone(),
//         response_type: DeliberationContentResponse
//     };

//     assert_eq!(status, 200);

//     assert_eq!(body.files.len(), 1);
//     assert_eq!(
//         body.html_contents,
//         "<div>deliberation recommendation description</div>"
//     );

//     let (status, _headers, _body) = patch! {
//         app: app,
//         path: path.clone(),
//         headers: headers.clone(),
//         body: {
//             "title": Some("deliberation title".to_string()),
//             "visibility": SpaceVisibility::Public,
//             "started_at": now,
//             "ended_at": now + 86400,
//             "recommendation_html_contents": Some(
//                 "<div>update deliberation recommendation description</div>".to_string(),
//             ),
//             "recommendation_files": vec![File {
//                 name: "update deliberation recommendation file title".to_string(),
//                 size: "15KB".to_string(),
//                 ext: crate::types::FileExtension::PDF,
//                 url: None,
//             }],
//         },
//         response_type: UpdateDeliberationRecommendationResponse
//     };

//     assert_eq!(status, 200);

//     let (status, _headers, body) = get! {
//         app: app,
//         path: &path,
//         headers: headers.clone(),
//         response_type: DeliberationContentResponse
//     };

//     assert_eq!(status, 200);

//     assert_eq!(body.files.len(), 1);
//     assert_eq!(
//         body.html_contents,
//         "<div>update deliberation recommendation description</div>"
//     );
// }

// #[tokio::test]
// async fn test_delete_space_handler() {
//     let TestContextV3 {
//         app,
//         test_user: (user, headers),
//         ..
//     } = setup_v3().await;

//     //FIXME: fix by session and one test code
//     let app_state = create_app_state();
//     let cli = &app_state.dynamo.client;
//     let _auth = get_auth(&user);

//     let (_status, _headers, post) = post! {
//         app: app,
//         path: "/v3/posts",
//         headers: headers.clone(),
//         response_type: CreatePostResponse
//     };

//     let feed_pk = post.post_pk.clone();

//     // SPACE

//     let (status, _headers, body) = post! {
//         app: app,
//         path: "/v3/spaces",
//         headers: headers.clone(),
//         body: {
//             "space_type": SpaceType::Deliberation,
//             "post_pk": feed_pk
//         },
//         response_type: CreateSpaceResponse
//     };

//     assert_eq!(status, 200);

//     // create user
//     let team_1 = create_test_user(&cli).await.pk;
//     let team_2 = create_test_user(&cli).await.pk;

//     let users = vec![team_1.clone(), team_2];

//     let now = chrono::Utc::now().timestamp();
//     let space_pk = body.space_pk;
//     let space_pk_encoded = space_pk.to_string().replace('#', "%23");
//     let path = format!("/v3/spaces/{}/deliberation", space_pk_encoded);

//     let (status, _headers, _body) = post! {
//         app: app,
//         path: path.clone(),
//         headers: headers.clone(),
//         body: {
//             "title": Some("deliberation title".to_string()),
//             "html_contents": Some("<div>deliberation description</div>".to_string()),
//             "files": vec![File {
//                 name: "deliberation summary file title".to_string(),
//                 size: "15KB".to_string(),
//                 ext: crate::types::FileExtension::PDF,
//                 url: None,
//             }],
//             "visibility": SpaceVisibility::Public,
//             "started_at": now,
//             "ended_at": now + 86400,
//             "discussions": vec![DiscussionCreateRequest {
//                 discussion_pk: None,
//                 started_at: now,
//                 ended_at: now,
//                 name: "discussion title".to_string(),
//                 description: "discussion description".to_string(),
//                 user_ids: users,
//             }],
//             "elearning_files": vec![File {
//                 name: "deliberation elearning file title".to_string(),
//                 size: "15KB".to_string(),
//                 ext: crate::types::FileExtension::PDF,
//                 url: None,
//             }],
//             "surveys": vec![SurveyCreateRequest {
//                 survey_pk: None,
//                 started_at: now,
//                 ended_at: now + 10_000,
//                 status: SurveyStatus::Ready,
//                 questions: vec![
//                     SurveyQuestion::SingleChoice(ChoiceQuestion {
//                         title: "How did you hear about us?".into(),
//                         description: Some("Pick one".into()),
//                         image_url: None,
//                         options: vec![
//                             "Search".into(),
//                             "Friend".into(),
//                             "Social".into(),
//                             "Other".into(),
//                         ],
//                         is_required: Some(true),
//                     }),
//                     SurveyQuestion::MultipleChoice(ChoiceQuestion {
//                         title: "Which topics interest you?".into(),
//                         description: None,
//                         image_url: None,
//                         options: vec![
//                             "DeFi".into(),
//                             "NFTs".into(),
//                             "Governance".into(),
//                             "Education".into(),
//                         ],
//                         is_required: Some(false),
//                     }),
//                     SurveyQuestion::LinearScale(LinearScaleQuestion {
//                         title: "Rate your onboarding experience".into(),
//                         description: Some("1 = Poor, 5 = Excellent".into()),
//                         image_url: None,
//                         min_value: 1,
//                         max_value: 5,
//                         min_label: "Poor".into(),
//                         max_label: "Excellent".into(),
//                         is_required: Some(true),
//                     }),
//                 ],
//             }],
//             "recommendation_html_contents": Some(
//                 "<div>deliberation recommendation description</div>".to_string(),
//             ),
//             "recommendation_files": vec![File {
//                 name: "deliberation recommendation file title".to_string(),
//                 size: "15KB".to_string(),
//                 ext: crate::types::FileExtension::PDF,
//                 url: None,
//             }],
//         },
//         response_type: DeliberationDetailResponse
//     };

//     assert_eq!(status, 200);

//     let space_pk_encoded = space_pk.to_string().replace('#', "%23");
//     let path = format!("/v3/spaces/{}/deliberation", space_pk_encoded);

//     let (status, _headers, _body) = delete! {
//         app: app,
//         path: path.clone(),
//         headers: headers.clone(),
//         body: {},
//         response_type: DeleteDeliberationResponse
//     };

//     assert_eq!(status, 200);
// }

// async fn bootstrap_deliberation_space(
//     app: &AxumRouter,
//     headers: axum::http::HeaderMap,
// ) -> CreatedDeliberationSpace {
//     let (_status, _headers, post) = post! {
//         app: app,
//         path: "/v3/posts",
//         headers: headers.clone(),
//         response_type: CreatePostResponse
//     };

//     let feed_pk = post.post_pk.clone();

//     let (_status, _headers, space) = post! {
//         app: app,
//         path: "/v3/spaces",
//         headers: headers.clone(),
//         body: {
//             "space_type": SpaceType::Deliberation,
//             "post_pk": feed_pk
//         },
//         response_type: CreateSpaceResponse
//     };

//     CreatedDeliberationSpace {
//         space_pk: space.space_pk,
//     }
// }
