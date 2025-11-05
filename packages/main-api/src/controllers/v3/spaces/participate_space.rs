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
    Extension(space): Extension<SpaceCommon>,
    Json(req): Json<ParticipateSpaceRequest>,
) -> Result<Json<ParticipateSpaceResponse>> {
    tracing::debug!("Handling request: {:?}", req);
    // TODO: Check verifiable_presentation and add user as SpaceParticipant

    let display_name = space.participants + 1;

    let now = time::get_now_timestamp_millis();

    let sp = SpaceParticipant::new(space.pk.clone(), user.pk.clone(), display_name.to_string());
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
