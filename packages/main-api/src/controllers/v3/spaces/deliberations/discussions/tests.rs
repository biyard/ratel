// use bdk::prelude::axum::AxumRouter;

// use crate::controllers::v3::spaces::CreateSpaceResponse;
// use crate::controllers::v3::spaces::deliberations::update_deliberation_deliberation::UpdateDeliberationDeliberationResponse;
// use crate::models::DiscussionCreateRequest;
// use crate::types::{File, SpaceType, SpaceVisibility};
// use crate::*;
// use crate::{
//     controllers::v3::posts::create_post::CreatePostResponse,
//     models::space::DeliberationDiscussionResponse,
//     tests::{
//         create_app_state, create_test_user, get_auth,
//         v3_setup::{TestContextV3, setup_v3},
//     },
//     types::Partition,
// };
// use axum::http::HeaderMap;
// use std::time::{SystemTime, UNIX_EPOCH};

// #[tokio::test]
// async fn test_create_discussion_handler() {
//     let TestContextV3 {
//         app,
//         test_user: (user, headers),
//         ..
//     } = setup_v3().await;
//     let app_state = create_app_state();
//     let _ = get_auth(&user);
//     let (_space_pk, _space_pk_encoded, body) = bootstrap_discussion(
//         &app,
//         headers.clone(),
//         &app_state.dynamo.client,
//         "Test discussion title",
//         "Test discussion description",
//     )
//     .await;
//     assert!(!body.pk.to_string().is_empty());
// }

// #[tokio::test]
// async fn test_start_meeting_handler() {
//     let TestContextV3 {
//         app,
//         test_user: (user, headers),
//         ..
//     } = setup_v3().await;
//     let app_state = create_app_state();
//     let _ = get_auth(&user);
//     let (space_pk, space_pk_encoded, body) = bootstrap_discussion(
//         &app,
//         headers.clone(),
//         &app_state.dynamo.client,
//         "Test discussion title",
//         "Test discussion description",
//     )
//     .await;

//     let discussion_pk_encoded = body.pk.to_string().replace('#', "%23");
//     let path = format!(
//         "/v3/spaces/{}/deliberation/discussions/{}/start-meeting",
//         space_pk_encoded, discussion_pk_encoded
//     );

//     let (status, _headers, body) = post! {
//         app: app,
//         path: path,
//         headers: headers.clone(),
//         body: {},
//         response_type: DeliberationDiscussionResponse
//     };

//     assert_eq!(status, 200);
//     assert!(body.members.len() == 2, "Meeting count is not matched");
//     let _ = space_pk;
// }

// #[tokio::test]
// async fn test_create_participants_handler() {
//     let TestContextV3 {
//         app,
//         test_user: (user, headers),
//         ..
//     } = setup_v3().await;
//     let app_state = create_app_state();
//     let _ = get_auth(&user);
//     let (_space_pk, space_pk_encoded, body) = bootstrap_discussion(
//         &app,
//         headers.clone(),
//         &app_state.dynamo.client,
//         "Test discussion title",
//         "Test discussion description",
//     )
//     .await;

//     let discussion_pk_encoded = body.pk.to_string().replace('#', "%23");
//     let path = format!(
//         "/v3/spaces/{}/deliberation/discussions/{}/participant-meeting",
//         space_pk_encoded, discussion_pk_encoded
//     );

//     let (status, _headers, body) = post! {
//         app: app,
//         path: path,
//         headers: headers.clone(),
//         body: {},
//         response_type: DeliberationDiscussionResponse
//     };

//     assert_eq!(status, 200);
//     assert!(
//         body.participants.len() == 1,
//         "Failed to participant meeting"
//     );
// }

// #[tokio::test]
// async fn test_exit_meeting_handler() {
//     let TestContextV3 {
//         app,
//         test_user: (user, headers),
//         ..
//     } = setup_v3().await;
//     let app_state = create_app_state();
//     let _ = get_auth(&user);
//     let (_space_pk, space_pk_encoded, body) = bootstrap_discussion(
//         &app,
//         headers.clone(),
//         &app_state.dynamo.client,
//         "Test discussion title",
//         "Test discussion description",
//     )
//     .await;

//     let discussion_pk_encoded = body.pk.to_string().replace('#', "%23");

//     let start_meeting_path = format!(
//         "/v3/spaces/{}/deliberation/discussions/{}/start-meeting",
//         space_pk_encoded, discussion_pk_encoded
//     );
//     let (status, _headers, _) = post! {
//         app: app,
//         path: start_meeting_path,
//         headers: headers.clone(),
//         body: {},
//         response_type: DeliberationDiscussionResponse
//     };
//     assert_eq!(status, 200);

//     let participant_path = format!(
//         "/v3/spaces/{}/deliberation/discussions/{}/participant-meeting",
//         space_pk_encoded, discussion_pk_encoded
//     );
//     let (status, _headers, body) = post! {
//         app: app,
//         path: participant_path,
//         headers: headers.clone(),
//         body: {},
//         response_type: DeliberationDiscussionResponse
//     };
//     assert_eq!(status, 200);

//     let me_user_pk: Partition = body
//         .participants
//         .iter()
//         .find_map(|p| match &p.user_pk {
//             Partition::User(v) | Partition::Team(v) if !v.is_empty() => Some(p.user_pk.clone()),
//             _ => None,
//         })
//         .unwrap_or_default();

//     assert!(body.participants.iter().any(|p| p.user_pk == me_user_pk));

//     let exit_path = format!(
//         "/v3/spaces/{}/deliberation/discussions/{}/exit-meeting",
//         space_pk_encoded, discussion_pk_encoded
//     );
//     let (status, _headers, body) = post! {
//         app: app,
//         path: exit_path,
//         headers: headers.clone(),
//         body: {},
//         response_type: DeliberationDiscussionResponse
//     };
//     assert_eq!(status, 200);
//     assert!(body.participants.len() == 0, "not matched participants len");
// }

// #[tokio::test]
// async fn test_start_recording_handler() {
//     let TestContextV3 {
//         app,
//         test_user: (user, headers),
//         ..
//     } = setup_v3().await;
//     let app_state = create_app_state();
//     let _ = get_auth(&user);
//     let (_space_pk, space_pk_encoded, disc_body) = bootstrap_discussion(
//         &app,
//         headers.clone(),
//         &app_state.dynamo.client,
//         "recording test",
//         "recording test desc",
//     )
//     .await;

//     let discussion_pk_encoded = disc_body.pk.to_string().replace('#', "%23");

//     let start_meeting_path = format!(
//         "/v3/spaces/{}/deliberation/discussions/{}/start-meeting",
//         space_pk_encoded, discussion_pk_encoded
//     );
//     let (status, _headers, _) = post! {
//         app: app,
//         path: start_meeting_path,
//         headers: headers.clone(),
//         body: {},
//         response_type: DeliberationDiscussionResponse
//     };
//     assert_eq!(status, 200);

//     let start_recording_path = format!(
//         "/v3/spaces/{}/deliberation/discussions/{}/start-recording",
//         space_pk_encoded, discussion_pk_encoded
//     );
//     let (status, _headers, resp) = post! {
//         app: app,
//         path: start_recording_path,
//         headers: headers.clone(),
//         body: {},
//         response_type: DeliberationDiscussionResponse
//     };
//     assert_eq!(status, 200);
//     assert!(!resp.members.is_empty());
// }

// #[tokio::test]
// async fn test_end_recording_handler() {
//     let TestContextV3 {
//         app,
//         test_user: (user, headers),
//         ..
//     } = setup_v3().await;
//     let app_state = create_app_state();
//     let _ = get_auth(&user);
//     let (_space_pk, space_pk_encoded, disc_body) = bootstrap_discussion(
//         &app,
//         headers.clone(),
//         &app_state.dynamo.client,
//         "recording test",
//         "recording test desc",
//     )
//     .await;

//     let discussion_pk_encoded = disc_body.pk.to_string().replace('#', "%23");

//     let start_meeting_path = format!(
//         "/v3/spaces/{}/deliberation/discussions/{}/start-meeting",
//         space_pk_encoded, discussion_pk_encoded
//     );
//     let (status, _, _) = post! {
//         app: app,
//         path: start_meeting_path,
//         headers: headers.clone(),
//         body: {},
//         response_type: DeliberationDiscussionResponse
//     };
//     assert_eq!(status, 200);

//     let start_recording_path = format!(
//         "/v3/spaces/{}/deliberation/discussions/{}/start-recording",
//         space_pk_encoded, discussion_pk_encoded
//     );
//     let (status, _, _) = post! {
//         app: app,
//         path: start_recording_path,
//         headers: headers.clone(),
//         body: {},
//         response_type: DeliberationDiscussionResponse
//     };
//     assert_eq!(status, 200);

//     let end_recording_path = format!(
//         "/v3/spaces/{}/deliberation/discussions/{}/end-recording",
//         space_pk_encoded, discussion_pk_encoded
//     );
//     let (status, _headers, resp) = post! {
//         app: app,
//         path: end_recording_path,
//         headers: headers.clone(),
//         body: {},
//         response_type: DeliberationDiscussionResponse
//     };
//     assert_eq!(status, 200);
//     assert!(!resp.members.is_empty());
// }

// async fn bootstrap_discussion(
//     app: &AxumRouter,
//     headers: HeaderMap,
//     cli: &aws_sdk_dynamodb::Client,
//     name: &str,
//     description: &str,
// ) -> (Partition, String, DeliberationDiscussionResponse) {
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

//     let now = SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .unwrap()
//         .as_secs() as i64;

//     let team_1 = create_test_user(&cli).await.pk;
//     let team_2 = create_test_user(&cli).await.pk;
//     let members = vec![team_1, team_2];

//     let space_pk = space.space_pk;
//     let space_pk_encoded = space_pk.to_string().replace('#', "%23");
//     let path = format!("/v3/spaces/{}/deliberation/deliberation", space_pk_encoded);

//     let (_status, _headers, body) = patch! {
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
//                 name: name.to_string(),
//                 description: description.to_string(),
//                 user_ids: members.clone(),
//             }],
//             "elearning_files": Vec::<File>::new(),
//         },
//         response_type: UpdateDeliberationDeliberationResponse
//     };

//     (space_pk, space_pk_encoded, body.discussions[0].clone())
// }
