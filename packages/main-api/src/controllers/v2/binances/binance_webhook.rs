use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as B64;
use bdk::prelude::*;
use by_axum::axum::{Extension, Json, body::Bytes, extract::State, http::HeaderMap};
use dto::{
    Membership, Purchase, PurchaseRepositoryUpdateRequest, PurchaseStatus, Result, User,
    UserRepositoryUpdateRequest,
    by_axum::auth::Authorization,
    sqlx::{Pool, Postgres},
};
use openssl::{hash::MessageDigest, pkey::PKey, sign::Verifier};
use serde_json::Value;

use crate::{config, utils::wallets::sign_for_binance::sign_for_binance};

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
    let base = conf.binance_base_url;
    let api_key = conf.binance_api_key;
    let secret = conf.binance_secret_key;

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
    let payload = format!("{ts}\n{nonce}\n{raw_body}\n");
    tracing::debug!("payload composed ({} bytes)", payload.len());

    let certs = fetch_all_cert_pems(&base, &api_key, &secret)
        .await
        .map_err(|e| dto::Error::ServerError(format!("certificates failed: {e}")))?;
    tracing::debug!("fetched {} cert(s)", certs.len());

    let mut tried = 0usize;
    let mut verified = false;

    if let Some(sn) = cert_sn {
        if let Some(pem) = certs.iter().find_map(|(this_sn, pem)| {
            if this_sn == sn {
                Some(pem.as_str())
            } else {
                None
            }
        }) {
            tried += 1;
            if verify_rsa_sha256(pem, payload.as_bytes(), sig_b64).unwrap_or(false) {
                verified = true;
            } else {
                tracing::warn!("verify failed with cert_sn={}", sn);
            }
        } else {
            tracing::warn!("cert_sn from header not found in /certificates: {}", sn);
        }
    }

    if !verified {
        for (this_sn, pem) in &certs {
            tried += 1;
            if verify_rsa_sha256(pem, payload.as_bytes(), sig_b64).unwrap_or(false) {
                tracing::debug!("verify success with cert_sn={}", this_sn);
                verified = true;
                break;
            }
        }
    }

    tracing::debug!("verify result: {}, tried {} cert(s)", verified, tried);
    if !verified {
        return Err(dto::Error::Unauthorized);
    }

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

async fn fetch_all_cert_pems(
    base: &str,
    api_key: &str,
    secret: &str,
) -> std::result::Result<Vec<(String, String)>, String> {
    let body = serde_json::json!({});
    let (ts, nonce, sign) =
        sign_for_binance(secret, &body).map_err(|e| format!("sign_for_binance failed: {e}"))?;
    let url = format!("{}/certificates", base);

    let client = reqwest::Client::new();
    let resp = client
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
    tracing::debug!("certificates http={} body={}", status, text);

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
        } else {
            tracing::warn!("invalid PUBLIC KEY PEM for cert serial={}", sn);
        }
    }
    Ok(out)
}

fn verify_rsa_sha256(
    cert_pem: &str,
    payload: &[u8],
    sig_b64: &str,
) -> std::result::Result<bool, String> {
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
