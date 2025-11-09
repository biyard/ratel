use super::*;

pub async fn list_attribute_codes_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Query(Pagination { bookmark }): Query<Pagination>,
) -> Result<Json<ListItemsResponse<AttributeCode>>> {
    let res = AttributeCode::find(
        &dynamo.client,
        EntityType::AttributeCode,
        AttributeCode::opt_with_bookmark(bookmark),
    )
    .await?;

    Ok(Json(res.into()))
}
