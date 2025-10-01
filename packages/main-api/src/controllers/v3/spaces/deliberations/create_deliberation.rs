use crate::{
    AppState, Error2,
    models::{
        feed::Post,
        space::{DeliberationDetailResponse, DeliberationMetadata, DeliberationSpace, SpaceCommon},
    },
    types::{EntityType, Partition, TeamGroupPermission},
    utils::{
        dynamo_extractor::extract_user_from_session,
        security::{RatelResource, check_permission_from_session},
    },
};
use bdk::prelude::axum::{
    Extension,
    extract::{Json, State},
};
use bdk::prelude::*;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use urlencoding::decode;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema, Validate)]
pub struct CreateDeliberationRequest {
    #[schemars(description = "Post ID")]
    pub feed_pk: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreateDeliberationResponse {
    pub metadata: DeliberationDetailResponse,
}

const FEED_PREFIX: &str = "FEED#";

pub async fn create_deliberation_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<Session>,
    Json(req): Json<CreateDeliberationRequest>,
) -> Result<Json<CreateDeliberationResponse>, Error2> {
    let feed_pk = decode(&req.feed_pk).unwrap_or_default().to_string();
    let feed_id = feed_pk
        .strip_prefix(FEED_PREFIX)
        .ok_or_else(|| Error2::BadRequest("Invalid feed_pk format".into()))?
        .to_string();

    let post = Post::get(&dynamo.client, &feed_pk, Some(EntityType::Post))
        .await?
        .ok_or(Error2::NotFound("Post not found".to_string()))?;

    tracing::debug!("post info: {:?}", post);
    let _ = match post.user_pk.clone() {
        Partition::Team(_) => {
            check_permission_from_session(
                &dynamo.client,
                &session,
                RatelResource::Team {
                    team_pk: post.user_pk.to_string(),
                },
                vec![TeamGroupPermission::SpaceWrite],
            )
            .await?;
        }
        Partition::User(_) => {
            let user = extract_user_from_session(&dynamo.client, &session).await?;
            if user.pk != post.user_pk {
                return Err(Error2::Unauthorized(
                    "You do not have permission to create a deliberation".into(),
                ));
            }
        }
        _ => return Err(Error2::InternalServerError("Invalid post author".into())),
    };

    tracing::debug!("create_deliberation_handler called with req: {:?}", req,);
    let user = extract_user_from_session(&dynamo.client, &session).await?;
    tracing::debug!("User extracted: {:?}", user);

    let mut deliberation = DeliberationSpace::new(user);
    deliberation.user_pk = post.user_pk;
    deliberation.author_display_name = post.author_display_name;
    deliberation.author_profile_url = post.author_profile_url;
    deliberation.author_username = post.author_username;
    deliberation.create(&dynamo.client).await?;

    let common = SpaceCommon::new(
        deliberation.pk.clone(),
        crate::types::Partition::Feed(feed_id),
    );
    common.create(&dynamo.client).await?;

    let metadata = DeliberationMetadata::query(&dynamo.client, deliberation.pk.clone()).await?;

    let metadata: DeliberationDetailResponse = metadata.into();
    Ok(Json(CreateDeliberationResponse { metadata }))
}
