use super::*;
use crate::features::auth::User;
use crate::features::membership::controllers::normalize_error;
use crate::features::membership::models::UserPayment;
#[cfg(feature = "server")]
use crate::features::membership::services::portone::PortOne;
use crate::features::membership::services::portone::VerifiedCustomer;
use crate::features::membership::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "camelCase")]
pub struct IdentificationRequest {
    pub id: String,
}

#[post("/v3/payments/identify", user: User)]
pub async fn identify_handler(req: IdentificationRequest) -> Result<VerifiedCustomer> {
    let result = async {
        let conf = crate::features::membership::config::get();
        let cli = conf.common.dynamodb();
        let portone = PortOne::new(conf.portone.api_secret);

        let result = portone.identify(&req.id).await?;
        let verified = result.verified_customer.clone();

        UserPayment::new(
            user.pk,
            verified.id.clone(),
            verified.name.clone(),
            verified.birth_date.clone(),
        )
        .create(cli)
        .await?;

        Ok(verified)
    }
    .await;

    result.map_err(normalize_error)
}
