use crate::config;
use dto::Result;
use ethers::core::k256::sha2::Sha256;
use hmac::{Hmac, Mac};
use serde::Deserialize;

// https://core.telegram.org/bots/webapps#validating-data-received-via-the-mini-app

pub fn parse_telegram_raw(telegram_raw: String) -> Result<TelegramUser> {
    let config = config::get();
    let telegram_token = config.telegram_token.unwrap_or_default();

    let validation_result = (|| {
        let mut params: Vec<(String, String)> =
            url::form_urlencoded::parse(telegram_raw.as_bytes())
                .into_owned()
                .collect();
        tracing::debug!("Telegram raw params: {:?}", params);

        let received_hash = params
            .iter()
            .position(|(key, _)| key == "hash")
            .map(|index| params.remove(index).1)
            .ok_or_else(|| {
                tracing::warn!("Missing hash parameter in Telegram raw data");
                ()
            })
            .ok()?;

        params.sort_by(|a, b| a.0.cmp(&b.0));

        let data_check_string = params
            .iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<_>>()
            .join("\n");

        let mut secret_key_mac = Hmac::<Sha256>::new_from_slice(b"WebAppData").ok()?;
        secret_key_mac.update(telegram_token.as_bytes());
        let secret_key = secret_key_mac.finalize().into_bytes();

        let mut mac = Hmac::<Sha256>::new_from_slice(&secret_key).ok()?;
        mac.update(data_check_string.as_bytes());
        let calculated_hash = hex::encode(mac.finalize().into_bytes());

        if calculated_hash == received_hash {
            tracing::debug!("Telegram raw validation successful");
            let user_str = params
                .into_iter()
                .find(|(key, _)| key == "user")
                .map(|(_, value)| value)?;
            serde_json::from_str::<TelegramUser>(&user_str).ok()
        } else {
            None
        }
    })();

    if let Some(validation_result) = validation_result {
        Ok(validation_result)
    } else {
        return Err(dto::Error::InvalidTelegramData);
    }
}

#[derive(Debug, Deserialize)]
pub struct TelegramUser {
    pub id: i64,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub photo_url: Option<String>,
}
