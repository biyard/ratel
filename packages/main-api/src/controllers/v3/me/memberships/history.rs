use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct GetPurchaseHistoryResponse {
    #[schemars(description = "Status of the operation")]
    pub status: String,
}

pub async fn get_purchase_history_handler(
    State(AppState { .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
    Query(Pagination { bookmark: _ }): PaginationQuery,
) -> Result<Json<ListResponse<GetPurchaseHistoryResponse>>> {
    // TODO: Implement the handler logic here

    unimplemented!()
}
