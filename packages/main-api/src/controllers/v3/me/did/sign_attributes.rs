use crate::features::did::*;
use crate::services::portone::{IdentifyResponse, PortOne, VerifiedCustomer, VerifiedGender};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum SignAttributesRequest {
    #[schemars(description = "Sign attributes via PortOne")]
    PortOne {
        #[schemars(description = "Identity Verifycation ID")]
        id: String,
    },
}

pub async fn sign_attributes_handler(
    State(AppState {
        portone, dynamo, ..
    }): State<AppState>,
    NoApi(user): NoApi<User>,
    Json(req): Json<SignAttributesRequest>,
) -> Result<Json<VerifiedAttributes>> {
    tracing::debug!("Handling request: {:?}", req);

    match req {
        SignAttributesRequest::PortOne { id } => {
            portone_sign_attributes_handler(&portone, &dynamo.client, &user, id).await
        }
    }
}

async fn portone_sign_attributes_handler(
    portone: &PortOne,
    ddb: &aws_sdk_dynamodb::Client,
    user: &User,
    id: String,
) -> Result<Json<VerifiedAttributes>> {
    // Get identity verification data from PortOne
    let IdentifyResponse {
        verified_customer: c,
        ..
    } = portone.identify(&id).await?;
    let VerifiedCustomer {
        birth_date, gender, ..
    } = c;

    let attrs: VerifiedAttributes = VerifiedAttributes::new(user.pk.clone())
        .with_birth_date(birth_date.replace("-", ""))
        .with_gender(match gender {
            VerifiedGender::Female => Gender::Female,
            VerifiedGender::Male => Gender::Male,
            VerifiedGender::None => {
                return Err(Error::InvalidGender);
            }
        });

    attrs.upsert(ddb).await?;

    Ok(Json(attrs))
}
