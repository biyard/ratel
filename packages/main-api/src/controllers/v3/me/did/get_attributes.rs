use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct GetAttributesResponse {
    pub age: Option<u32>,
    pub gender: Option<Gender>,
    pub university: Option<String>,
}

pub async fn get_attributes_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
) -> Result<Json<GetAttributesResponse>> {
    let res = VerifiedAttributes::get(
        &dynamo.client,
        CompositePartition(user.pk.clone(), Partition::Attributes),
        None::<String>,
    )
    .await?
    .unwrap_or_default();

    Ok(Json(GetAttributesResponse {
        age: res.age(),
        gender: res.gender,
        university: res.university,
    }))
}
