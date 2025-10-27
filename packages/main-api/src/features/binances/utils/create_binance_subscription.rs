use crate::Error;
use crate::config;
use crate::features::binances::PurchaseMembershipResponse;
use crate::features::membership::Membership;
use crate::features::membership::MembershipTier;
use crate::types::Partition;
use crate::utils::crypto::sign_for_binance;
use crate::utils::generate_merchant_trade_no::gen_merchant_trade_no;
use bdk::prelude::*;
use chrono::{DateTime, Datelike, Duration, NaiveDate, TimeZone, Timelike, Utc};

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
    let merchant_contract_code = sanitize_contract_code(format!("contract{}", merchant_trade_no));
    tracing::debug!("merchant contract code: {:?}", merchant_contract_code);

    let first_deduct_ms = next_first_deduct_time_monthly_ms(Utc::now());

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
        "scenarioCode": "Membership",
        "singleUpperLimit": amount_usdt,
        "periodic": true,
        "cycleDebitFixed": true,
        "cycleType": "MONTH",
        "cycleValue": 1,
        "firstDeductTime": first_deduct_ms
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

fn next_first_deduct_time_monthly_ms(now: DateTime<Utc>) -> i64 {
    let mut year = now.year();
    let mut month = now.month() as i32;
    if month == 12 {
        year += 1;
        month = 1;
    } else {
        month += 1;
    }

    let day = now.day().min(28);

    let naive_date = NaiveDate::from_ymd_opt(year, month as u32, day).expect("valid y-m-d");

    let naive_dt = naive_date
        .and_hms_opt(now.hour(), now.minute(), 0)
        .or_else(|| naive_date.and_hms_opt(9, 0, 0))
        .expect("valid h:m:s");

    let target = Utc.from_utc_datetime(&naive_dt);

    let min_time = now + Duration::minutes(10);
    let final_time = if target <= min_time { min_time } else { target };

    final_time.timestamp_millis()
}

fn sanitize_contract_code<S: AsRef<str>>(raw: S) -> String {
    let mut s: String = raw
        .as_ref()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect();

    if s.is_empty() {
        s.push_str("RATEL");
    }

    if s.len() > 32 {
        s.truncate(32);
    }
    s
}
