use names::{Generator, Name};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct ParticipateSpaceRequest {
    #[schemars(description = "Proof if the user has rights to participate in the space")]
    pub verifiable_presentation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct ParticipateSpaceResponse {
    pub username: String,
    pub display_name: String,
    pub profile_url: String,
}

pub async fn participate_space_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): Path<SpacePathParam>,
    Json(req): Json<ParticipateSpaceRequest>,
) -> Result<Json<ParticipateSpaceResponse>> {
    tracing::debug!("Handling request: {:?}", req);
    // TODO: Check verifiable_presentation and add user as SpaceParticipant

    let is_verified = SpaceParticipant::verify_credential(&dynamo, &space_pk, user.clone()).await;

    if !is_verified {
        return Err(Error::InvalidPanel);
    }

    let (space, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        Some(&user.pk),
        TeamGroupPermission::SpaceRead,
    )
    .await?;
    if !has_perm {
        return Err(Error::NoPermission);
    }

    let now = time::get_now_timestamp_millis();

    let display_name = Generator::with_naming(Name::Numbered)
        .next()
        .unwrap()
        .replace('-', " ");

    let sp = SpaceParticipant::new(space.pk.clone(), user.pk.clone(), display_name);
    let new_space = SpaceCommon::updater(&space.pk, &space.sk)
        .increase_participants(1)
        .with_updated_at(now);

    transact_write!(
        &dynamo.client,
        sp.create_transact_write_item(),
        new_space.transact_write_item(),
    )?;

    Ok(Json(ParticipateSpaceResponse {
        username: sp.username,
        display_name: sp.display_name,
        profile_url: sp.profile_url,
    }))
}
