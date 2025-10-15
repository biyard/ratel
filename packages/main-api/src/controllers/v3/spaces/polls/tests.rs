// use crate::controllers::v3::spaces::create_space::CreateSpaceResponse;
// use crate::controllers::v3::spaces::tests::setup_post;
// use crate::tests::v3_setup::TestContextV3;
// use crate::*;

// #[tokio::test]
// async fn test_full_poll() {
//     let (ctx, post_pk, space_pk) = setup_space().await;

//     let TestContextV3 {
//         app,
//         test_user,
//         now,
//         ..
//     } = ctx.clone();
// }

// async fn setup_space() -> (TestContextV3, String, String) {
//     let (ctx, post_pk) = setup_post().await;

//     let TestContextV3 {
//         app,
//         now,
//         test_user: (_user, headers),
//         ..
//     } = ctx.clone();

//     let (status, _, res) = post! {
//         app: app,
//         path: "/v3/spaces",
//         headers: headers.clone(),
//         body: {
//             "space_type": 2,
//             "post_pk": post_pk.clone(),
//         },
//         response_type: CreateSpaceResponse
//     };
//     tracing::debug!("Create space response: {:?}", res);
//     assert_eq!(status, 200);
//     assert!(res.space_pk.to_string().len() > 0);
//     let space_pk = res.space_pk;

//     (ctx, post_pk, space_pk.to_string())
// }
