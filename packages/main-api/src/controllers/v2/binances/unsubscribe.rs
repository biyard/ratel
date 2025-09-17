use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    Membership, Purchase, PurchaseStatus, Result, User, UserRepositoryUpdateRequest,
    by_axum::auth::Authorization,
    sqlx::{Pool, Postgres},
};
use serde_json::Value;

use crate::{
    config,
    utils::{
        generate_merchant_trade_no::gen_merchant_trade_no, users::extract_user,
        wallets::sign_for_binance::sign_for_binance,
    },
};

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
pub struct RefundResponse {
    pub refund_request_id: String,
    pub refund_amount: f64,
    pub currency: String,
    pub prepay_id: String,
    pub binance_status: String,
    pub binance_code: String,
}

//TODO: test this function when unsubscribe api is authorized
pub async fn unsubscribe_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<RefundResponse>> {
    let mut tx = pool.begin().await?;
    let user = extract_user(&pool, auth).await?;
    let user_id = user.id;
    let membership = user.membership;
    let repo = User::get_repository(pool.clone());
    let purchase_repo = Purchase::get_repository(pool.clone());

    let p = Purchase::query_builder()
        .order_by_created_at_desc()
        .user_id_equals(user_id)
        .query()
        .map(Purchase::from)
        .fetch_one(&mut *tx)
        .await?;

    let prepay_id = p.payment_id.ok_or(dto::Error::NotFound)?;
    if p.status != PurchaseStatus::Purchased {
        return Err(dto::Error::NotFound);
    }
    if matches!(membership, Membership::Free | Membership::Admin) {
        return Err(dto::Error::NotFound);
    }

    let refund_amount = match membership {
        Membership::Paid1 => 20.0,
        Membership::Paid2 => 50.0,
        _ => 100.0,
    };

    let conf = config::get();
    let base = conf.binance.base_url;
    let api_key = conf.binance.api_key;
    let secret = conf.binance.secret_key;

    let refund_request_id = gen_merchant_trade_no("REFUND");
    let body = serde_json::json!({
        "refundRequestId": refund_request_id,
        "prepayId": prepay_id,
        "refundAmount": refund_amount,
        "refundReason": "USER_REQUEST"
    });

    let (ts, nonce, sign) = sign_for_binance(&secret, &body)?;
    let url = format!("{}/order/refund", base);
    let client = reqwest::Client::new();
    let resp = client
        .post(url)
        .header("content-type", "application/json")
        .header("BinancePay-Timestamp", ts)
        .header("BinancePay-Nonce", nonce)
        .header("BinancePay-Certificate-SN", api_key)
        .header("BinancePay-Signature", sign)
        .body(body.to_string())
        .send()
        .await
        .map_err(|e| dto::Error::ServerError(e.to_string()))?;

    let status = resp.status();
    let text = resp
        .text()
        .await
        .map_err(|e| dto::Error::ServerError(e.to_string()))?;
    let v: Value =
        serde_json::from_str(&text).unwrap_or_else(|_| serde_json::json!({ "raw": text }));

    if !status.is_success() || v.get("status").and_then(|s| s.as_str()) != Some("SUCCESS") {
        let code = v.get("code").and_then(|x| x.as_str()).unwrap_or("UNKNOWN");
        let msg = v
            .get("errorMessage")
            .and_then(|x| x.as_str())
            .unwrap_or("no errorMessage");
        return Err(dto::Error::ServerError(format!(
            "binance refund failed: http={status}, code={code}, msg={msg}, body={v}"
        )));
    }

    let binance_code = v
        .get("code")
        .and_then(|x| x.as_str())
        .unwrap_or_default()
        .to_string();
    let out = RefundResponse {
        refund_request_id,
        refund_amount,
        currency: "USDT".into(),
        prepay_id,
        binance_status: "SUCCESS".into(),
        binance_code,
    };

    let _ = purchase_repo
        .insert_with_tx(&mut *tx, user_id, dto::PurchaseStatus::Refunded, None)
        .await?;

    let _ = repo
        .update_with_tx(
            &mut *tx,
            user_id,
            UserRepositoryUpdateRequest {
                membership: Some(Membership::Free),
                ..Default::default()
            },
        )
        .await?;

    tx.commit().await?;
    Ok(Json(out))
}
