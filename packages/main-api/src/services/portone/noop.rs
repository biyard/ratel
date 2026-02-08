use super::*;
use crate::*;

#[derive(Debug, Clone)]
pub struct PortOne {}

impl PortOne {
    pub fn new(_api_secret: &str) -> Self {
        Self {}
    }

    pub async fn identify(&self, _id: &str) -> Result<IdentifyResponse> {
        // Return mock data for testing
        Ok(IdentifyResponse {
            channel: ChannelResponse {
                id: "channel-test".to_string(),
                name: "Test Channel".to_string(),
                key: "test-key".to_string(),
                pg_merchant_id: "test-merchant".to_string(),
                pg_provider: "test-provider".to_string(),
                r#type: "identity".to_string(),
            },
            id: "test-identity-id".to_string(),
            pg_raw_response: "{}".to_string(),
            pg_tx_id: "test-tx-id".to_string(),
            requested_at: "2024-01-01T00:00:00Z".to_string(),
            status: "verified".to_string(),
            status_changed_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            verified_at: "2024-01-01T00:00:00Z".to_string(),
            verified_customer: VerifiedCustomer {
                birth_date: "1990-01-15".to_string(),
                gender: VerifiedGender::Male,
                id: "test-customer-id".to_string(),
                is_foreigner: false,
                name: "Test User".to_string(),
                phone_number: "+821012345678".to_string(),
            },
            version: "1".to_string(),
        })
    }

    pub async fn get_billing_key(
        &self,
        _customer_id: String,
        _customer_name: String,
        _card_number: String,
        _expiry_year: String,
        _expiry_month: String,
        _birth_or_business_registration_number: String,
        _password_two_digits: String,
    ) -> Result<BillingKeyResponse> {
        Ok(BillingKeyResponse {
            billing_key_info: BillingKeyInfo {
                billing_key: "test-billing-key".to_string(),
            },
        })
    }

    pub async fn pay_with_billing_key(
        &self,
        customer_id: String,
        _customer_name: String,
        _order_name: String,
        _billing_key: String,
        _amount: i64,
        _currency: Currency,
    ) -> Result<(BillingKeyPaymentResponse, String)> {
        Ok((
            BillingKeyPaymentResponse {
                payment: Payment {
                    paid_at: "2025-11-03T11:01:50.08942321Z".to_string(),
                    pg_tx_id: "merchantest".to_string(),
                },
            },
            customer_id,
        ))
    }

    pub async fn schedule_pay_with_billing_key(
        &self,
        customer_id: String,
        _customer_name: String,
        _order_name: String,
        _billing_key: String,
        _amount: i64,
        _currency: Currency,
        _time_to_pay: String,
    ) -> Result<(PaymentScheduleResponse, String)> {
        Ok((
            PaymentScheduleResponse {
                schedule: PaymentSchedule {
                    id: "merchantest".to_string(),
                },
            },
            customer_id,
        ))
    }

    pub async fn cancel_schedule_with_billing_key(
        &self,
        billing_key: String,
    ) -> Result<PaymentCancelScheduleResponse> {
        Ok(PaymentCancelScheduleResponse {
            revoked_schedule_ids: vec![billing_key],
            revoked_at: Some("2025-11-03T11:01:50.08942321Z".to_string()),
        })
    }

    pub async fn list_payments(&self, page: i32, page_size: i32) -> Result<PaymentListResponse> {
        // Return mock payment list for testing
        Ok(PaymentListResponse {
            items: vec![PaymentItem {
                id: "test-payment-1".to_string(),
                status: "PAID".to_string(),
                currency: "KRW".to_string(),
                paid_at: Some("2025-02-03T10:00:00Z".to_string()),
                order_name: "Test Order 1".to_string(),
                customer: PaymentCustomer {
                    id: "USER#test-user-1##PAYMENT".to_string(),
                    name: Some("Test User 1".to_string()),
                },
                amount: PaymentAmount {
                    total: 10000,
                    tax_free: None,
                    vat: None,
                    supply: None,
                    discount: None,
                    paid: 10000,
                    cancelled: None,
                    cancelled_tax_free: None,
                },
                billing_key: None,
            }],
            page: PageInfo {
                number: page,
                size: page_size,
                total_count: 1,
            },
        })
    }
}
