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
    NoApi(user): NoApi<User>,
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

    let poll = Poll::new(space_pk, sk)?;
    poll.create(&dynamo.client).await?;

    let poll_response: PollResponse = PollResponse::from(poll);

    Ok(Json(poll_response))
}
