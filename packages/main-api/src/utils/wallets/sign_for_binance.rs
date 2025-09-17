use hmac::{Hmac, Mac};
use rand::Rng;
use sha2::Sha512;
use tower_sessions::cookie::time::OffsetDateTime;

pub fn sign_for_binance(
    secret: &str,
    body_json: &serde_json::Value,
) -> dto::Result<(String, String, String)> {
    let timestamp_ms = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000).to_string();

    let mut rnd = [0u8; 16];
    rand::rng().fill(&mut rnd);
    let nonce = hex::encode(rnd);

    let payload = body_json.to_string();

    let sign_content = format!(
        "{timestamp}\n{nonce}\n{payload}\n",
        timestamp = timestamp_ms,
        nonce = nonce,
        payload = payload,
    );

    let mut mac = Hmac::<Sha512>::new_from_slice(secret.as_bytes())
        .map_err(|e| dto::Error::HMacInitError(format!("HMAC init err: {:?}", e)))?;
    mac.update(sign_content.as_bytes());
    let sig = mac.finalize().into_bytes();
    let signature = hex::encode_upper(sig);

    Ok((timestamp_ms, nonce, signature))
}
