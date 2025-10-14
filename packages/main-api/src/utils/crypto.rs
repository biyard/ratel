use crate::sqlx::types::time::OffsetDateTime;
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as B64;
use hmac::{Hmac, Mac};
use openssl::{hash::MessageDigest, pkey::PKey, sign::Verifier};
use rand::Rng;
use serde_json::Value;
use sha2::Sha512;
use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};
use std::time::{Duration, Instant};

type CertList = Vec<(String, String)>;

static CERT_CACHE: OnceLock<RwLock<HashMap<(String, String), (Instant, CertList)>>> =
    OnceLock::new();
fn cert_cache() -> &'static RwLock<HashMap<(String, String), (Instant, CertList)>> {
    CERT_CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

fn compose_sign_content(ts: &str, nonce: &str, payload: &str) -> String {
    format!("{ts}\n{nonce}\n{payload}\n")
}

fn verify_rsa_sha256(cert_pem: &str, payload: &[u8], sig_b64: &str) -> Result<bool, String> {
    let pkey = PKey::public_key_from_pem(cert_pem.as_bytes())
        .map_err(|e| format!("load pem failed: {e}"))?;
    let sig = B64
        .decode(sig_b64.as_bytes())
        .map_err(|e| format!("b64 decode failed: {e}"))?;
    let mut verifier = Verifier::new(MessageDigest::sha256(), &pkey)
        .map_err(|e| format!("verifier init failed: {e}"))?;
    verifier
        .update(payload)
        .map_err(|e| format!("verifier update failed: {e}"))?;
    verifier
        .verify(&sig)
        .map_err(|e| format!("verifier verify failed: {e}"))
}

async fn fetch_all_cert_pems(base: &str, api_key: &str, secret: &str) -> Result<CertList, String> {
    use crate::reqwest::Client;

    {
        let guard = cert_cache().read().unwrap();
        if let Some((ts, list)) = guard.get(&(base.to_string(), api_key.to_string())) {
            if ts.elapsed() < Duration::from_secs(600) && !list.is_empty() {
                return Ok(list.clone());
            }
        }
    }

    let body = serde_json::json!({});
    let (ts, nonce, sign) =
        sign_for_binance(secret, &body).map_err(|e| format!("sign_for_binance failed: {e}"))?;

    let url = format!("{}/certificates", base);
    let resp = Client::new()
        .post(url)
        .header("content-type", "application/json")
        .header("BinancePay-Timestamp", &ts)
        .header("BinancePay-Nonce", &nonce)
        .header("BinancePay-Certificate-SN", api_key)
        .header("BinancePay-Signature", &sign)
        .body(body.to_string())
        .send()
        .await
        .map_err(|e| format!("request failed: {e}"))?;

    let status = resp.status();
    let text = resp
        .text()
        .await
        .map_err(|e| format!("read body failed: {e}"))?;
    let v: Value =
        serde_json::from_str(&text).unwrap_or_else(|_| serde_json::json!({ "raw": text }));

    if !status.is_success() || v.get("status").and_then(|s| s.as_str()) != Some("SUCCESS") {
        let code = v.get("code").and_then(|x| x.as_str()).unwrap_or("UNKNOWN");
        let msg = v
            .get("errorMessage")
            .and_then(|x| x.as_str())
            .unwrap_or("no errorMessage");
        return Err(format!(
            "certificates api failed: http={status}, code={code}, msg={msg}"
        ));
    }

    let arr = v
        .get("data")
        .and_then(|d| d.as_array())
        .ok_or("no data array")?;
    let mut out = Vec::with_capacity(arr.len());
    for it in arr {
        let sn = it
            .get("certSn")
            .or_else(|| it.get("certSerial"))
            .and_then(|x| x.as_str())
            .ok_or("certSn/certSerial missing")?;
        let pem = it
            .get("certPublic")
            .and_then(|x| x.as_str())
            .ok_or("certPublic missing")?;
        if PKey::public_key_from_pem(pem.as_bytes()).is_ok() {
            out.push((sn.to_string(), pem.to_string()));
        }
    }

    {
        let mut guard = cert_cache().write().unwrap();
        guard.insert(
            (base.to_string(), api_key.to_string()),
            (Instant::now(), out.clone()),
        );
    }
    Ok(out)
}

pub fn sign_for_binance(
    secret: &str,
    body_json: &serde_json::Value,
) -> crate::Result<(String, String, String)> {
    let timestamp_ms = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000).to_string();

    let mut rnd = [0u8; 16];
    rand::rng().fill(&mut rnd);
    let nonce = hex::encode(rnd);

    let payload = body_json.to_string();
    let sign_content = compose_sign_content(&timestamp_ms, &nonce, &payload);

    let mut mac = Hmac::<Sha512>::new_from_slice(secret.as_bytes())
        .map_err(|e| crate::Error::HMacInitError(format!("HMAC init err: {:?}", e)))?;
    mac.update(sign_content.as_bytes());
    let sig = mac.finalize().into_bytes();
    let signature = hex::encode_upper(sig);

    Ok((timestamp_ms, nonce, signature))
}

pub async fn verify_webhook_signature(
    base: &str,
    api_key: &str,
    secret: &str,
    ts: &str,
    nonce: &str,
    sig_b64: &str,
    cert_sn_header: Option<&str>,
    raw_body: &str,
) -> Result<(), crate::Error> {
    let payload = compose_sign_content(ts, nonce, raw_body);
    let certs = fetch_all_cert_pems(base, api_key, secret)
        .await
        .map_err(|e| crate::Error::ServerError(format!("certificates failed: {e}")))?;

    if let Some(sn) = cert_sn_header {
        if let Some(pem) = certs
            .iter()
            .find_map(|(this_sn, pem)| (this_sn == sn).then(|| pem.as_str()))
        {
            if verify_rsa_sha256(pem, payload.as_bytes(), sig_b64).unwrap_or(false) {
                return Ok(());
            } else {
                tracing::warn!("verify failed with cert_sn={}", sn);
            }
        } else {
            tracing::warn!("cert_sn from header not found in /certificates: {}", sn);
        }
    }

    for (this_sn, pem) in &certs {
        if verify_rsa_sha256(pem, payload.as_bytes(), sig_b64).unwrap_or(false) {
            tracing::debug!("verify success with cert_sn={}", this_sn);
            return Ok(());
        }
    }
    Err(crate::Error::Unauthorized("signature verification failed".to_string()))
}

pub fn apply_binance_headers(
    rb: crate::reqwest::RequestBuilder,
    api_key: &str,
    ts: &str,
    nonce: &str,
    signature: &str,
) -> crate::reqwest::RequestBuilder {
    rb.header("content-type", "application/json")
        .header("BinancePay-Timestamp", ts)
        .header("BinancePay-Nonce", nonce)
        .header("BinancePay-Certificate-SN", api_key)
        .header("BinancePay-Signature", signature)
}
