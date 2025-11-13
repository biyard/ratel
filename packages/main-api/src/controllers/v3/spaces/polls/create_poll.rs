use aws_sdk_dynamodb::types::TransactWriteItem;

use crate::features::spaces::SpaceRequirement;
use crate::models::space::SpaceCommon;

use crate::features::spaces::polls::*;
use crate::models::user::User;
use crate::spaces::SpacePath;
use crate::spaces::SpacePathParam;
use crate::types::{EntityType, Partition, Question, TeamGroupPermission};
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
            space_pk,
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
        .expect("failed to create poll");

    Ok(Json(poll.into()))
}
