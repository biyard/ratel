use crate::config;
use crate::utils::crypto::verify_webhook_signature;
use bdk::prelude::*;
use by_axum::axum::{Extension, Json, body::Bytes, extract::State, http::HeaderMap};
use dto::{
    Membership, Purchase, PurchaseRepositoryUpdateRequest, PurchaseStatus, Result, User,
    UserRepositoryUpdateRequest,
    by_axum::auth::Authorization,
    sqlx::{Pool, Postgres},
};
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

pub async fn binance_webhook_handler(
    Extension(_auth): Extension<Option<Authorization>>,
    State(pool): State<Pool<Postgres>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Json<Value>> {
    tracing::debug!("webhook binance called with body: {:?}", body);

    let repo = User::get_repository(pool.clone());
    let purchase_repo = Purchase::get_repository(pool.clone());
    let mut tx = pool.begin().await?;

    let conf = config::get();
    let base = conf.binance.base_url;
    let api_key = conf.binance.api_key;
    let secret = conf.binance.secret_key;

    let ts = get_header(&headers, "BinancePay-Timestamp")?;
    let nonce = get_header(&headers, "BinancePay-Nonce")?;
    let sig_b64 = get_header(&headers, "BinancePay-Signature")?;
    let cert_sn = headers
        .get("BinancePay-Cert-SN")
        .or_else(|| headers.get("X-BinancePay-Cert-SN"))
        .and_then(|v| v.to_str().ok());

    tracing::debug!(
        "headers ts={ts}, nonce={nonce}, sig(len)={}, cert_sn={:?}",
        sig_b64.len(),
        cert_sn
    );

    let raw_body = String::from_utf8_lossy(&body);

    verify_webhook_signature(
        &base, &api_key, &secret, ts, nonce, sig_b64, cert_sn, &raw_body,
    )
    .await?;

    let notif: BinancePayNotification = serde_json::from_str(raw_body.as_ref())
        .map_err(|e| dto::Error::ServerError(format!("invalid webhook payload: {e}")))?;

    tracing::debug!("parsed notif: {:?}", notif);

    if notif.biz_status != BizStatus::PaySuccess {
        tracing::info!("binance webhook ignored: biz_status={:?}", notif.biz_status);
        return Ok(Json(serde_json::json!({
            "returnCode": "SUCCESS",
            "returnMessage": "OK"
        })));
    }

    let plan = notif
        .data
        .product_name
        .clone()
        .or(notif.data.reference_goods_id.clone())
        .unwrap_or_else(|| "UNKNOWN".to_string());

    let binance_user = notif.data.open_user_id.clone();

    let user_id = extract_user_id_from_data(&notif.data)
        .ok_or_else(|| dto::Error::ServerError("cannot parse userId from payload".into()))?;

    tracing::info!(
        "binance webhook user_id={:?}, plan={:?}, binance_user={:?}",
        user_id,
        plan,
        binance_user
    );

    let user_id: i64 = user_id
        .parse::<i64>()
        .map_err(|e| dto::Error::ServerError(format!("invalid user_id: {e}")))?;

    let subscribe_type = if plan == "RATEL_PRO" {
        Membership::Paid1
    } else if plan == "RATEL_PREMIUM" {
        Membership::Paid2
    } else {
        Membership::Paid3
    };

    let d = Purchase::query_builder()
        .order_by_created_at_desc()
        .user_id_equals(user_id)
        .query()
        .map(Purchase::from)
        .fetch_one(&mut *tx)
        .await?;

    let _ = repo
        .update_with_tx(
            &mut *tx,
            user_id,
            UserRepositoryUpdateRequest {
                membership: Some(subscribe_type),
                ..Default::default()
            },
        )
        .await?;

    let _ = purchase_repo
        .update_with_tx(
            &mut *tx,
            d.id,
            PurchaseRepositoryUpdateRequest {
                status: Some(PurchaseStatus::Purchased),
                ..Default::default()
            },
        )
        .await?;

    tx.commit().await?;

    Ok(Json(serde_json::json!({
        "returnCode": "SUCCESS",
        "returnMessage": "OK",
        "userId": user_id,
        "plan": plan,
        "binanceUserId": binance_user
    })))
}

fn get_header<'a>(headers: &'a HeaderMap, key: &str) -> Result<&'a str> {
    headers
        .get(key)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| dto::Error::ServerError(format!("missing header: {key}")))
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum BizStatus {
    PaySuccess,
    PayFail,
    Pending,
    OrderCancelled,
    OrderClosed,
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum BizType {
    Pay,
    Refund,
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct NotificationData {
    #[serde(default)]
    product_name: Option<String>,
    #[serde(default)]
    reference_goods_id: Option<String>,
    #[serde(default)]
    open_user_id: Option<String>,

    #[serde(default, deserialize_with = "deserialize_maybe_string_value")]
    pass_through_info: Option<Value>,
    #[serde(default, deserialize_with = "deserialize_maybe_string_value")]
    merchant_attach: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct BinancePayNotification {
    biz_status: BizStatus,
    biz_type: BizType,
    biz_id: Option<i64>,
    biz_id_str: Option<String>,

    #[serde(deserialize_with = "deserialize_notification_data")]
    data: NotificationData,
}

fn deserialize_notification_data<'de, D>(
    deserializer: D,
) -> std::result::Result<NotificationData, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;
    match value {
        Value::String(s) => serde_json::from_str(&s).map_err(DeError::custom),
        Value::Object(_) => serde_json::from_value(value).map_err(DeError::custom),
        _ => Err(DeError::custom("unsupported data format")),
    }
}

fn deserialize_maybe_string_value<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<Value>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<Value> = Option::deserialize(deserializer)?;
    match value {
        None => Ok(None),
        Some(Value::String(s)) => {
            let v: Value = serde_json::from_str(&s).map_err(DeError::custom)?;
            Ok(Some(v))
        }
        Some(v @ Value::Object(_)) => Ok(Some(v)),
        Some(other) => Ok(Some(other)),
    }
}

fn extract_user_id_from_data(data: &NotificationData) -> Option<String> {
    fn pick(v: &Value) -> Option<String> {
        v.get("userId").and_then(|x| {
            x.as_str()
                .map(|s| s.to_string())
                .or_else(|| x.as_i64().map(|n| n.to_string()))
        })
    }
    data.pass_through_info
        .as_ref()
        .and_then(pick)
        .or_else(|| data.merchant_attach.as_ref().and_then(pick))
}
