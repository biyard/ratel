use crate::models::VerifiedAttributes;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CredentialResponse {
    pub age: Option<u32>,
    pub gender: Option<String>,
    pub university: Option<String>,
}

#[get("/api/me/credentials", user: ratel_auth::User)]
pub async fn get_credentials_handler() -> Result<CredentialResponse> {
    let conf = common::config::ServerConfig::default();
    let cli = conf.dynamodb();

    let pk = CompositePartition(user.pk.clone(), Partition::Attributes);
    let res = VerifiedAttributes::get(cli, pk, None::<String>)
        .await?
        .unwrap_or_default();

    Ok(CredentialResponse {
        age: res.age(),
        gender: res.gender.map(|value| value.to_string()),
        university: res.university,
    })
}
