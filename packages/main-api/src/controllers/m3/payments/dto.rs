use crate::services::portone::{PaymentCancellation, PaymentItem};
use crate::*;

pub use crate::services::portone::PortoneCancelRequester as CancelRequester;

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct AdminPaymentResponse {
    pub payment_id: String,
    pub status: String,
    pub currency: String,
    pub paid_at: Option<String>,
    pub order_name: String,
    pub user_pk: Option<String>,
    pub user_email: Option<String>,
    pub user_name: Option<String>,
    pub total: i64,
    pub cancelled: Option<i64>,
}

impl From<PaymentItem> for AdminPaymentResponse {
    fn from(item: PaymentItem) -> Self {
        Self {
            payment_id: item.id,
            status: item.status,
            currency: item.currency,
            paid_at: item.paid_at,
            order_name: item.order_name,
            total: item.amount.total,
            cancelled: item.amount.cancelled,
            user_pk: None,
            user_email: None,
            user_name: None,
        }
    }
}

impl AdminPaymentResponse {
    pub fn with_user(mut self, pk: String, email: String, name: String) -> Self {
        self.user_pk = Some(pk);
        self.user_email = Some(email);
        self.user_name = Some(name);
        self
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct AdminCancelPaymentRequest {
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct AdminCancelPaymentResponse {
    pub status: String,
    pub cancellation_id: String,
    pub total_amount: i64,
    pub reason: String,
    pub cancelled_at: Option<String>,
    pub requested_at: String,
}

impl From<PaymentCancellation> for AdminCancelPaymentResponse {
    fn from(cancellation: PaymentCancellation) -> Self {
        Self {
            status: cancellation.status,
            cancellation_id: cancellation.id,
            total_amount: cancellation.total_amount,
            reason: cancellation.reason,
            cancelled_at: cancellation.cancelled_at,
            requested_at: cancellation.requested_at,
        }
    }
}
