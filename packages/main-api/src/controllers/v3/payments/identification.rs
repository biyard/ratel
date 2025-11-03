use crate::{
    features::payment::UserPayment,
    services::portone::{IdentifyResponse, VerifiedCustomer},
    *,
};

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct IdentificationRequest {
    #[schemars(description = "Identity Verifycation ID")]
    pub id: String,
}

pub async fn identification_handler(
    State(AppState {
        dynamo, portone, ..
    }): State<AppState>,
    NoApi(user): NoApi<User>,
    Json(req): Json<IdentificationRequest>,
) -> Result<Json<VerifiedCustomer>> {
    tracing::debug!("Handling request: {:?}", req);
    let result = portone.identify(&req.id).await?;

    UserPayment::new(
        user.pk,
        result.verified_customer.id.clone(),
        result.verified_customer.name.clone(),
        result.verified_customer.birth_date.clone(),
    )
    .create(&dynamo.client)
    .await?;

    Ok(Json(result.verified_customer))
}
