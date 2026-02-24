use crate::controllers::get_credentials::CredentialResponse;
use crate::models::{AttributeCodeLocal, VerifiedAttributesLocal};
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum SignAttributesRequest {
    PortOne { id: String },
    Code { code: String },
}

#[post("/api/me/did", user: ratel_auth::User)]
pub async fn sign_attributes_handler(body: SignAttributesRequest) -> Result<CredentialResponse> {
    #[cfg(not(feature = "server"))]
    {
        let _ = body;
        return Err(Error::NotSupported(
            "DID attribute signing is server-only".to_string(),
        ));
    }

    #[cfg(feature = "server")]
    {
        let conf = common::config::ServerConfig::default();
        let cli = conf.dynamodb();
        let attrs = match body {
            SignAttributesRequest::PortOne { id } => {
                portone_sign_attributes(cli, &user, id).await?
            }
            SignAttributesRequest::Code { code } => {
                add_attributes_by_code(cli, &user, code).await?
            }
        };

        return Ok(CredentialResponse {
            age: attrs.age(),
            gender: attrs.gender,
            university: attrs.university,
        });
    }
}

#[cfg(feature = "server")]
async fn add_attributes_by_code(
    cli: &common::aws_sdk_dynamodb::Client,
    user: &ratel_auth::User,
    code: String,
) -> Result<VerifiedAttributesLocal> {
    let code = code.trim().to_string();
    if code.is_empty() {
        return Err(Error::BadRequest(
            "Verification code is required".to_string(),
        ));
    }

    let code_item = AttributeCodeLocal::get(
        cli,
        Partition::AttributeCode(code),
        Some(EntityType::AttributeCode),
    )
    .await?
    .ok_or_else(|| Error::NotFound("Attribute code not found".to_string()))?;

    let (pk, sk) = VerifiedAttributesLocal::keys(&user.pk);
    let mut attrs = VerifiedAttributesLocal::get(cli, pk.clone(), Some(sk.clone()))
        .await?
        .unwrap_or_default();
    attrs.pk = pk;
    attrs.sk = sk;

    if let Some(birth_date) = code_item.birth_date {
        attrs.birth_date = Some(birth_date);
    }
    if let Some(gender) = code_item.gender {
        attrs.gender = Some(gender);
    }
    if let Some(university) = code_item.university {
        attrs.university = Some(university);
    }

    attrs.upsert(cli).await?;
    Ok(attrs)
}

#[cfg(feature = "server")]
async fn portone_sign_attributes(
    cli: &common::aws_sdk_dynamodb::Client,
    user: &ratel_auth::User,
    id: String,
) -> Result<VerifiedAttributesLocal> {
    use crate::services::PortOneClient;

    let portone = PortOneClient::new();
    let response = portone.identify(&id).await?;
    let verified = response.verified_customer;

    let gender = match verified.gender.to_lowercase().as_str() {
        "male" => Some("male".to_string()),
        "female" => Some("female".to_string()),
        _ => return Err(Error::BadRequest("Invalid gender".to_string())),
    };

    let birth_date = verified.birth_date.replace('-', "");

    let (pk, sk) = VerifiedAttributesLocal::keys(&user.pk);
    let mut attrs = VerifiedAttributesLocal::get(cli, pk.clone(), Some(sk.clone()))
        .await?
        .unwrap_or_default();
    attrs.pk = pk;
    attrs.sk = sk;
    attrs.birth_date = Some(birth_date);
    if let Some(gender) = gender {
        attrs.gender = Some(gender);
    }

    attrs.upsert(cli).await?;
    Ok(attrs)
}
