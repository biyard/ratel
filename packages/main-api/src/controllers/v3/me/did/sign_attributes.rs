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

    #[schemars(description = "Verify attributes by pre-issued code")]
    Code {
        #[schemars(description = "Code issued in advanced")]
        code: String,
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
        SignAttributesRequest::Code { code } => {
            add_attributes_by_code(&dynamo.client, &user, code).await
        }
    }
}

async fn add_attributes_by_code(
    ddb: &aws_sdk_dynamodb::Client,
    user: &User,
    code: String,
) -> Result<Json<VerifiedAttributes>> {
    let AttributeCode {
        birth_date,
        gender,
        university,
        ..
    } = AttributeCode::get(ddb, Partition::AttributeCode(code), None::<String>)
        .await?
        .ok_or(Error::AttributeCodeNotFound)?;

    let (pk, sk) = VerifiedAttributes::keys(&user.pk);
    let mut u = VerifiedAttributes::updater(pk, sk);

    if let Some(birth_date) = birth_date {
        u = u.with_birth_date(birth_date);
    }

    if let Some(gender) = gender {
        u = u.with_gender(gender);
    }

    if let Some(university) = university {
        u = u.with_university(university);
    }

    let attrs = u.execute(ddb).await?;

    Ok(Json(attrs))
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

    let (pk, sk) = VerifiedAttributes::keys(&user.pk);
    let attrs = VerifiedAttributes::updater(pk, sk)
        .with_birth_date(birth_date.replace("-", ""))
        .with_gender(match gender {
            VerifiedGender::Female => Gender::Female,
            VerifiedGender::Male => Gender::Male,
            VerifiedGender::None => {
                return Err(Error::InvalidGender);
            }
        })
        .execute(ddb)
        .await?;

    Ok(Json(attrs))
}
