use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    Purchase, Result,
    by_axum::auth::Authorization,
    sqlx::{Pool, Postgres},
};

use crate::{
    config,
    utils::{
        generate_merchant_trade_no::gen_merchant_trade_no, users::extract_user_id,
        wallets::sign_for_binance::sign_for_binance,
    },
};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    Default,
    dto::JsonSchema,
)]
#[serde(rename_all = "lowercase")]
pub enum SubscribeType {
    #[default]
    Free = 1,
    Pro = 2,
    Premium = 3,
    Vip = 4,
    Admin = 5,
}

impl SubscribeType {
    pub const fn plan_code(self) -> &'static str {
        match self {
            SubscribeType::Free => "RATEL_FREE",
            SubscribeType::Pro => "RATEL_PRO",
            SubscribeType::Premium => "RATEL_PREMIUM",
            SubscribeType::Vip => "RATEL_VIP",
            SubscribeType::Admin => "RATEL_ADMIN",
        }
    }
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
pub struct SubscribeRequest {
    // #[schemars(description = "User Email Address")]
    // pub user_account: String, // exp: user123@ratel.app
    #[schemars(description = "Subscribe Type (1: Personal, 2: Business, 3: Enterprise)")]
    pub subscribe_type: SubscribeType,
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
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<Pool<Postgres>>,
    Json(req): Json<SubscribeRequest>,
) -> Result<Json<SubscribeResponse>> {
    let repo = Purchase::get_repository(pool.clone());
    let user_id = extract_user_id(&pool, auth).await?;

    let conf = config::get();

    let base = conf.binance.base_url;
    let api_key = conf.binance.api_key;
    let secret = conf.binance.secret_key;
    let binance_webhook = conf.binance.webhook;

    let base_domain = conf.binance.redirect_domain;

    let plan_code = req.subscribe_type.plan_code();

    if req.subscribe_type == SubscribeType::Free || req.subscribe_type == SubscribeType::Admin {
        return Ok(Json(SubscribeResponse {
            checkout_url: "".to_string(),
            deeplink: "".to_string(),
            prepay_id: "".to_string(),
            qr_content: "".to_string(),
            qrcode_link: "".to_string(),
        }));
    }

    let amount_usdt = if req.subscribe_type == SubscribeType::Pro {
        20
    } else if req.subscribe_type == SubscribeType::Premium {
        50
    } else {
        100
    };

    // let mut rnd = [0u8; 6];
    // rand::rng().fill(&mut rnd);
    // let rnd_tag = hex::encode(rnd);

    let merchant_trade_no = gen_merchant_trade_no(&plan_code);
    tracing::debug!(
        "merchant no: {:?} {:?} {:?}",
        merchant_trade_no,
        base_domain,
        binance_webhook
    );
    // let merchant_contract_code = format!("contract_{}", rnd_tag);

    let body = serde_json::json!({
      "env": { "terminalType": "WEB" },
      "merchantTradeNo": merchant_trade_no,
      "orderAmount": amount_usdt,
      "currency": "USDT",
      "description": format!("{} Subscription", plan_code),
      "goodsDetails": [{
        "goodsType": "02", "goodsCategory": "Z000",
        "referenceGoodsId": plan_code, "goodsName": plan_code
      }],
      "returnUrl": base_domain,
      "cancelUrl": base_domain,
      "webhookUrl": binance_webhook,

      "passThroughInfo": serde_json::json!({
          "env": conf.env,
          "plan": plan_code,
          "userId": user_id,
      }).to_string(),
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
    let db_prepay_id: String = data["prepayId"].as_str().unwrap_or_default().to_string();

    let _ = repo
        .insert(user_id, dto::PurchaseStatus::InProgress, Some(db_prepay_id))
        .await?;

    let out = SubscribeResponse {
        checkout_url: data["checkoutUrl"].as_str().unwrap_or_default().to_string(),
        deeplink: data["deeplink"].as_str().unwrap_or_default().to_string(),
        prepay_id: data["prepayId"].as_str().unwrap_or_default().to_string(),
        qr_content: data["qrContent"].as_str().unwrap_or_default().to_string(),
        qrcode_link: data["qrcodeLink"].as_str().unwrap_or_default().to_string(),
    };

    Ok(Json(out))
}
