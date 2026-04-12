use super::super::models::{AttributeCode, VerifiedAttributes};
use super::super::*;
use super::get_credentials::CredentialResponse;
use crate::features::social::types::SocialError;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum SignAttributesRequest {
    PortOne { id: String },
    Code { code: String },
}

#[post("/api/me/did", user: crate::features::auth::User)]
pub async fn sign_attributes_handler(body: SignAttributesRequest) -> Result<CredentialResponse> {
    debug!("Handling sign attributes request: {:?}", body);
    let conf = crate::common::config::ServerConfig::default();
    let cli = conf.dynamodb();
    let attrs = match body {
        SignAttributesRequest::PortOne { id } => portone_sign_attributes(cli, &user, id).await?,
        SignAttributesRequest::Code { code } => add_attributes_by_code(cli, &user, code).await?,
    };

    return Ok(CredentialResponse {
        age: attrs.age(),
        gender: attrs.gender.map(|value| value.to_string()),
        university: attrs.university,
    });
}

#[cfg(feature = "server")]
async fn add_attributes_by_code(
    cli: &crate::common::aws_sdk_dynamodb::Client,
    user: &crate::features::auth::User,
    code: String,
) -> Result<VerifiedAttributes> {
    let code = code.trim().to_string();
    if code.is_empty() {
        return Err(SocialError::InvalidVerificationAttribute.into());
    }

    let code_item = AttributeCode::get(
        cli,
        Partition::AttributeCode(code),
        Some(EntityType::AttributeCode),
    )
    .await?
    .ok_or_else(|| Error::NotFound("Attribute code not found".to_string()))?;

    let (pk, sk) = VerifiedAttributes::keys(&user.pk);
    let mut attrs = VerifiedAttributes::get(cli, pk.clone(), Some(sk.clone()))
        .await?
        .unwrap_or_default();
    attrs.pk = pk;
    attrs.sk = sk;

    if let Some(birth_date) = code_item.birth_date {
        attrs.birth_date = Some(birth_date);
    }
    attrs.gender = code_item.gender.or(attrs.gender);
    if let Some(university) = code_item.university {
        attrs.university = Some(university);
    }

    attrs.upsert(cli).await?;
    Ok(attrs)
}

#[cfg(all(feature = "server", not(feature = "bypass")))]
async fn portone_sign_attributes(
    cli: &crate::common::aws_sdk_dynamodb::Client,
    user: &crate::features::auth::User,
    id: String,
) -> Result<VerifiedAttributes> {
    use super::super::services::PortOneClient;

    let portone = PortOneClient::new();
    let response = portone.identify(&id).await?;
    let verified = response.verified_customer;

    let gender = match verified.gender.to_lowercase().as_str() {
        "male" => Some(crate::common::attribute::Gender::Male),
        "female" => Some(crate::common::attribute::Gender::Female),
        _ => return Err(SocialError::InvalidGender.into()),
    };

    let birth_date = verified.birth_date.replace('-', "");

    let (pk, sk) = VerifiedAttributes::keys(&user.pk);
    let mut attrs = VerifiedAttributes::get(cli, pk.clone(), Some(sk.clone()))
        .await?
        .unwrap_or_default();
    attrs.pk = pk;
    attrs.sk = sk;
    attrs.birth_date = Some(birth_date);
    attrs.gender = gender.or(attrs.gender);

    attrs.upsert(cli).await?;
    Ok(attrs)
}

#[cfg(all(feature = "server", feature = "bypass"))]
async fn portone_sign_attributes(
    cli: &crate::common::aws_sdk_dynamodb::Client,
    user: &crate::features::auth::User,
    id: String,
) -> Result<VerifiedAttributes> {
    use super::super::services::PortOneClient;

    let gender = Some(crate::common::attribute::Gender::Male);

    let birth_date = "20000101".to_string();

    let (pk, sk) = VerifiedAttributes::keys(&user.pk);
    let mut attrs = VerifiedAttributes::get(cli, pk.clone(), Some(sk.clone()))
        .await?
        .unwrap_or_default();
    attrs.pk = pk;
    attrs.sk = sk;
    attrs.birth_date = Some(birth_date);
    attrs.gender = gender.or(attrs.gender);

    attrs.upsert(cli).await?;
    Ok(attrs)
}
