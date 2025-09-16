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

    let biz_status = parse_biz_status(&raw_body).unwrap_or_default();
    tracing::debug!("biz status: {:?}", biz_status);

    if biz_status != "PAY_SUCCESS" {
        tracing::info!("binance webhook ignored: biz_status={}", biz_status);
        return Ok(Json(serde_json::json!({
            "returnCode": "SUCCESS",
            "returnMessage": "OK"
        })));
    }

    let (user_id, plan, binance_user) = parse_ids_and_plan(&raw_body)
        .ok_or_else(|| dto::Error::ServerError("cannot parse userId/plan from payload".into()))?;

    tracing::info!(
        "binance webhook user_id={:?}, plan={:?}, binance_user={:?}",
        user_id,
        plan,
        binance_user
    );

    if user_id.is_none() {
        return Err(dto::Error::NotFound);
    }

    let user_id: i64 = user_id
        .unwrap_or_default()
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

fn parse_biz_status(raw_body: &str) -> Option<String> {
    let v: Value = serde_json::from_str(raw_body).ok()?;
    v.get("bizStatus")
        .and_then(|x| x.as_str().map(|s| s.to_string()))
}

fn parse_ids_and_plan(raw_body: &str) -> Option<(Option<String>, String, Option<String>)> {
    let v: Value = serde_json::from_str(raw_body).ok()?;
    let data_raw = v.get("data")?;
    let data: Value = match data_raw {
        Value::String(s) => serde_json::from_str(s).ok()?,
        Value::Object(_) => data_raw.clone(),
        _ => return None,
    };

    let plan = data
        .get("productName")
        .and_then(|x| x.as_str())
        .map(|s| s.to_string())
        .or_else(|| {
            data.get("referenceGoodsId")
                .and_then(|x| x.as_str().map(|s| s.to_string()))
        })
        .unwrap_or_else(|| "UNKNOWN".to_string());

    let binance_user = data
        .get("openUserId")
        .and_then(|x| x.as_str().map(|s| s.to_string()));

    let attach_obj = data
        .get("passThroughInfo")
        .and_then(|att| match att {
            Value::String(s) => serde_json::from_str::<Value>(s).ok(),
            Value::Object(_) => Some(att.clone()),
            _ => None,
        })
        .or_else(|| {
            data.get("merchantAttach").and_then(|att| match att {
                Value::String(s) => serde_json::from_str::<Value>(s).ok(),
                Value::Object(_) => Some(att.clone()),
                _ => None,
            })
        });

    let user_id = attach_obj.as_ref().and_then(|inner| {
        inner.get("userId").and_then(|x| {
            x.as_str()
                .map(|s| s.to_string())
                .or_else(|| x.as_i64().map(|n| n.to_string()))
        })
    });

    Some((user_id, plan, binance_user))
}
