// use crate::models::{
//     DeliberationDiscussionMember, DeliberationDiscussionMemberQueryOption,
//     DeliberationDiscussionResponse, DeliberationPath, DeliberationSpaceDiscussion,
//     DeliberationSpaceElearning, DeliberationSpaceParticipant, ElearningResponse, Post, SpaceCommon,
// };
// use crate::types::{File, Partition, SpaceVisibility, TeamGroupPermission};
// use crate::utils::aws::DynamoClient;
// use crate::{
//     AppState, Error2,
//     models::{
//         space::{DeliberationDetailResponse, DeliberationMetadata, DiscussionCreateRequest},
//         user::User,
//     },
//     types::EntityType,
// };
// use aws_sdk_dynamodb::types::TransactWriteItem;
// use bdk::prelude::axum::extract::{Json, Path, State};
// use bdk::prelude::*;
// use serde::Deserialize;
// use validator::Validate;

// use aide::NoApi;

// #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, JsonSchema)]
// pub struct SpaceDiscussionCreateRequest {
//     pub discussion_pk: Option<String>,
//     pub started_at: i64,
//     pub ended_at: i64,

//     pub name: String,
//     pub description: String,
//     pub user_ids: Vec<Partition>,
// }

// #[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema, Validate)]
// pub struct UpdateDeliberationDeliberationRequest {
//     #[schemars(description = "Discussion informations")]
//     pub discussions: Vec<SpaceDiscussionCreateRequest>,
// }

// #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
// pub struct UpdateDeliberationDeliberationResponse {
//     pub discussions: Vec<DeliberationDiscussionResponse>,
// }

// //FIXME: implement with dynamodb upsert method
// pub async fn update_deliberation_deliberation_handler(
//     State(AppState { dynamo, .. }): State<AppState>,
//     NoApi(user): NoApi<Option<User>>,
//     Path(DeliberationPath { space_pk }): Path<DeliberationPath>,
//     Json(req): Json<UpdateDeliberationDeliberationRequest>,
// ) -> Result<Json<UpdateDeliberationDeliberationResponse>, Error2> {
//     if !matches!(space_pk, Partition::Space(_)) {
//         return Err(Error2::NotFoundDeliberationSpace);
//     }

//     let mut tx = vec![];

//     let tx_discussion = update_discussion(
//         &dynamo,
//         user.clone().unwrap_or_default(),
//         space_pk.clone(),
//         req.discussions,
//     )
//     .await?;

//     let tx_elearning = update_elearning(&dynamo, space_pk.clone(), req.elearning_files).await?;

//     let mut tx = Vec::with_capacity(tx_common.len() + tx_discussion.len() + tx_elearning.len());
//     tx.extend(tx_common);
//     tx.extend(tx_discussion);
//     tx.extend(tx_elearning);

//     dynamo
//         .client
//         .transact_write_items()
//         .set_transact_items(Some(tx))
//         .send()
//         .await
//         .map_err(|e| {
//             tracing::error!("Failed to update deliberation {}", e);
//             crate::Error2::ServerError(e.to_string())
//         })?;

//     let metadata = match DeliberationMetadata::query(&dynamo.client, space_pk).await {
//         Ok(v) => v,
//         Err(e) => {
//             tracing::debug!("deliberation metadata error: {:?}", e);
//             return Err(e);
//         }
//     };

//     let metadata: DeliberationDetailResponse = metadata.into();

//     let space_common = metadata.space_common;
//     let discussions = metadata.discussions;
//     let elearnings = metadata.elearnings;

//     Ok(Json(UpdateDeliberationDeliberationResponse {
//         space_common,
//         discussions,
//         elearnings,
//     }))
// }

// pub async fn update_discussion(
//     dynamo: &DynamoClient,
//     user: User,
//     space_pk: Partition,
//     discussions: Vec<DiscussionCreateRequest>,
// ) -> Result<Vec<TransactWriteItem>, Error2> {
//     let metadata = DeliberationMetadata::query(&dynamo.client, space_pk.clone()).await?;
//     let mut tx = vec![];

//     for data in metadata.into_iter() {
//         match data {
//             DeliberationMetadata::DeliberationSpaceParticipant(v) => {
//                 //FIXME: fix deliberation delete logic with transaction code
//                 DeliberationSpaceParticipant::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
//             }
//             DeliberationMetadata::DeliberationSpaceMember(v) => {
//                 //FIXME: fix deliberation delete logic with transaction code
//                 DeliberationDiscussionMember::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
//             }
//             // DeliberationMetadata::DeliberationSpaceDiscussion(v) => {
//             //     DeliberationSpaceDiscussion::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
//             // }
//             _ => {}
//         }
//     }

//     for discussion in discussions {
//         if discussion.discussion_pk.is_some() {
//             let discussion_id = discussion
//                 .discussion_pk
//                 .clone()
//                 .unwrap()
//                 .split("#")
//                 .last()
//                 .ok_or_else(|| Error2::BadRequest("Invalid discussion_pk format".into()))?
//                 .to_string();

//             tracing::debug!("discussion debug");
//             let d = DeliberationSpaceDiscussion::updater(
//                 &space_pk.clone(),
//                 EntityType::DeliberationDiscussion(discussion_id.to_string()),
//             )
//             .with_started_at(discussion.started_at)
//             .with_ended_at(discussion.ended_at)
//             .with_name(discussion.name)
//             .with_description(discussion.description)
//             .transact_write_item();

//             tx.push(d);

//             let option = DeliberationDiscussionMemberQueryOption::builder();

//             let deleted_members = DeliberationDiscussionMember::find_by_discussion_pk(
//                 &dynamo.client,
//                 discussion.discussion_pk.unwrap(),
//                 option,
//             )
//             .await?
//             .0;

//             for member in deleted_members {
//                 //FIXME: fix deliberation delete logic with transaction code
//                 DeliberationDiscussionMember::delete(&dynamo.client, member.pk, Some(member.sk))
//                     .await?;
//             }

//             for member in discussion.user_ids {
//                 let user = User::get(&dynamo.client, member, Some(EntityType::User))
//                     .await?
//                     .ok_or(Error2::NotFound("User not found".into()))?;

//                 let m = DeliberationDiscussionMember::new(
//                     space_pk.clone(),
//                     Partition::Discussion(discussion_id.to_string()),
//                     user,
//                 )
//                 .create_transact_write_item();

//                 tx.push(m);
//             }
//         } else {
//             let disc = DeliberationSpaceDiscussion::new(
//                 space_pk.clone(),
//                 discussion.name,
//                 discussion.description,
//                 discussion.started_at,
//                 discussion.ended_at,
//                 None,
//                 "".to_string(),
//                 None,
//                 None,
//                 user.clone(),
//             );

//             disc.create(&dynamo.client).await?;

//             let disc_id = match disc.clone().sk {
//                 EntityType::DeliberationDiscussion(v) => v,
//                 _ => "".to_string(),
//             };

//             for member in discussion.user_ids {
//                 let user = User::get(&dynamo.client, member, Some(EntityType::User))
//                     .await?
//                     .ok_or(Error2::NotFound("User not found".into()))?;

//                 let m = DeliberationDiscussionMember::new(
//                     space_pk.clone(),
//                     Partition::Discussion(disc_id.clone()),
//                     user,
//                 )
//                 .create_transact_write_item();

//                 tx.push(m);
//             }
//         }
//     }

//     Ok(tx)
// }

// pub async fn update_elearning(
//     dynamo: &DynamoClient,
//     space_pk: Partition,
//     elearning_files: Vec<File>,
// ) -> Result<Vec<TransactWriteItem>, Error2> {
//     let mut tx = vec![];
//     let elearning = DeliberationSpaceElearning::get(
//         &dynamo.client,
//         &space_pk,
//         Some(EntityType::DeliberationElearning),
//     )
//     .await?;

//     if elearning.is_some() {
//         let d = DeliberationSpaceElearning::updater(&space_pk, EntityType::DeliberationElearning)
//             .with_files(elearning_files)
//             .transact_write_item();

//         tx.push(d);
//     } else {
//         let elearning = DeliberationSpaceElearning::new(space_pk, elearning_files);
//         elearning.create(&dynamo.client).await?;
//     }

//     Ok(tx)
// }

// pub async fn update_common(
//     dynamo: &DynamoClient,
//     space_pk: Partition,

//     title: Option<String>,
//     visibility: SpaceVisibility,
//     started_at: i64,
//     ended_at: i64,
// ) -> Result<Vec<TransactWriteItem>, Error2> {
//     let mut tx = vec![];

//     let space_common = SpaceCommon::get(&dynamo.client, &space_pk, Some(EntityType::SpaceCommon))
//         .await?
//         .ok_or(Error2::NotFound("Space Common not found".to_string()))?;

//     let post_pk = space_common.post_pk;

//     let d = SpaceCommon::updater(&space_pk, EntityType::SpaceCommon)
//         .with_visibility(visibility)
//         .with_started_at(started_at)
//         .with_ended_at(ended_at)
//         .transact_write_item();

//     tx.push(d);

//     let d = Post::updater(&post_pk, EntityType::Post)
//         .with_title(title.unwrap_or_default())
//         .transact_write_item();

//     tx.push(d);

//     Ok(tx)
// }
