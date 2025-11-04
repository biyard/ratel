use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct ParticipateSpaceRequest {
    #[schemars(description = "Name of the entity")]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct ParticipateSpaceResponse {
    #[schemars(description = "Status of the operation")]
    pub status: String,
}

pub async fn participate_space_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Json(req): Json<ParticipateSpaceRequest>,
) -> Result<Json<ParticipateSpaceResponse>> {
    tracing::debug!("Handling request: {:?}", req);
    // TODO: Implement the handler logic here

    unimplemented!()
}
