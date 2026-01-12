// FIXME: fix to fetcher code
// use crate::features::spaces::analyzes::SpaceAnalyze;
// use crate::features::spaces::boards::models::space_post_comment::SpacePostComment;
// use crate::posts::CreatePostResponse;
// use crate::spaces::CreateSpaceResponse;
// use crate::spaces::boards::CreateSpacePostResponse;
// use crate::tests::v3_setup::TestContextV3;
// use crate::*;

// #[tokio::test]
// async fn test_upsert_analyze() {
//     let (ctx, space_pk, space_post_pk) = setup_deliberation_space().await;
//     let TestContextV3 { app, test_user, .. } = ctx;

//     let comments = vec![
//         "핵심 쟁점이 무엇인지 먼저 정리하고, 합의 가능한 범위를 좁혀보면 좋겠어요.",
//         "이 사안의 이해관계자(당사자/운영자/외부 사용자) 관점이 각각 어떻게 다른지 궁금합니다.",
//         "단기 효과보다 장기적으로 부작용이 없는 설계인지 검토가 필요해 보여요.",
//         "현재 데이터/근거가 부족한데, 어떤 지표로 판단할지 먼저 합의하면 좋겠습니다.",
//         "대안 A와 B의 장단점을 표로 비교해보면 의사결정이 쉬울 것 같아요.",
//         "이 정책(또는 기능)의 목표가 ‘공정성’인지 ‘효율’인지 우선순위를 명확히 해야 할 듯합니다.",
//         "예외 케이스(악용/어뷰징) 시나리오를 한 번 구체적으로 써보면 리스크가 보일 것 같아요.",
//         "사용자 경험 관점에서, 절차가 복잡해지면 이탈이 늘지 않을까요?",
//         "투명성을 높이려면 공개 가능한 로그/지표 범위를 어디까지로 할지 정해야 할 것 같습니다.",
//         "운영 비용이 늘어날 수 있는데, 비용 대비 효과가 충분한지 평가가 필요해요.",
//         "기존 규칙을 유지하되, 보완 규칙(가중치/락업/상한)을 추가하는 절충안도 가능할까요?",
//         "대상 사용자군(초보/고급/관리자)마다 필요한 안내 수준이 달라 보입니다.",
//         "실제 케이스를 3~5개만 예시로 놓고 시뮬레이션하면 논쟁이 줄 것 같아요.",
//         "처벌/제한보다 인센티브 구조를 더 강화하는 방향이 사용자 반발이 적을 것 같습니다.",
//         "모호한 용어 정의(예: ‘공정’, ‘책임’, ‘참여’)를 문서로 고정하면 혼선이 줄어요.",
//         "모든 걸 한 번에 바꾸기보다, 단계적 롤아웃(베타→확대)으로 안전하게 가면 좋겠어요.",
//         "설계가 좋은지 판단하려면 실패 기준(Stop 조건)도 함께 정해야 합니다.",
//         "이 기능이 다른 모듈(알림/권한/게시판)과 충돌할 가능성이 있는지 체크가 필요합니다.",
//         "사용자 피드백을 수집할 질문지를 미리 정의하고, 정량/정성 지표를 같이 보죠.",
//         "결론보다 ‘왜 그렇게 판단했는지’ 근거가 남아야 다음 개선이 쉬울 것 같습니다.",
//     ];

//     let mut created_comments: Vec<SpacePostComment> = Vec::with_capacity(comments.len());

//     for (i, content) in comments.iter().enumerate() {
//         let content = format!("{} (#{})", content, i + 1);

//         let (status, _headers, comment) = post! {
//             app: app,
//             path: format!(
//                 "/v3/spaces/{}/boards/{}/comments",
//                 space_pk.to_string(),
//                 space_post_pk.to_string()
//             ),
//             headers: test_user.1.clone(),
//             body: {
//                 "content": content
//             },
//             response_type: SpacePostComment
//         };

//         assert_eq!(status, 200, "Failed to create comment #{}", i + 1);
//         created_comments.push(comment);
//     }

//     assert_eq!(created_comments.len(), 20);

//     let (status, _headers, analyze) = post! {
//         app: app,
//         path: format!(
//             "/v3/spaces/{}/analyzes",
//             space_pk.to_string(),
//         ),
//         headers: test_user.1.clone(),
//         body: {
//             "lda_topics": 5,
//             "tf_idf_keywords": 10,
//             "network_top_nodes": 30,
//         },
//         response_type: SpaceAnalyze
//     };

//     tracing::debug!("analyze response: {:?}", analyze);

//     assert_ne!(analyze.lda_topics.len(), 0);
//     assert_eq!(status, 200);
// }

// #[tokio::test]
// async fn test_update_analyze() {
//     let (ctx, space_pk, space_post_pk) = setup_deliberation_space().await;
//     let TestContextV3 { app, test_user, .. } = ctx;

//     let comments = vec![
//         "핵심 쟁점이 무엇인지 먼저 정리하고, 합의 가능한 범위를 좁혀보면 좋겠어요.",
//         "이 사안의 이해관계자(당사자/운영자/외부 사용자) 관점이 각각 어떻게 다른지 궁금합니다.",
//         "단기 효과보다 장기적으로 부작용이 없는 설계인지 검토가 필요해 보여요.",
//         "현재 데이터/근거가 부족한데, 어떤 지표로 판단할지 먼저 합의하면 좋겠습니다.",
//         "대안 A와 B의 장단점을 표로 비교해보면 의사결정이 쉬울 것 같아요.",
//         "이 정책(또는 기능)의 목표가 ‘공정성’인지 ‘효율’인지 우선순위를 명확히 해야 할 듯합니다.",
//         "예외 케이스(악용/어뷰징) 시나리오를 한 번 구체적으로 써보면 리스크가 보일 것 같아요.",
//         "사용자 경험 관점에서, 절차가 복잡해지면 이탈이 늘지 않을까요?",
//         "투명성을 높이려면 공개 가능한 로그/지표 범위를 어디까지로 할지 정해야 할 것 같습니다.",
//         "운영 비용이 늘어날 수 있는데, 비용 대비 효과가 충분한지 평가가 필요해요.",
//         "기존 규칙을 유지하되, 보완 규칙(가중치/락업/상한)을 추가하는 절충안도 가능할까요?",
//         "대상 사용자군(초보/고급/관리자)마다 필요한 안내 수준이 달라 보입니다.",
//         "실제 케이스를 3~5개만 예시로 놓고 시뮬레이션하면 논쟁이 줄 것 같아요.",
//         "처벌/제한보다 인센티브 구조를 더 강화하는 방향이 사용자 반발이 적을 것 같습니다.",
//         "모호한 용어 정의(예: ‘공정’, ‘책임’, ‘참여’)를 문서로 고정하면 혼선이 줄어요.",
//         "모든 걸 한 번에 바꾸기보다, 단계적 롤아웃(베타→확대)으로 안전하게 가면 좋겠어요.",
//         "설계가 좋은지 판단하려면 실패 기준(Stop 조건)도 함께 정해야 합니다.",
//         "이 기능이 다른 모듈(알림/권한/게시판)과 충돌할 가능성이 있는지 체크가 필요합니다.",
//         "사용자 피드백을 수집할 질문지를 미리 정의하고, 정량/정성 지표를 같이 보죠.",
//         "결론보다 ‘왜 그렇게 판단했는지’ 근거가 남아야 다음 개선이 쉬울 것 같습니다.",
//     ];

//     let mut created_comments: Vec<SpacePostComment> = Vec::with_capacity(comments.len());

//     for (i, content) in comments.iter().enumerate() {
//         let content = format!("{} (#{})", content, i + 1);

//         let (status, _headers, comment) = post! {
//             app: app,
//             path: format!(
//                 "/v3/spaces/{}/boards/{}/comments",
//                 space_pk.to_string(),
//                 space_post_pk.to_string()
//             ),
//             headers: test_user.1.clone(),
//             body: {
//                 "content": content
//             },
//             response_type: SpacePostComment
//         };

//         assert_eq!(status, 200, "Failed to create comment #{}", i + 1);
//         created_comments.push(comment);
//     }

//     assert_eq!(created_comments.len(), 20);

//     let (status, _headers, analyze) = post! {
//         app: app,
//         path: format!(
//             "/v3/spaces/{}/analyzes",
//             space_pk.to_string(),
//         ),
//         headers: test_user.1.clone(),
//         body: {
//             "lda_topics": 5,
//             "tf_idf_keywords": 10,
//             "network_top_nodes": 30,
//         },
//         response_type: SpaceAnalyze
//     };

//     tracing::debug!("analyze response: {:?}", analyze);

//     assert_ne!(analyze.lda_topics.len(), 0);
//     assert_eq!(status, 200);

//     let (status, _headers, updated_analyze) = patch! {
//         app: app,
//         path: format!("/v3/spaces/{}/analyzes", space_pk.to_string()),
//         headers: test_user.1.clone(),
//         body: {
//             "topics": vec![
//                 "topic_1".to_string(),
//                 "topic_2".to_string(),
//                 "topic_3".to_string(),
//                 "topic_4".to_string(),
//                 "topic_5".to_string(),
//             ],
//             "keywords": vec![
//                 vec!["합의".to_string(), "쟁점".to_string(), "범위".to_string()],
//                 vec![
//                     "이해관계자".to_string(),
//                     "관점".to_string(),
//                     "사용자".to_string(),
//                 ],
//                 vec!["부작용".to_string(), "설계".to_string(), "검토".to_string()],
//                 vec!["지표".to_string(), "데이터".to_string(), "근거".to_string()],
//                 vec!["비용".to_string(), "효과".to_string(), "운영".to_string()],
//             ],
//         },
//         response_type: SpaceAnalyze
//     };

//     assert_eq!(status, 200);
//     let lda_topics = updated_analyze.lda_topics.clone();
//     assert_eq!(lda_topics.len(), 5 * 3);
//     assert!(
//         lda_topics
//             .iter()
//             .any(|row| row.topic == "topic_3" && row.keyword == "설계"),
//         "need to '설계' keyword in topic_3"
//     );

//     let (status, _headers, fetched_analyze) = get! {
//         app: app,
//         path: format!("/v3/spaces/{}/analyzes", space_pk.to_string()),
//         headers: test_user.1.clone(),
//         response_type: SpaceAnalyze
//     };
//     assert_eq!(status, 200);
//     assert_eq!(fetched_analyze.lda_topics.len(), 15);
// }

// pub async fn setup_deliberation_space() -> (TestContextV3, Partition, Partition) {
//     let ctx = TestContextV3::setup().await;
//     let TestContextV3 { app, test_user, .. } = ctx.clone();

//     // Create a post first
//     let (_status, _headers, create_post_res) = post! {
//         app: app,
//         path: "/v3/posts",
//         headers: test_user.1.clone(),
//         response_type: CreatePostResponse
//     };

//     let post_pk = create_post_res.post_pk;

//     // Publish the post
//     let (_status, _headers, _body) = patch! {
//         app: app,
//         path: format!("/v3/posts/{}", post_pk.to_string()),
//         headers: test_user.1.clone(),
//         body: {
//             "title": "Deliberation Post",
//             "content": "<p>This is a deliberation post</p>",
//             "publish": true
//         }
//     };

//     // Create a deliberation space
//     let (status, _headers, create_space_res) = post! {
//         app: app,
//         path: "/v3/spaces",
//         headers: test_user.1.clone(),
//         body: {
//             "space_type": SpaceType::Deliberation,
//             "post_pk": post_pk,
//         },
//         response_type: CreateSpaceResponse
//     };
//     assert_eq!(status, 200);

//     let space_pk = create_space_res.space_pk;
//     let now = chrono::Utc::now().timestamp();

//     // Create a space board post
//     let (status, _headers, create_space_post_res) = post! {
//         app: app,
//         path: format!("/v3/spaces/{}/boards", space_pk.to_string()),
//         headers: test_user.1.clone(),
//         body: {
//             "title": "space boards title".to_string(),
//             "html_contents": "<div>space boards desc</div>".to_string(),
//             "category_name": "space_category".to_string(),
//             "urls": [],
//             "files": [],
//             "started_at": now,
//             "ended_at": now
//         },
//         response_type: CreateSpacePostResponse
//     };
//     assert_eq!(status, 200);

//     let (status, _headers, _res) = patch! {
//         app: app,
//         path: format!("/v3/spaces/{}", space_pk.to_string()),
//         headers: test_user.1.clone(),
//         body: {
//             "publish": true,
//             "visibility": "PUBLIC",
//         }
//     };
//     assert_eq!(status, 200);

//     (ctx, space_pk, create_space_post_res.space_post_pk)
// }
