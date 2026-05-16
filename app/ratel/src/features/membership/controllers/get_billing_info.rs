use super::*;
use crate::features::auth::User;
use crate::features::membership::controllers::normalize_error;
use crate::features::membership::models::UserPayment;
use crate::features::membership::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct BillingInfoResponse {
    pub has_card: bool,
    pub customer_name: String,
    pub masked_card_number: Option<String>,
}

#[get("/v3/me/billing", user: User)]
pub async fn get_billing_info_handler() -> Result<BillingInfoResponse> {
    let result = async {
        let conf = crate::features::membership::config::get();
        let cli = conf.common.dynamodb();

        let pk = CompositePartition::user_payment_pk(user.pk.into());
        let payment: Option<UserPayment> = UserPayment::get(cli, &pk, None::<String>).await?;

        match payment {
            Some(p) => Ok(BillingInfoResponse {
                has_card: p.billing_key.is_some(),
                customer_name: p.name,
                masked_card_number: p.masked_card_number,
            }),
            None => Ok(BillingInfoResponse::default()),
        }
    }
    .await;

    result.map_err(normalize_error)
}
