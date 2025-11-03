use crate::*;
use percent_encoding::{AsciiSet, NON_ALPHANUMERIC};

use super::{IdentifyResponse, VerifiedCustomer, VerifiedGender};

const BASE_URL: &str = "https://api.portone.io/identity-verifications/";

#[derive(Debug, Clone)]
pub struct PortOne {
    cli: reqwest::Client,
}

impl PortOne {
    pub fn new(api_secret: &str) -> Self {
        let cli = reqwest::Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    "Authorization",
                    reqwest::header::HeaderValue::from_str(&format!("PortOne {}", api_secret))
                        .unwrap(),
                );
                headers
            })
            .build()
            .unwrap();
        Self { cli }
    }

    pub async fn identify(&self, id: &str) -> Result<IdentifyResponse> {
        let res = self
            .cli
            .get(format!(
                "{}/{}",
                BASE_URL,
                percent_encoding::utf8_percent_encode(id, NON_ALPHANUMERIC)
            ))
            .send()
            .await?;

        Ok(res.json().await?)
    }
}
