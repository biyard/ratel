use crate::features::social::users::*;

#[derive(Debug, Clone)]
pub struct PortOneClient {
    cli: common::reqwest::Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortOneIdentifyResponse {
    pub verified_customer: PortOneVerifiedCustomer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortOneVerifiedCustomer {
    pub birth_date: String,
    pub gender: String,
}

impl PortOneClient {
    pub fn new() -> Self {
        let api_secret = option_env!("PORTONE_API_SECRET").unwrap_or_else(|| {
            warn!(
                "PORTONE_API_SECRET not set, using default value. Identity verification may fail."
            );
            "your_default_api_secret"
        });

        let cli = common::reqwest::Client::builder()
            .default_headers({
                let mut headers = common::reqwest::header::HeaderMap::new();
                headers.insert(
                    "Authorization",
                    common::reqwest::header::HeaderValue::from_str(&format!(
                        "PortOne {}",
                        api_secret
                    ))
                    .unwrap(),
                );
                headers
            })
            .build()
            .unwrap();

        Self { cli }
    }

    pub async fn identify(&self, id: &str) -> Result<PortOneIdentifyResponse> {
        let res = self
            .cli
            .get(format!(
                "https://api.portone.io/identity-verifications/{}",
                id
            ))
            .send()
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(Error::BadRequest(format!(
                "PortOne identify failed ({}): {}",
                status, text
            )));
        }

        res.json().await.map_err(|e| Error::Unknown(e.to_string()))
    }
}
