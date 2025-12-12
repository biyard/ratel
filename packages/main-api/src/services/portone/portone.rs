use super::*;
use crate::{
    utils::time::{after_days_from_now_rfc_3339, get_now_timestamp_micros},
    *,
};

use percent_encoding::{AsciiSet, NON_ALPHANUMERIC};

const BASE_URL: &str = "https://api.portone.io";
const CHARSET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-";

#[derive(Debug, Clone)]
pub struct PortOne {
    cli: reqwest::Client,
}

impl PortOne {
    pub fn new(api_secret: &str) -> Self {
        let cli = reqwest::Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    "Authorization",
                    reqwest::header::HeaderValue::from_str(&format!("PortOne {}", api_secret))
                        .unwrap(),
                );
                headers
            })
            .build()
            .unwrap();
        Self { cli }
    }

    pub async fn identify(&self, id: &str) -> Result<IdentifyResponse> {
        let res = self
            .cli
            .get(format!(
                "{}/identity-verifications/{}",
                BASE_URL,
                percent_encoding::utf8_percent_encode(id, NON_ALPHANUMERIC)
            ))
            .send()
            .await?;
        let j: serde_json::Value = res.json().await?;
        warn!("PortOne identify response: {:?}", j);
        Ok(serde_json::from_value(j)?)
    }

    pub async fn get_billing_key(
        &self,
        customer_id: String,
        customer_name: String,
        card_number: String,
        expiry_year: String,
        expiry_month: String,
        birth_or_business_registration_number: String,
        password_two_digits: String,
    ) -> Result<BillingKeyResponse> {
        let portone_conf = config::get().portone;

        let body = BillingKeyRequest {
            store_id: portone_conf.store_id.to_string(),
            channel_key: portone_conf.kpn_channel_key.to_string(),
            customer: CustomerRequest {
                id: customer_id,
                name: CustomerName {
                    full: customer_name,
                },
            },
            method: MethodRequest {
                card: CardRequest {
                    credential: CardCredentialRequest {
                        number: card_number,
                        expiry_year,
                        expiry_month,
                        birth_or_business_registration_number,
                        password_two_digits,
                    },
                },
            },
        };

        let res = self
            .cli
            .post(format!("{}/billing-keys", BASE_URL))
            .json(&body)
            .send()
            .await?;

        Ok(res.json().await?)
    }

    pub async fn pay_with_billing_key(
        &self,
        customer_id: String,
        customer_name: String,
        order_name: String,
        billing_key: String,
        amount: i64,
        currency: Currency,
    ) -> Result<(BillingKeyPaymentResponse, String)> {
        let portone_conf = config::get().portone;
        let payment_id = format!(
            "{}-{}",
            random_string::generate(10, CHARSET),
            get_now_timestamp_micros()
        );

        let body = BillingKeyPaymentRequest {
            store_id: portone_conf.store_id.to_string(),
            channel_key: portone_conf.kpn_channel_key.to_string(),
            billing_key,
            order_name,
            customer: CustomerRequest {
                id: customer_id,
                name: CustomerName {
                    full: customer_name,
                },
            },
            amount: PaymentAmountInput {
                total: amount,
                tax_free: None,
                vat: None,
            },
            currency: currency.to_string(),
            locale: None,
        };

        let res = self
            .cli
            .post(format!("{}/payments/{}/billing-key", BASE_URL, payment_id))
            .json(&body)
            .send()
            .await?;

        Ok((res.json().await?, payment_id))
    }

    pub async fn schedule_pay_with_billing_key(
        &self,
        customer_id: String,
        customer_name: String,
        order_name: String,
        billing_key: String,
        amount: i64,
        currency: Currency,
        time_to_pay: String,
    ) -> Result<(BillingKeyPaymentResponse, String)> {
        let portone_conf = config::get().portone;
        let payment_id = format!(
            "{}-{}",
            random_string::generate(10, CHARSET),
            get_now_timestamp_micros(),
        );

        let payment = BillingKeyPaymentRequest {
            store_id: portone_conf.store_id.to_string(),
            channel_key: portone_conf.kpn_channel_key.to_string(),
            billing_key,
            order_name,
            customer: CustomerRequest {
                id: customer_id,
                name: CustomerName {
                    full: customer_name,
                },
            },
            amount: PaymentAmountInput {
                total: amount,
                tax_free: None,
                vat: None,
            },
            currency: currency.to_string(),
            locale: None,
        };

        let body = ScheduleBillingKeyRequest {
            payment,
            time_to_pay,
        };

        let res = self
            .cli
            .post(format!("{}/payments/{}/schedule", BASE_URL, payment_id))
            .json(&body)
            .send()
            .await?;

        Ok((res.json().await?, payment_id))
    }
}
