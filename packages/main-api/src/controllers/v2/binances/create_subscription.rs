use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    Result,
    by_axum::auth::Authorization,
    sqlx::{Pool, Postgres},
};
use rand::{Rng, distr::Alphanumeric};

use crate::{config, utils::wallets::sign_for_binance::sign_for_binance};

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
pub struct SubscribeRequest {
    // #[schemars(description = "User Email Address")]
    // pub user_account: String, // exp: user123@ratel.app
    #[schemars(description = "Subscribe Plan Title (ex) RATEL_PRO)")]
    pub plan_code: String, // exp: RATEL_PRO
    #[schemars(description = "Subscription Fee")]
    pub amount_usdt: f64, // subscription fee
                          // #[schemars(description = "Start Subscription Date")]
                          // pub start_ms_utc: i64, // firstDeductTime
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
pub struct SubscribeResponse {
    pub qrcode_link: String,
    pub qr_content: String,
    pub checkout_url: String,
    pub deeplink: String,
    pub prepay_id: String,
}

pub async fn create_subscription_handler(
    Extension(_auth): Extension<Option<Authorization>>,
    State(_pool): State<Pool<Postgres>>,
    Json(req): Json<SubscribeRequest>,
) -> Result<Json<SubscribeResponse>> {
    let conf = config::get();

    let base = conf.binance_base_url;
    let api_key = conf.binance_api_key;
    let secret = conf.binance_secret_key;

    // let mut rnd = [0u8; 6];
    // rand::rng().fill(&mut rnd);
    // let rnd_tag = hex::encode(rnd);

    let merchant_trade_no = gen_merchant_trade_no(&req.plan_code);
    tracing::debug!("merchant no: {:?}", merchant_trade_no);
    // let merchant_contract_code = format!("contract_{}", rnd_tag);

    let body = serde_json::json!({
      "env": { "terminalType": "WEB" },
      "merchantTradeNo": merchant_trade_no,
      "orderAmount": req.amount_usdt,
      "currency": "USDT",
      "description": format!("{} Subscription", req.plan_code),
      "goodsDetails": [{
        "goodsType": "02", "goodsCategory": "Z000",
        "referenceGoodsId": req.plan_code, "goodsName": req.plan_code
      }],
    //   "directDebitContract": { "merchantContractCode": merchant_contract_code, "serviceName": "Ratel Pro", "scenarioCode": "SUBSCRIPTION", "singleUpperLimit": 200.0, "periodic": true, "cycleDebitFixed": true, "cycleType": "MONTH", "cycleValue": 1, "firstDeductTime": req.start_ms_utc, "merchantAccountNo": req.user_account }
    });

    let (timestamp_ms, nonce, signature) = sign_for_binance(secret, &body)?;

    let client = reqwest::Client::new();
    let url = format!("{}/v3/order", base);

    let resp = client
        .post(url)
        .header("content-type", "application/json")
        .header("BinancePay-Timestamp", &timestamp_ms)
        .header("BinancePay-Nonce", &nonce)
        .header("BinancePay-Certificate-SN", api_key)
        .header("BinancePay-Signature", &signature)
        .body(body.to_string())
        .send()
        .await
        .map_err(|e| dto::Error::ServerError(e.to_string()))?;

    let status = resp.status();
    let text = resp
        .text()
        .await
        .map_err(|e| dto::Error::ServerError(format!("read body failed: {e:?}")))?;
    let json: serde_json::Value =
        serde_json::from_str(&text).unwrap_or_else(|_| serde_json::json!({ "raw": text }));

    if !status.is_success() || json.get("status").and_then(|v| v.as_str()) != Some("SUCCESS") {
        let code = json
            .get("code")
            .and_then(|v| v.as_str())
            .unwrap_or("UNKNOWN");
        let msg = json
            .get("errorMessage")
            .and_then(|v| v.as_str())
            .unwrap_or("no errorMessage");
        return Err(dto::Error::ServerError(format!(
            "binance v3/order failed: http={status}, code={code}, msg={msg}, body={json}"
        )));
    }

    let data = &json["data"];
    let out = SubscribeResponse {
        checkout_url: data["checkoutUrl"].as_str().unwrap_or_default().to_string(),
        deeplink: data["deeplink"].as_str().unwrap_or_default().to_string(),
        prepay_id: data["prepayId"].as_str().unwrap_or_default().to_string(),
        qr_content: data["qrContent"].as_str().unwrap_or_default().to_string(),
        qrcode_link: data["qrcodeLink"].as_str().unwrap_or_default().to_string(),
    };

    Ok(Json(out))
}

fn sanitize_alnum(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect()
}

fn gen_merchant_trade_no(plan_code: &str) -> String {
    let base = sanitize_alnum(plan_code);
    let rand_tag: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    let mut mt = format!("{}{}", base, rand_tag);
    if mt.len() > 32 {
        mt.truncate(32);
    }
    if mt.is_empty() {
        mt = rand_tag;
    }
    mt
}
