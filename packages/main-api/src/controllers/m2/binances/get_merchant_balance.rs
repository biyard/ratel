use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    Result,
    by_axum::auth::Authorization,
    sqlx::{Pool, Postgres},
};

use crate::config;
use crate::utils::crypto::{apply_binance_headers, sign_for_binance};

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
    pub wallet: String,
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

pub async fn binance_merchant_balance_handler(
    Extension(_auth): Extension<Option<Authorization>>,
    State(_pool): State<Pool<Postgres>>,
    Json(req): Json<MerchantBalanceRequest>,
) -> Result<Json<MerchantBalanceResponse>> {
    let conf = config::get();

    let base = conf.binance.base_url;
    let api_key = conf.binance.api_key;
    let secret = conf.binance.secret_key;

    let body_value = serde_json::json!({
        "wallet": req.wallet,
        "currency": req.currency
    });

    let (ts, nonce, signature) = sign_for_binance(&secret, &body_value)?;

    let client = reqwest::Client::new();
    let url = format!("{}/v2/balance", base);

    let resp = apply_binance_headers(client.post(url), &api_key, &ts, &nonce, &signature)
        .body(body_value.to_string())
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
