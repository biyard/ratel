use aws_sdk_dynamodb::types::TransactWriteItem;

use crate::features::spaces::SpaceRequirement;
use crate::models::Post;
use crate::models::space::SpaceCommon;
use crate::utils::aws::PollScheduler;

use crate::features::spaces::polls::*;
use crate::models::user::User;
use crate::spaces::SpacePath;
use crate::spaces::SpacePathParam;
use crate::types::{EntityType, Partition, Question, TeamGroupPermission};
use crate::utils::aws::get_aws_config;
use crate::utils::time::get_now_timestamp_millis;
use crate::*;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema, Default)]
pub struct CreatePollSpaceRequest {
    default: bool,
}

pub async fn create_poll_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Extension(space): Extension<SpaceCommon>,
    Json(req): Json<CreatePollSpaceRequest>,
) -> crate::Result<Json<PollResponse>> {
    //Request Validation
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceEdit) {
        return Err(Error::NoPermission);
    }

    let sk = if req.default {
        Some(space_pk.to_poll_sk()?)
    } else {
        None
    };

    let poll = Poll::new(space_pk.clone(), sk)?;
    let mut items: Vec<TransactWriteItem> = Vec::with_capacity(2);
    items.push(poll.create_transact_write_item());

    if poll.is_default_poll() {
        let requirement = SpaceRequirement::new(
            space_pk.clone(),
            features::spaces::SpaceRequirementType::PrePoll,
            (poll.pk.to_string(), poll.sk.clone()),
        );
        items.push(requirement.create_transact_write_item());
    }

    dynamo
        .client
        .transact_write_items()
        .set_transact_items(Some(items))
        .send()
        .await
        .map_err(|e| {
            tracing::debug!("Failed to create poll: {:?}", e);
            Error::InternalServerError(e.to_string())
        })?;

    if space.status == Some(crate::types::SpaceStatus::InProgress) {
        let sdk_config = get_aws_config();
        let scheduler = PollScheduler::new(&sdk_config);

        poll.schedule_start_notification(&scheduler, poll.started_at)
            .await?;
    }

    Ok(Json(poll.into()))
}
