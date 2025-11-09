use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct DeleteAttributeCodeResponse {
    #[schemars(description = "Status of the operation")]
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct DeleteAttributeCodeParam {
    pub code_pk: String,
}

pub type DeleteAttributeCodePay = Path<DeleteAttributeCodeParam>;

pub async fn delete_attribute_code_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Path(DeleteAttributeCodeParam { code_pk }): DeleteAttributeCodePay,
) -> Result<Json<DeleteAttributeCodeResponse>> {
    AttributeCode::delete(
        &dynamo.client,
        &code_pk,
        Some(EntityType::VerifiedAttributes),
    )
    .await?;

    Ok(Json(DeleteAttributeCodeResponse {
        status: "Deleted".to_string(),
    }))
}
