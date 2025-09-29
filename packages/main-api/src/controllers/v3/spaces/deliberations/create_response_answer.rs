// use crate::{
//     AppState, Error2,
//     controllers::v3::spaces::deliberations::update_deliberation::DeliberationPath,
//     models::space::{
//         DeliberationDetailResponse, DeliberationMetadata, DeliberationSpace, SpaceCommon,
//     },
//     utils::dynamo_extractor::extract_user,
// };
// use dto::by_axum::{
//     auth::Authorization,
//     axum::{
//         Extension,
//         extract::{Json, Path, State},
//     },
// };

// use dto::{JsonSchema, aide, schemars};
// use serde::{Deserialize, Serialize};
// use validator::Validate;

// #[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema, Validate)]
// pub struct CreateResponseAnswerRequest {
//     pub survey_id: String,
//     pub survey_type: SurveyType,
//     pub answers: Vec<SurveyAnswer>,
// }

// #[derive(Debug, Clone, Serialize, Default, aide::OperationIo, JsonSchema)]
// pub struct CreateDeliberationResponse {
//     pub metadata: DeliberationDetailResponse,
// }

// pub async fn create_response_answer_handler(
//     State(AppState { dynamo, .. }): State<AppState>,
//     Extension(auth): Extension<Option<Authorization>>,
//     Path(DeliberationPath { id }): Path<DeliberationPath>,
//     Json(req): Json<CreateDeliberationRequest>,
// ) -> Result<Json<CreateDeliberationResponse>, Error2> {
//     let user = extract_user(&dynamo.client, auth).await?;

//     let deliberation = DeliberationSpace::new(user);
//     deliberation.create(&dynamo.client).await?;

//     let common = SpaceCommon::new(
//         deliberation.pk.clone(),
//         crate::types::Partition::Feed(req.feed_id),
//     );
//     common.create(&dynamo.client).await?;

//     let metadata = DeliberationMetadata::query(&dynamo.client, deliberation.pk.clone()).await?;

//     let metadata: DeliberationDetailResponse = metadata.into();
//     Ok(Json(CreateDeliberationResponse { metadata }))
// }
