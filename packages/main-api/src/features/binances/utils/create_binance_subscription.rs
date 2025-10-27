use crate::Error;
use crate::config;
use crate::features::binances::PurchaseMembershipResponse;
use crate::features::membership::Membership;
use crate::features::membership::MembershipTier;
use crate::types::Partition;
use crate::utils::crypto::sign_for_binance;
use crate::utils::generate_merchant_trade_no::gen_merchant_trade_no;
use bdk::prelude::*;

pub async fn create_binance_subscription(
    user_pk: Partition,
    membership: Membership,
) -> Result<PurchaseMembershipResponse, Error> {
    let user_id = match user_pk {
        Partition::User(v) | Partition::Team(v) => v.to_string(),
        _ => "".to_string(),
    };

    let membership_id = match membership.pk {
        Partition::Membership(v) => v.to_string(),
        _ => "".to_string(),
    };

    let conf = config::get();

    let base = conf.binance.base_url;
    let api_key = conf.binance.api_key;
    let secret = conf.binance.secret_key;
    let binance_webhook = conf.binance.webhook;

    let base_domain = conf.binance.redirect_domain;
    let tier = membership.tier;

    if tier == MembershipTier::Free {
        return Ok(PurchaseMembershipResponse {
            checkout_url: "".to_string(),
            deeplink: "".to_string(),
            prepay_id: "".to_string(),
            qr_content: "".to_string(),
            qrcode_link: "".to_string(),
        });
    }

    let amount_usdt = membership.price_dollars;

    let merchant_trade_no = gen_merchant_trade_no(&tier.to_string());
    tracing::debug!(
        "merchant no: {:?} {:?} {:?}",
        merchant_trade_no,
        base_domain,
        binance_webhook
    );
    let merchant_contract_code = format!("contract_{}", merchant_trade_no);

    let body = serde_json::json!({
      "env": { "terminalType": "WEB" },
      "merchantTradeNo": merchant_trade_no,
      "orderAmount": amount_usdt,
      "currency": "USDT",
      "description": format!("{} Subscription", tier.to_string()),
      "goodsDetails": [{
        "goodsType": "02", "goodsCategory": "Z000",
        "referenceGoodsId": tier.to_string(), "goodsName": tier.to_string()
      }],
      "returnUrl": base_domain,
      "cancelUrl": base_domain,
      "webhookUrl": binance_webhook,

      "passThroughInfo": serde_json::json!({
          "env": conf.env,
          "plan": tier.to_string(),
          "userId": user_id,
          "membershipId": membership_id,
      }).to_string(),
      "directDebitContract": {
        "merchantContractCode": merchant_contract_code,
        "serviceName": "Ratel",
        "scenarioCode": "SUBSCRIPTION",
        "singleUpperLimit": amount_usdt,
        "periodic": true,
        "cycleDebitFixed": true,
        "cycleType": "MONTH",
        "cycleValue": 1,
     }
    });

    let (timestamp_ms, nonce, signature) = sign_for_binance(secret, &body)?;
    let url = format!("{}/v3/order", base);

    let resp = crate::utils::crypto::apply_binance_headers(
        reqwest::Client::new().post(url),
        api_key,
        &timestamp_ms,
        &nonce,
        &signature,
    )
    .body(body.to_string())
    .send()
    .await
    .map_err(|e| Error::ServerError(e.to_string()))?;

    let status = resp.status();
    let text = resp
        .text()
        .await
        .map_err(|e| Error::ServerError(format!("read body failed: {e:?}")))?;
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
        return Err(Error::ServerError(format!(
            "binance v3/order failed: http={status}, code={code}, msg={msg}, body={json}"
        )));
    }

    let data = &json["data"];

    let out = PurchaseMembershipResponse {
        checkout_url: data["checkoutUrl"].as_str().unwrap_or_default().to_string(),
        deeplink: data["deeplink"].as_str().unwrap_or_default().to_string(),
        prepay_id: data["prepayId"].as_str().unwrap_or_default().to_string(),
        qr_content: data["qrContent"].as_str().unwrap_or_default().to_string(),
        qrcode_link: data["qrcodeLink"].as_str().unwrap_or_default().to_string(),
    };

    Ok(out)
}
