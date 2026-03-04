use crate::models::Currency;
use crate::*;

use crate::config;
use percent_encoding::NON_ALPHANUMERIC;

const BASE_URL: &str = "https://api.portone.io";
const CHARSET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-";

#[cfg(feature = "server")]
#[derive(Debug, Clone)]
pub struct PortOne {
    cli: reqwest::Client,
}

#[cfg(feature = "server")]
impl PortOne {
    fn map_reqwest(err: reqwest::Error) -> Error {
        Error::BadRequest(err.to_string())
    }

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
            .await
            .map_err(Self::map_reqwest)?;
        let j: serde_json::Value = res.json().await.map_err(Self::map_reqwest)?;
        let parsed: IdentifyResponse =
            serde_json::from_value(j).map_err(|e| Error::BadRequest(e.to_string()))?;
        Ok(parsed)
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
            channel_key: portone_conf.channel_key().to_string(),
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
            .await
            .map_err(Self::map_reqwest)?;

        if !res.status().is_success() {
            let err_text = res.text().await.unwrap_or_default();
            return Err(Error::BadRequest(format!(
                "PortOne get billing key error: {}",
                err_text
            )));
        }

        Ok(res.json().await.map_err(Self::map_reqwest)?)
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
        let conf = config::get();
        let portone_conf = conf.portone;
        let payment_id = format!(
            "{}-{}",
            random_string::generate(10, CHARSET),
            common::utils::time::get_now_timestamp_micros()
        );

        let body = BillingKeyPaymentRequest {
            store_id: portone_conf.store_id.to_string(),
            channel_key: portone_conf.channel_key().to_string(),
            billing_key,
            order_name,
            amount: PaymentAmountInput {
                total: amount,
                tax_free: None,
                vat: None,
            },
            customer: CustomerRequest {
                id: customer_id,
                name: CustomerName {
                    full: customer_name,
                },
            },
            currency: currency.to_string(),
            locale: None,
            notice_urls: portone_conf.notice_urls(conf.common.env),
        };

        let res = self
            .cli
            .post(format!("{}/payments/{}/billing-key", BASE_URL, payment_id))
            .json(&body)
            .send()
            .await
            .map_err(Self::map_reqwest)?;

        if !res.status().is_success() {
            let err_text = res.text().await.unwrap_or_default();
            return Err(Error::BadRequest(format!(
                "PortOne payment error: {}",
                err_text
            )));
        }

        Ok((res.json().await.map_err(Self::map_reqwest)?, payment_id))
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
    ) -> Result<(PaymentScheduleResponse, String)> {
        let conf = config::get();
        let portone_conf = conf.portone;
        let payment_id = format!(
            "{}-{}",
            random_string::generate(10, CHARSET),
            common::utils::time::get_now_timestamp_micros()
        );

        let payment = BillingKeyPaymentRequest {
            store_id: portone_conf.store_id.to_string(),
            channel_key: portone_conf.channel_key().to_string(),
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
            notice_urls: portone_conf.notice_urls(conf.common.env),
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
            .await
            .map_err(Self::map_reqwest)?;

        if !res.status().is_success() {
            let err_text = res.text().await.unwrap_or_default();
            return Err(Error::BadRequest(format!(
                "PortOne schedule payment error: {}",
                err_text
            )));
        }

        Ok((res.json().await.map_err(Self::map_reqwest)?, payment_id))
    }

    pub async fn cancel_schedule_with_billing_key(
        &self,
        billing_key: String,
    ) -> Result<PaymentCancelScheduleResponse> {
        let portone_conf = config::get().portone;
        let body = CancelPaymentScheduleRequest {
            store_id: portone_conf.store_id.to_string(),
            billing_key,
        };

        let res = self
            .cli
            .delete(format!("{}/payment-schedules", BASE_URL))
            .json(&body)
            .send()
            .await
            .map_err(Self::map_reqwest)?;

        if !res.status().is_success() {
            let err_text = res.text().await.unwrap_or_default();
            return Err(Error::BadRequest(format!(
                "PortOne cancel schedule error: {}",
                err_text
            )));
        }

        Ok(res.json().await.map_err(Self::map_reqwest)?)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "camelCase")]
pub struct IdentifyResponse {
    pub verified_customer: VerifiedCustomer,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "camelCase")]
pub struct VerifiedCustomer {
    pub birth_date: String,
    pub gender: String,
    pub id: String,
    pub is_foreigner: bool,
    pub name: String,
    pub phone_number: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BillingKeyRequest {
    pub store_id: String,
    pub channel_key: String,
    pub customer: CustomerRequest,
    pub method: MethodRequest,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BillingKeyResponse {
    pub billing_key_info: BillingKeyInfo,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BillingKeyInfo {
    pub billing_key: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BillingKeyPaymentRequest {
    pub store_id: String,
    pub channel_key: String,
    pub billing_key: String,
    pub order_name: String,
    pub customer: CustomerRequest,
    pub amount: PaymentAmountInput,
    pub currency: String,
    pub locale: Option<Locale>,
    pub notice_urls: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleBillingKeyRequest {
    pub payment: BillingKeyPaymentRequest,
    pub time_to_pay: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BillingKeyPaymentResponse {
    pub payment: PaymentResponse,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentResponse {
    pub paid_at: String,
    pub pg_tx_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentScheduleResponse {
    pub schedule: PaymentSchedule,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentSchedule {
    pub id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentAmountInput {
    pub total: i64,
    pub tax_free: Option<i64>,
    pub vat: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, DynamoEnum)]
#[serde(rename_all = "UPPERCASE")]
pub enum Locale {
    #[default]
    EnUs,
    KoKr,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerRequest {
    pub id: String,
    pub name: CustomerName,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerName {
    pub full: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MethodRequest {
    pub card: CardRequest,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardRequest {
    pub credential: CardCredentialRequest,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardCredentialRequest {
    pub number: String,
    pub expiry_year: String,
    pub expiry_month: String,
    pub birth_or_business_registration_number: String,
    pub password_two_digits: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelPaymentScheduleRequest {
    pub store_id: String,
    pub billing_key: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PaymentCancelScheduleResponse {
    pub revoked_schedule_ids: Vec<String>,
    pub revoked_at: Option<String>,
}
