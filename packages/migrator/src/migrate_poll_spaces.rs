// use bdk::prelude::*;
// use main_api::{
//     features::spaces::polls::{Poll, PollUserAnswer},
//     models::{SpaceCommon, Team, User},
//     types::{Author, EntityType, Partition, Question},
// };

// use dto::Space as Sp;

// pub async fn migrate_poll_spaces(pool: &sqlx::PgPool, cli: &aws_sdk_dynamodb::Client) {
//     let poll_spaces: Vec<dto::Space> = dto::Space::query_builder(0)
//         .space_type_equals(dto::SpaceType::Poll)
//         .query()
//         .map(Into::into)
//         .fetch_all(pool)
//         .await
//         .expect("Failed to fetch poll spaces from Postgres");
//     tracing::info!("Total poll spaces to migrate: {}", poll_spaces.len());

//     for poll_space in poll_spaces {
//         let Sp {
//             id,
//             created_at,
//             updated_at,
//             title, // depends on post
//             html_contents,
//             space_type,
//             owner_id: _,    // depends on post
//             industry_id: _, // not supported
//             started_at,
//             ended_at,
//             feed_id,
//             status,
//             files: _,     // no data
//             contracts: _, // no data
//             author,
//             industry: _,            // not supported
//             badges: _,              // no data
//             members: _,             // not supported
//             groups: _,              // not supported
//             num_of_redeem_codes: _, // not supported
//             codes: _,               // not supported
//             comments: _,            // depends on post
//             feed_comments: _,       // depends on post
//             discussions: _,         // not supported
//             elearnings: _,          // not suuported
//             surveys: _,             // skip
//             responses: _,           // skip
//             user_responses: _,      // skip
//             drafts: _,              // ignored
//             likes: _,               // depends on post
//             shares: _,              // depends on post
//             is_liked: _,            // ignored
//             rewards: _,             // not supported
//             is_bookmarked: _,       // ignored
//             number_of_comments: _,  // depends on post
//             image_url: _,           // replaced with content
//             sprint_leagues: _,      // not supported
//             notice_quiz: _,         // not supported
//             booster_type: _,        // not supported
//             publishing_scope,
//         } = poll_space;

//         let post_pk = Partition::Feed(feed_id.to_string());
//         let author = author.first().cloned().unwrap();
//         let user: Author = match &author.user_type {
//             &dto::UserType::Individual => User::get(
//                 cli,
//                 Partition::User(author.id.to_string()),
//                 Some(EntityType::User),
//             )
//             .await
//             .unwrap()
//             .unwrap()
//             .into(),
//             &dto::UserType::Team => Team::get(
//                 cli,
//                 Partition::Team(author.id.to_string()),
//                 Some(EntityType::Team),
//             )
//             .await
//             .unwrap()
//             .unwrap()
//             .into(),
//             _ => unimplemented!(),
//         };

//         let mut space = SpaceCommon::new(post.clone())
//             .with_space_type(main_api::types::SpaceType::Poll)
//             .with_created_at(created_at * 1000)
//             .with_updated_at(updated_at * 1000)
//             .with_content(html_contents)
//             .with_visibility(match publishing_scope {
//                 dto::PublishingScope::Private => main_api::types::SpaceVisibility::Private,
//                 dto::PublishingScope::Public => main_api::types::SpaceVisibility::Public,
//             })
//             .with_publish_state(match status {
//                 dto::SpaceStatus::Draft => main_api::types::SpacePublishState::Draft,
//                 _ => main_api::types::SpacePublishState::Published,
//             })
//             .with_status(match status {
//                 dto::SpaceStatus::Draft => main_api::types::SpaceStatus::Waiting,
//                 dto::SpaceStatus::InProgress => main_api::types::SpaceStatus::InProgress,
//                 dto::SpaceStatus::Finish => main_api::types::SpaceStatus::Finished,
//             });

//         if let Some(started_at) = started_at {
//             space = space.with_started_at(started_at * 1000);
//         }

//         if let Some(ended_at) = ended_at {
//             space = space.with_ended_at(ended_at * 1000);
//         }

//         if let Err(e) = space.create(cli).await {
//             tracing::error!("Failed to create poll space {}: {:?}", id, e);
//         }

//         // surveys
//         let surveys: Vec<dto::Survey> = dto::Survey::query_builder()
//             .space_id_equals(id)
//             .query()
//             .map(Into::into)
//             .fetch_all(pool)
//             .await
//             .expect("Failed to fetch surveys from Postgres");
//         if surveys.len() == 0 {
//             tracing::warn!("No surveys found for poll space {}", id);
//             continue;
//         } else if surveys.len() > 1 {
//             panic!(
//                 "Multiple surveys found for poll space {}. Only the first one will be migrated.",
//                 id
//             );
//         } else {
//             use dto::Survey as DSurvey;

//             let DSurvey {
//                 id,
//                 created_at,
//                 updated_at,
//                 space_id: _, // manipulated above by space.pk
//                 status: _,   // depends on space
//                 started_at,
//                 ended_at,
//                 questions,
//                 response_count,
//             } = surveys[0].clone();

//             let questions = questions
//                 .clone()
//                 .into_iter()
//                 .map(|q| {
//                     let val = serde_json::to_value(q).unwrap();
//                     let question: Question = serde_json::from_value(val).unwrap();
//                     question
//                 })
//                 .collect();

//             let poll = Poll::new(space.pk.clone(), None)
//                 .unwrap()
//                 .with_user_response_count(response_count)
//                 .with_questions(questions)
//                 .with_created_at(created_at * 1000)
//                 .with_updated_at(updated_at * 1000)
//                 .with_started_at(started_at * 1000)
//                 .with_ended_at(ended_at * 1000);

//             if let Err(e) = poll.create(cli).await {
//                 tracing::error!("Failed to create poll for space {}: {:?}", id, e);
//             }

//             // survey responses
//             let responses: Vec<dto::SurveyResponse> = dto::SurveyResponse::query_builder()
//                 .space_id_equals(id)
//                 .query()
//                 .map(Into::into)
//                 .fetch_all(pool)
//                 .await
//                 .expect("Failed to fetch survey responses from Postgres");

//             for response in responses {
//                 let user_pk = Partition::User(response.user_id.to_string());

//                 let user = User::get(&cli, user_pk, Some(EntityType::User))
//                     .await
//                     .unwrap_or_default()
//                     .unwrap_or_default();
//                 let answers = response
//                     .answers
//                     .into_iter()
//                     .map(|a| {
//                         let val = serde_json::to_value(a).unwrap();
//                         let answer: main_api::types::Answer = serde_json::from_value(val).unwrap();
//                         answer
//                     })
//                     .collect();

//                 let create_tx = PollUserAnswer::new(
//                     poll.pk.clone(),
//                     poll.sk.clone().try_into().unwrap(),
//                     answers,
//                     None,
//                     user,
//                 );
//             }
//         }
//     }
// }
