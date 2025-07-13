use crate::config;
use ethers::core::k256::sha2::Sha256;
use hmac::{Hmac, Mac};
use serde::Deserialize;

// https://core.telegram.org/bots/webapps#validating-data-received-via-the-mini-app

pub fn validate_telegram_raw(telegram_raw: &Option<String>) -> Option<i64> {
    tracing::debug!("Validating Telegram raw: {:?}", telegram_raw);
    let raw = telegram_raw.as_ref().filter(|s| !s.is_empty())?;

    let config = config::get();
    let telegram_token = config.telegram_token;

    let validation_result = (|| {
        let mut params: Vec<(String, String)> = url::form_urlencoded::parse(raw.as_bytes())
            .into_owned()
            .collect();
        tracing::debug!("Telegram raw params: {:?}", params);
        let received_hash = params
            .iter()
            .position(|(key, _)| key == "hash")
            .map(|index| params.remove(index).1)?;

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

            serde_json::from_str::<TelegramUser>(&user_str)
                .map(|user| user.id)
                .ok()
        } else {
            None
        }
    })();

    validation_result
}

#[derive(Deserialize)]
struct TelegramUser {
    id: i64,
}
