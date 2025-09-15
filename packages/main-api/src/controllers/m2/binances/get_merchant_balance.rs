use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    Result,
    by_axum::auth::Authorization,
    sqlx::{Pool, Postgres},
};
use hmac::{Hmac, Mac};
use rand::{Rng, distr::Alphanumeric, rng};
use sha2::Sha512;

use crate::config;
use crate::sqlx::types::time::OffsetDateTime;

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct MerchantBalanceRequest {
    #[schemars(description = "FUNDING_WALLET or SPOT_WALLET")]
    pub wallet: String,
    #[schemars(description = "USDT")]
    pub currency: String,
}

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct MerchantBalanceResponse {
    pub balance: f64,
}

fn sign_sha512_upper_hex(secret: &str, payload: &str) -> String {
    let mut mac = Hmac::<Sha512>::new_from_slice(secret.as_bytes()).expect("hmac key");
    mac.update(payload.as_bytes());
    let sig = mac.finalize().into_bytes();
    hex::encode_upper(sig)
}

fn gen_nonce_32_alnum() -> String {
    rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

pub async fn binance_merchant_balance_handler(
    Extension(_auth): Extension<Option<Authorization>>,
    State(_pool): State<Pool<Postgres>>,
    Json(req): Json<MerchantBalanceRequest>,
) -> Result<Json<MerchantBalanceResponse>> {
    let conf = config::get();

    let base = conf.binance.base_url;
    let api_key = conf.binance.api_key;
    let secret = conf.binance.secret_key;

    let body_json = serde_json::json!({
        "wallet": req.wallet,
        "currency": req.currency
    })
    .to_string();

    let ts = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000).to_string();
    let nonce = gen_nonce_32_alnum();

    let sign_payload = format!(
        "{ts}\n{nonce}\n{body}\n",
        ts = ts,
        nonce = nonce,
        body = body_json
    );
    let signature = sign_sha512_upper_hex(&secret, &sign_payload);

    let client = reqwest::Client::new();
    let url = format!("{}/v2/balance", base);

    let resp = client
        .post(url)
        .header("content-type", "application/json")
        .header("BinancePay-Timestamp", &ts)
        .header("BinancePay-Nonce", &nonce)
        .header("BinancePay-Certificate-SN", api_key)
        .header("BinancePay-Signature", &signature)
        .body(body_json)
        .send()
        .await
        .map_err(|e| dto::Error::ServerError(e.to_string()))?;

    let status = resp.status();
    let text = resp
        .text()
        .await
        .map_err(|e| dto::Error::ServerError(format!("binance balance read body failed: {e:?}")))?;

    let json: serde_json::Value =
        serde_json::from_str(&text).unwrap_or_else(|_| serde_json::json!({ "raw": text }));

    let ok = status.is_success() && json.get("status").and_then(|v| v.as_str()) == Some("SUCCESS");

    if !ok {
        let code = json
            .get("code")
            .and_then(|v| v.as_str())
            .unwrap_or("UNKNOWN");
        let msg = json
            .get("errorMessage")
            .and_then(|v| v.as_str())
            .unwrap_or("no errorMessage");
        return Err(dto::Error::ServerError(format!(
            "binance v2/balance failed: http={status}, code={code}, msg={msg}, body={json}"
        )));
    }

    let data = &json["data"];
    tracing::debug!("balance data: {:?}", data);

    fn as_f64_any(v: &serde_json::Value) -> Option<f64> {
        v.as_f64()
            .or_else(|| v.as_str().and_then(|s| s.parse::<f64>().ok()))
    }

    let balance = if let Some(arr) = data.get("balance").and_then(|v| v.as_array()) {
        let want = req.currency.as_str();
        let item = arr
            .iter()
            .find(|it| it.get("asset").and_then(|v| v.as_str()) == Some(want))
            .or_else(|| arr.first());

        item.and_then(|it| it.get("available"))
            .and_then(as_f64_any)
            .unwrap_or(0.0)
    } else {
        0.0
    };

    Ok(Json(MerchantBalanceResponse { balance }))
}
