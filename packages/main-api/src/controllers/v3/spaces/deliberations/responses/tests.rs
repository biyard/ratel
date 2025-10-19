// use bdk::prelude::axum::AxumRouter;

// use crate::controllers::v3::posts::create_post::CreatePostResponse;
// use crate::controllers::v3::spaces::CreateSpaceResponse;
// use crate::controllers::v3::spaces::deliberations::update_deliberation_poll::UpdateDeliberationPollResponse;
// use crate::tests::create_app_state;
// use crate::types::{Partition, SpaceType};
// use crate::{
//     controllers::v3::spaces::deliberations::responses::create_response_answer::CreateDeliberationResponse,
//     models::space::{DeliberationSpaceResponse, SurveyCreateRequest},
//     tests::v3_setup::{TestContextV3, setup_v3},
//     types::{ChoiceQuestion, LinearScaleQuestion, SpaceVisibility, SurveyQuestion, SurveyStatus},
// };

// use crate::types::SurveyAnswer;
// use crate::*;

// #[tokio::test]
// async fn test_create_response_answer_handler() {
//     let TestContextV3 {
//         app,
//         test_user: (_user, headers),
//         ..
//     } = setup_v3().await;

//     let app_state = create_app_state();
//     let _cli = &app_state.dynamo.client;

//     let space_pk = bootstrap_deliberation_space(&app, headers.clone()).await;

//     let now = chrono::Utc::now().timestamp();
//     let space_pk_encoded = space_pk.to_string().replace('#', "%23");
//     let path = format!("/v3/spaces/{}/deliberation/poll", space_pk_encoded);

//     let (status, _headers, body) = patch! {
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

//     let survey_pk = body.clone().surveys.pk;

//     let space_pk_encoded = space_pk.to_string().replace('#', "%23");
//     let path = format!("/v3/spaces/{}/deliberation/responses", space_pk_encoded);

//     let (status, _headers, body) = post! (
//         app: app,
//         path: path.clone(),
//         headers: headers.clone(),
//         body: {
//             "survey_pk": survey_pk,
//             "survey_type": crate::types::SurveyType::Survey,
//             "answers": vec![
//                 SurveyAnswer::SingleChoice { answer: Some(1) },
//                 SurveyAnswer::MultipleChoice {
//                     answer: Some(vec![1]),
//                 },
//             ],
//         },
//         response_type: CreateDeliberationResponse
//     );

//     assert_eq!(status, 200);

//     let meta = &body.metadata;

//     assert_eq!(
//         meta.surveys.user_responses.len(),
//         1,
//         "Failed to match user response answer length"
//     );
//     assert_eq!(
//         meta.surveys.responses.len(),
//         1,
//         "Failed to match response answer length"
//     );
// }

// #[tokio::test]
// async fn test_get_response_answer_handler() {
//     let TestContextV3 {
//         app,
//         test_user: (_user, headers),
//         ..
//     } = setup_v3().await;

//     let app_state = create_app_state();
//     let _cli = &app_state.dynamo.client;

//     let space_pk = bootstrap_deliberation_space(&app, headers.clone()).await;

//     let now = chrono::Utc::now().timestamp();
//     let space_pk_encoded = space_pk.to_string().replace('#', "%23");
//     let path = format!("/v3/spaces/{}/deliberation/poll", space_pk_encoded);

//     let (status, _headers, body) = patch! {
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

//     let survey_pk = body.surveys.pk;

//     tracing::debug!("responses: {:?}", body.surveys.responses);
//     let path = format!("/v3/spaces/{}/deliberation/responses", space_pk_encoded);

//     let (status, _headers, body) = post! (
//         app: app,
//         path: path.clone(),
//         headers: headers.clone(),
//         body: {
//             "survey_pk": survey_pk,
//             "survey_type": crate::types::SurveyType::Survey,
//             "answers": vec![
//                 SurveyAnswer::SingleChoice { answer: Some(1) },
//                 SurveyAnswer::MultipleChoice {
//                     answer: Some(vec![1]),
//                 },
//             ],
//         },
//         response_type: CreateDeliberationResponse
//     );

//     assert_eq!(status, 200);

//     assert_eq!(
//         body.metadata.surveys.user_responses.len(),
//         1,
//         "Failed to match user response answer length"
//     );
//     assert_eq!(
//         body.metadata.surveys.responses.len(),
//         1,
//         "Failed to match response answer length"
//     );

//     let response_pk = body.metadata.surveys.user_responses[0].pk.clone();

//     let response_pk_encoded = response_pk.to_string().replace('#', "%23");
//     let path = format!(
//         "/v3/spaces/{}/deliberation/responses/{}",
//         space_pk_encoded, response_pk_encoded
//     );

//     let (_status, _headers, body) = get! (
//         app: app,
//         path: path.clone(),
//         headers: headers,
//         response_type: DeliberationSpaceResponse
//     );

//     let meta = &body.answers;

//     assert_eq!(
//         meta.len(),
//         2,
//         "Failed to match retrieved response answer length"
//     );

//     assert!(
//         matches!(&meta[0], SurveyAnswer::SingleChoice { answer: Some(1) }),
//         "Failed to match updated single choice answer"
//     );
//     assert!(
//         matches!(
//             &meta[1],
//             SurveyAnswer::MultipleChoice { answer: Some(v) } if v.as_slice() == &[1]
//         ),
//         "Failed to match updated multiple choice answer"
//     );
// }

// async fn bootstrap_deliberation_space(
//     app: &AxumRouter,
//     headers: axum::http::HeaderMap,
// ) -> Partition {
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

//     space.space_pk
// }
