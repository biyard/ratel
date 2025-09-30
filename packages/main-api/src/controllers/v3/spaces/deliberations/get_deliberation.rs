use crate::{
    AppState, Error2,
    models::space::{
        DeliberationDetailResponse, DeliberationMetadata, DeliberationSpace, SpaceCommon,
    },
    types::{EntityType, Partition, SpaceVisibility, TeamGroupPermission},
    utils::{
        dynamo_extractor::extract_user_from_session,
        security::{RatelResource, check_permission_from_session},
    },
};
use dto::by_axum::axum::{
    Extension,
    extract::{Json, Path, State},
};
use dto::{aide, schemars};
use tower_sessions::Session;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct DeliberationGetPath {
    pub space_pk: String,
}

pub async fn get_deliberation_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<Session>,
    Path(DeliberationGetPath { space_pk }): Path<DeliberationGetPath>,
) -> Result<Json<DeliberationDetailResponse>, Error2> {
    let space_pk = space_pk.replace("%23", "#");
    tracing::debug!(
        "get_deliberation_handler called with space_pk: {}",
        space_pk
    );
    let metadata = DeliberationMetadata::query(&dynamo.client, space_pk.clone()).await?;

    let space = DeliberationSpace::get(&dynamo.client, &space_pk, Some(EntityType::Space))
        .await?
        .ok_or(Error2::NotFound("Space not found".to_string()))?;

    let space_common = SpaceCommon::get(&dynamo.client, &space_pk, Some(EntityType::SpaceCommon))
        .await?
        .ok_or(Error2::NotFound("Space not found".to_string()))?;

    if space_common.visibility != SpaceVisibility::Public {
        let _ = match space.user_pk.clone() {
            Partition::Team(_) => {
                check_permission_from_session(
                    &dynamo.client,
                    &session,
                    RatelResource::Team {
                        team_pk: space.user_pk.to_string(),
                    },
                    vec![TeamGroupPermission::SpaceRead],
                )
                .await?;
            }
            Partition::User(_) => {
                let user = extract_user_from_session(&dynamo.client, &session).await?;
                if user.pk != space.user_pk {
                    return Err(Error2::Unauthorized(
                        "You do not have permission to delete this post".into(),
                    ));
                }
            }
            _ => return Err(Error2::InternalServerError("Invalid post author".into())),
        };
    }

    tracing::debug!("Deliberation metadata retrieved: {:?}", metadata);
    let metadata: DeliberationDetailResponse = metadata.into();

    tracing::debug!("DeliberationDetailResponse formed: {:?}", metadata);

    Ok(Json(metadata))
}
